use std::sync::Arc;

use anyhow::{anyhow, bail, Context, Result};
use async_trait::async_trait;
use bevy::{prelude::*, reflect::TypeRegistryArc};
use smol_str::SmolStr;
use wasm_bindgen::{JsCast, JsValue};

use crate::{
    save::SaveMetadata,
    store::{deserialize, serialize, DynStore, Store},
};

pub fn init(mut commands: Commands, registry: Res<AppTypeRegistry>) {
    commands.insert_resource(DynStore(Arc::new(
        WebStore::new(registry.0.clone()).expect("failed to initialize storage"),
    )));
}

struct WebStore {
    registry: TypeRegistryArc,
}

impl WebStore {
    pub fn new(registry: TypeRegistryArc) -> Result<Self> {
        Ok(WebStore { registry })
    }
}

#[async_trait(?Send)]
impl Store for WebStore {
    async fn list(&self) -> Result<Vec<SaveMetadata>> {
        let storage = self.storage()?;
        let length = storage.length().map_err(map_err)?;

        let mut results = Vec::new();
        for i in 0..length {
            let Some(key) = storage.key(i).map_err(map_err)? else {
                continue;
            };
            if !key.starts_with("saves/") || !key.ends_with(".meta") {
                continue;
            }

            let Some(json) = storage
                .get_item(&key)
                .map_err(map_err)
                .with_context(|| format!("failed to read from '{key}'"))?
            else {
                continue;
            };
            let metadata = serde_json::from_str(&json)
                .with_context(|| format!("failed to parse JSON at '{}'", key))?;
            results.push(metadata)
        }

        Ok(results)
    }

    async fn save(&self, metadata: SaveMetadata, scene: DynamicScene) -> Result<()> {
        let storage = self.storage()?;
        let key = format!("saves/{}.json", metadata.name);
        let meta_key = format!("saves/{}.meta", metadata.name);

        let json = serialize(scene, &self.registry).context("failed to serialize JSON")?;
        let metadata_json = serde_json::to_string(&metadata).context("failed to serialize JSON")?;
        storage
            .set_item(&key, &json)
            .map_err(map_err)
            .with_context(|| format!("failed to write to '{key}'"))?;
        storage
            .set_item(&meta_key, &metadata_json)
            .map_err(map_err)
            .with_context(|| format!("failed to write to '{meta_key}'"))?;
        info!("Saved to '{key}'");
        Ok(())
    }

    async fn load(&self, name: SmolStr) -> Result<DynamicScene> {
        let storage = self.storage()?;
        let key = format!("saves/{name}.json");

        let json = storage
            .get_item(&key)
            .map_err(map_err)
            .with_context(|| format!("failed to read from '{key}'"))?;

        if let Some(json) = json {
            let save = deserialize(json.as_bytes(), &self.registry)
                .with_context(|| format!("failed to parse JSON at '{}'", key))?;
            info!("Loaded from '{key}'");
            Ok(save)
        } else {
            bail!("entry not found at '{key}'")
        }
    }
}

impl WebStore {
    fn storage(&self) -> Result<web_sys::Storage> {
        web_sys::window()
            .context("failed to get window")?
            .local_storage()
            .map_err(map_err)
            .context("failed to get local storage")?
            .context("failed to get local storage")
    }
}

fn map_err(err: JsValue) -> anyhow::Error {
    match err.dyn_into::<js_sys::Error>() {
        Ok(error) => anyhow!("{}", error.message()),
        Err(value) => anyhow!("{value:?}"),
    }
}
