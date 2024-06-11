use std::sync::Arc;

use anyhow::Result;
use async_trait::async_trait;
use bevy::{
    ecs::system::Resource,
    reflect::TypeRegistryArc,
    scene::{
        serde::{SceneDeserializer, SceneSerializer},
        DynamicScene,
    },
    utils::BoxedFuture,
};
use serde::de::DeserializeSeed;
use smol_str::SmolStr;

#[cfg_attr(not(target_arch = "wasm32"), path = "native.rs")]
#[cfg_attr(target_arch = "wasm32", path = "web.rs")]
mod sys;

use crate::{save::SaveMetadata, settings::Settings};

pub use self::sys::init;

#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
pub trait Store {
    async fn list_saves(&self) -> Result<Vec<SaveMetadata>>;

    async fn store_save(&self, metadata: SaveMetadata, scene: DynamicScene) -> Result<()>;

    async fn load_save(&self, name: SmolStr) -> Result<DynamicScene>;

    async fn store_settings(&self, settings: Settings) -> Result<()>;

    async fn load_settings(&self) -> Result<Settings>;
}

#[derive(Clone, Resource)]
pub struct DynStore(Arc<dyn Store + Send + Sync>);

impl Store for DynStore {
    fn list_saves<'a: 'b, 'b>(&'a self) -> BoxedFuture<'b, Result<Vec<SaveMetadata>>> {
        self.0.list_saves()
    }

    fn store_save<'a: 'b, 'b>(
        &'a self,
        metadata: SaveMetadata,
        scene: DynamicScene,
    ) -> BoxedFuture<'b, Result<()>> {
        self.0.store_save(metadata, scene)
    }

    fn load_save<'a: 'b, 'b>(&'a self, name: SmolStr) -> BoxedFuture<'b, Result<DynamicScene>> {
        self.0.load_save(name)
    }

    fn store_settings<'a: 'b, 'b>(&'a self, settings: Settings) -> BoxedFuture<'b, Result<()>> {
        self.0.store_settings(settings)
    }

    fn load_settings<'a: 'b, 'b>(&'a self) -> BoxedFuture<'b, Result<Settings>> {
        self.0.load_settings()
    }
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
