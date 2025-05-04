use bevy::{
    ecs::error::{BevyError, Result},
    log::info,
    reflect::TypePath,
};
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

        let json = storage.get_item(key).map_err(map_err)?;
        if let Some(json) = json {
            let settings = from_json(seed, &json)
                .map_err(|err| format!("failed to parse JSON at '{key}': {err}"))?;
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

    pub async fn set<T>(&self, key: &str, value: T) -> Result
    where
        T: Serialize + TypePath,
    {
        let storage = self.storage()?;

        let json = serde_json::to_string(&value)
            .map_err(|err| BevyError::from(format!("failed to serialize JSON: {err}")))?;
        storage.set_item(key, &json).map_err(map_err)?;

        let metadata = Metadata::new(file_stem(key));
        let metadata_json = serde_json::to_string(&metadata)
            .map_err(|err| BevyError::from(format!("failed to serialize JSON: {err}")))?;
        let metadata_key = format!("{}{}", key, META_SUFFIX);
        storage
            .set_item(&metadata_key, &metadata_json)
            .map_err(map_err)?;

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

            let Some(json) = storage.get_item(&key).map_err(map_err)? else {
                continue;
            };
            let metadata = serde_json::from_str(&json)
                .map_err(|error| format!("failed to parse JSON at '{}': {error}", key))?;
            results.push(metadata)
        }

        Ok(results)
    }
}

impl Store {
    fn storage(&self) -> Result<web_sys::Storage> {
        Ok(web_sys::window()
            .ok_or("failed to get window")?
            .local_storage()
            .map_err(map_err)?
            .ok_or("failed to get local storage")?)
    }
}

fn file_stem(path: &str) -> &str {
    let name = path.rsplit_once('/').map(|(_, s)| s).unwrap_or(path);
    name.rsplit_once('.').map(|(s, _)| s).unwrap_or(name)
}

fn map_err(err: JsValue) -> BevyError {
    match err.dyn_into::<js_sys::Error>() {
        Ok(error) => format!("{}", error.message()).into(),
        Err(value) => format!("{value:?}").into(),
    }
}
