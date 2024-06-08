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
    time::common_conditions::on_real_timer,
    utils::BoxedFuture,
};
use chrono::{DateTime, Local, Utc};
use pb_util::{spawn_io, AsDynError};
use serde::{de::DeserializeSeed, Deserialize, Serialize};

use pb_engine::pawn::Pawn;

pub const AUTO_SAVE_INTERVAL: Duration = Duration::from_secs(5 * 60);

pub struct StorePlugin;

#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
pub trait Store {
    async fn list(&self) -> Result<Vec<SaveMetadata>>;

    async fn save(&self, metadata: SaveMetadata, scene: DynamicScene) -> Result<()>;

    async fn load(&self, name: String) -> Result<DynamicScene>;
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

impl Store for DynStore {
    fn list<'a: 'b, 'b>(&'a self) -> BoxedFuture<'b, Result<Vec<SaveMetadata>>> {
        self.0.list()
    }

    fn save<'a: 'b, 'b>(
        &'a self,
        metadata: SaveMetadata,
        scene: DynamicScene,
    ) -> BoxedFuture<'b, Result<()>> {
        self.0.save(metadata, scene)
    }

    fn load<'a: 'b, 'b>(&'a self, name: String) -> BoxedFuture<'b, Result<DynamicScene>> {
        self.0.load(name)
    }
}

impl SaveMetadata {
    pub fn modified_local(&self) -> DateTime<Local> {
        self.modified.into()
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

    spawn_io(async move {
        let metadata = SaveMetadata {
            name,
            modified: Utc::now(),
        };
        let res = store.save(metadata, scene).await;
        callback(res);
    });
}

fn serialize(scene: DynamicScene, registry: &TypeRegistryArc) -> Result<String, serde_json::Error> {
    serde_json::to_string(&SceneSerializer::new(&scene, registry))
}

fn deserialize(json: &[u8], registry: &TypeRegistryArc) -> Result<DynamicScene, serde_json::Error> {
    let mut deserializer = serde_json::Deserializer::from_slice(json);
    let scene_deserializer = SceneDeserializer {
        type_registry: &registry.read(),
    };

    let scene = scene_deserializer.deserialize(&mut deserializer)?;
    deserializer.end()?;

    Ok(scene)
}
