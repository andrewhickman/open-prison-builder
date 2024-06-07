#[cfg(not(target_arch = "wasm32"))]
mod native;
#[cfg(target_arch = "wasm32")]
mod web;

use std::{sync::Arc, time::Duration};

use anyhow::Result;
use async_trait::async_trait;
use bevy::{
    prelude::*,
    reflect::TypeRegistryArc,
    scene::serde::{SceneDeserializer, SceneSerializer},
    tasks::IoTaskPool,
    time::common_conditions::on_real_timer,
};
use chrono::{DateTime, Utc};
use pb_util::AsDynError;
use serde::{de::DeserializeSeed, Deserialize, Serialize};

use pb_engine::pawn::Pawn;

pub const AUTO_SAVE_INTERVAL: Duration = Duration::from_secs(5 * 60);

pub struct StorePlugin;

#[async_trait]
pub trait Store {
    async fn save(&self, metadata: SaveMetadata, scene: DynamicScene) -> Result<()>;

    async fn load(&self, name: String) -> Result<(SaveMetadata, DynamicScene)>;
}

#[derive(Clone, Resource)]
pub struct DynStore(Arc<dyn Store + Send + Sync>);

#[derive(Serialize, Deserialize)]
pub struct SaveMetadata {
    pub name: String,
    pub modified: DateTime<Utc>,
}

impl Plugin for StorePlugin {
    fn build(&self, app: &mut App) {
        #[cfg(not(target_arch = "wasm32"))]
        app.add_systems(Startup, native::init);
        #[cfg(target_arch = "wasm32")]
        app.add_systems(Startup, web::init);

        app.add_systems(
            PostUpdate,
            auto_save.run_if(on_real_timer(AUTO_SAVE_INTERVAL)),
        );
    }
}

pub fn auto_save(world: &World, pawn_q: Query<Entity, With<Pawn>>, store: Res<DynStore>) {
    save(
        "autosave".to_owned(),
        world,
        &pawn_q,
        store.clone(),
        |res| {
            if let Err(error) = res {
                error!(error = error.as_dyn_error(), "Failed to auto-save");
            }
        },
    );
}

pub fn save(
    name: String,
    world: &World,
    entities: impl IntoIterator<Item = Entity>,
    store: DynStore,
    callback: impl FnOnce(Result<()>) + Send + 'static,
) {
    let scene = DynamicSceneBuilder::from_world(world)
        .allow::<Pawn>()
        .allow::<Transform>()
        .extract_entities(entities.into_iter())
        .build();

    IoTaskPool::get()
        .spawn(async move {
            let metadata = SaveMetadata {
                name,
                modified: Utc::now(),
            };
            let res = store.0.save(metadata, scene).await;
            callback(res);
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
