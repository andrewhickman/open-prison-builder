use std::sync::Arc;

use anyhow::{anyhow, bail, Context, Result};
use async_trait::async_trait;
use bevy::{prelude::*, reflect::TypeRegistryArc};
use wasm_bindgen::{JsCast, JsValue};

use crate::{deserialize, serialize, DynStore, SaveMetadata, Store};

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

#[async_trait]
impl Store for WebStore {
    async fn save(&self, metadata: SaveMetadata, scene: DynamicScene) -> Result<()> {
        let storage = self.storage()?;
        let key = format!("saves/{}", metadata.name);

        let json =
            serialize(metadata, scene, &self.registry).context("failed to serialize JSON")?;
        storage
            .set_item(&key, &json)
            .map_err(map_err)
            .with_context(|| format!("failed to write to '{key}'"))?;
        info!("Saved to '{key}'");
        Ok(())
    }

    async fn load(&self, name: String) -> Result<(SaveMetadata, DynamicScene)> {
        let storage = self.storage()?;
        let key = format!("saves/{name}");

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
