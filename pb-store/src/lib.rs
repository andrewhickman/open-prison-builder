#[cfg_attr(not(target_arch = "wasm32"), path = "native.rs")]
#[cfg_attr(target_arch = "wasm32", path = "web.rs")]
mod sys;

use std::{marker::PhantomData, sync::Arc};

use anyhow::{Context, Result};
use bevy::prelude::*;
use chrono::{DateTime, Local, Utc};
use serde::{
    Deserialize, Serialize,
    de::{DeserializeOwned, DeserializeSeed},
};
use smol_str::SmolStr;

pub struct PbStorePlugin;

#[derive(Clone, Resource)]
pub struct Store(Arc<sys::Store>);

#[derive(Serialize, Deserialize)]
pub struct Metadata {
    pub name: SmolStr,
    pub modified: DateTime<Utc>,
}

impl Plugin for PbStorePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Store>();
    }
}

impl Store {
    pub fn new() -> Self {
        Store(Arc::new(
            sys::Store::new().expect("failed to initialize storage"),
        ))
    }

    pub async fn try_get<T>(&self, key: &str) -> Result<Option<T>>
    where
        T: TypePath + DeserializeOwned + Send,
    {
        self.0.get(key, PhantomData).await
    }

    pub async fn get_with<S, T>(&self, key: &str, seed: S) -> Result<T>
    where
        S: for<'de> DeserializeSeed<'de, Value = T>,
        T: TypePath + Send + 'static,
    {
        self.0
            .get(key, seed)
            .await?
            .with_context(|| format!("file '{key}' not found"))
    }

    pub async fn try_get_with<S, T>(&self, key: &str, seed: S) -> Result<Option<T>>
    where
        S: for<'de> DeserializeSeed<'de, Value = T>,
        T: TypePath + Send + 'static,
    {
        self.0.get(key, seed).await
    }

    pub async fn set<T>(&self, key: &str, value: T) -> Result<()>
    where
        T: Serialize + TypePath + Send,
    {
        self.0.set(key, value).await
    }

    pub async fn iter(&self, key: &str) -> Result<Vec<Metadata>> {
        self.0.iter(key).await
    }
}

impl Default for Store {
    fn default() -> Self {
        Self::new()
    }
}

impl Metadata {
    pub fn new(name: impl Into<SmolStr>) -> Self {
        Metadata {
            name: name.into(),
            modified: Utc::now(),
        }
    }

    pub fn modified_local(&self) -> DateTime<Local> {
        self.modified.into()
    }
}

fn from_json<S, T>(seed: S, json: &str) -> Result<T, serde_json::Error>
where
    S: for<'de> DeserializeSeed<'de, Value = T>,
    T: 'static,
{
    let mut de = serde_json::Deserializer::from_str(json);
    let value = seed.deserialize(&mut de)?;
    de.end()?;
    Ok(value)
}
