use anyhow::{Error, Result};
use bevy_asset::{io, prelude::*, AssetLoader, AsyncReadExt, BoxedFuture, LoadContext};
use bevy_reflect::Reflect;

use serde::Deserialize;

#[derive(Reflect, Deserialize)]
pub struct MaterialDefinition {
    pub id: String,
    pub solid: bool,
}

#[derive(Asset, Reflect, Deserialize)]
pub struct MaterialDefinitions {
    pub materials: Vec<MaterialDefinition>,
}

#[derive(Default)]
pub struct MaterialDefinitionLoader;

impl AssetLoader for MaterialDefinitionLoader {
    type Asset = MaterialDefinitions;
    type Settings = ();
    type Error = Error;

    fn load<'a>(
        &'a self,
        reader: &'a mut io::Reader<'_>,
        _: &'a (),
        _: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<Self::Asset, Self::Error>> {
        Box::pin(async move {
            let mut buf = String::new();
            reader.read_to_string(&mut buf).await?;
            let value = serde_yaml2::from_str(&buf)?;
            Ok(value)
        })
    }
}
