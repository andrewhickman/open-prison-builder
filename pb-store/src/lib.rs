#[cfg(not(target_arch = "wasm32"))]
mod native;
#[cfg(target_arch = "wasm32")]
mod web;

use std::{sync::Arc, sync::OnceLock, time::Duration};

use anyhow::Result;
use async_trait::async_trait;
use bevy::{
    ecs::system::SystemId,
    prelude::*,
    reflect::TypeRegistryArc,
    scene::serde::{SceneDeserializer, SceneSerializer},
    tasks::IoTaskPool,
};
use chrono::{DateTime, Utc};
use serde::{de::DeserializeSeed, Deserialize, Serialize};

use pb_engine::pawn::Pawn;
use pb_util::AsDynError;

pub const AUTOSAVE_INTERVAL: Duration = Duration::from_secs(5 * 60);

pub struct StorePlugin;

#[async_trait]
pub trait Store {
    async fn save(&self, metadata: SaveMetadata, scene: DynamicScene) -> Result<()>;

    async fn load(&self, name: String) -> Result<(SaveMetadata, DynamicScene)>;
}

#[derive(Resource)]
pub struct DynStore(Arc<dyn Store + Send + Sync>);

#[derive(Resource)]
pub enum SaveState {
    Waiting(Timer),
    Running(Arc<OnceLock<Result<()>>>),
}

#[derive(Resource)]
pub struct SaveSystem(SystemId<SaveInput>);

pub struct SaveInput {
    name: String,
    result: Arc<OnceLock<Result<()>>>,
}

#[derive(Serialize, Deserialize)]
pub struct SaveMetadata {
    pub name: String,
    pub modified: DateTime<Utc>,
}

impl Plugin for StorePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostUpdate, trigger_save);
        let save_system_id = app.world.register_system(save);
        app.insert_resource(SaveSystem(save_system_id));

        app.init_resource::<SaveState>();

        #[cfg(not(target_arch = "wasm32"))]
        app.add_systems(Startup, native::init);
        #[cfg(target_arch = "wasm32")]
        app.add_systems(Startup, web::init);
    }
}

impl SaveInput {
    pub fn new(name: impl Into<String>) -> Self {
        SaveInput {
            name: name.into(),
            result: default(),
        }
    }
}

impl Default for SaveState {
    fn default() -> Self {
        SaveState::Waiting(Timer::new(AUTOSAVE_INTERVAL, TimerMode::Once))
    }
}

pub fn trigger_save(
    mut commands: Commands,
    mut state: ResMut<SaveState>,
    save_system: Res<SaveSystem>,
    time: ResMut<Time<Real>>,
) {
    match state.as_mut() {
        SaveState::Waiting(timer) => {
            timer.tick(time.delta());
            if timer.just_finished() {
                let input = SaveInput::new("autosave");
                let result = input.result.clone();
                commands.run_system_with_input(save_system.0, input);
                *state = SaveState::Running(result)
            }
        }
        SaveState::Running(result) => match result.get() {
            Some(Ok(())) => *state = SaveState::default(),
            Some(Err(error)) => {
                error!(error = error.as_dyn_error(), "Failed to run autosave");
                *state = SaveState::default()
            }
            None => (),
        },
    }
}

pub fn save(
    In(input): In<SaveInput>,
    world: &World,
    store: Res<DynStore>,
    pawn_q: Query<Entity, With<Pawn>>,
) {
    let scene = DynamicSceneBuilder::from_world(world)
        .allow::<Pawn>()
        .allow::<Transform>()
        .extract_entities(pawn_q.iter())
        .build();

    let store = store.0.clone();
    IoTaskPool::get()
        .spawn(async move {
            let metadata = SaveMetadata {
                name: input.name,
                modified: Utc::now(),
            };
            let res = store.save(metadata, scene).await;
            input.result.set(res).expect("result already set");
        })
        .detach();
}

fn serialize(
    metadata: SaveMetadata,
    scene: DynamicScene,
    registry: &TypeRegistryArc,
) -> Result<String, serde_json::Error> {
    let mut buf = Vec::new();

    serde_json::to_writer(&mut buf, &metadata)?;
    buf.push(b'\n');
    serde_json::to_writer(&mut buf, &SceneSerializer::new(&scene, registry))?;

    Ok(String::from_utf8(buf).expect("JSON should be valid UTF-8"))
}

fn deserialize(
    json: &[u8],
    registry: &TypeRegistryArc,
) -> Result<(SaveMetadata, DynamicScene), serde_json::Error> {
    let mut deserializer = serde_json::Deserializer::from_slice(json);
    let scene_deserializer = SceneDeserializer {
        type_registry: &registry.read(),
    };

    let metadata = SaveMetadata::deserialize(&mut deserializer)?;
    let scene = scene_deserializer.deserialize(&mut deserializer)?;
    deserializer.end()?;

    Ok((metadata, scene))
}
