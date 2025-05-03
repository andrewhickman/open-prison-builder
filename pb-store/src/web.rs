use anyhow::{Context, Result, anyhow};
use bevy::{log::info, reflect::TypePath};
use serde::{Serialize, de::DeserializeSeed};
use wasm_bindgen::{JsCast, JsValue};

use crate::{Metadata, from_json};

const META_SUFFIX: &str = ".meta";

pub(crate) struct Store;

impl Store {
    pub fn new() -> Result<Self> {
        Ok(Store)
    }
}

impl Store {
    pub async fn get<S, T>(&self, key: &str, seed: S) -> Result<Option<T>>
    where
        S: for<'de> DeserializeSeed<'de, Value = T>,
        T: TypePath + 'static,
    {
        let storage = self.storage()?;

        let json = storage
            .get_item(key)
            .map_err(map_err)
            .with_context(|| format!("failed to read from '{key}'"))?;
        if let Some(json) = json {
            let settings = from_json(seed, &json)
                .with_context(|| format!("failed to parse JSON at '{}'", key))?;
            info!(
                "Loaded value of type '{}' from '{}'",
                T::short_type_path(),
                key,
            );
            Ok(Some(settings))
        } else {
            Ok(None)
        }
    }

    pub async fn set<T>(&self, key: &str, value: T) -> Result<()>
    where
        T: Serialize + TypePath,
    {
        let storage = self.storage()?;

        let json = serde_json::to_string(&value).context("failed to serialize JSON")?;
        storage
            .set_item(key, &json)
            .map_err(map_err)
            .with_context(|| format!("failed to write to '{key}'"))?;

        let metadata = Metadata::new(file_stem(key));
        let metadata_json = serde_json::to_string(&metadata).context("failed to serialize JSON")?;
        let metadata_key = format!("{}{}", key, META_SUFFIX);
        storage
            .set_item(&metadata_key, &metadata_json)
            .map_err(map_err)
            .with_context(|| format!("failed to write to '{metadata_key}'"))?;

        info!(
            "Stored value of type '{}' at '{}'",
            T::short_type_path(),
            key,
        );
        Ok(())
    }

    pub async fn iter(&self, key: &str) -> Result<Vec<Metadata>> {
        let storage = self.storage()?;
        let prefix = if key.is_empty() {
            String::new()
        } else {
            format!("{key}/")
        };
        let length = storage.length().map_err(map_err)?;

        let mut results = Vec::new();
        for i in 0..length {
            let Some(key) = storage.key(i).map_err(map_err)? else {
                continue;
            };
            if !key.starts_with(&prefix) || !key.ends_with(META_SUFFIX) {
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
}

impl Store {
    fn storage(&self) -> Result<web_sys::Storage> {
        web_sys::window()
            .context("failed to get window")?
            .local_storage()
            .map_err(map_err)
            .context("failed to get local storage")?
            .context("failed to get local storage")
    }
}

fn file_stem(path: &str) -> &str {
    let name = path.rsplit_once('/').map(|(_, s)| s).unwrap_or(path);
    name.rsplit_once('.').map(|(s, _)| s).unwrap_or(name)
}

fn map_err(err: JsValue) -> anyhow::Error {
    match err.dyn_into::<js_sys::Error>() {
        Ok(error) => anyhow!("{}", error.message()),
        Err(value) => anyhow!("{value:?}"),
    }
}
