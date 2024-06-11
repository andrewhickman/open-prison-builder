use std::{ffi::OsStr, io, path::PathBuf, sync::Arc};

use anyhow::{Context, Result};
use async_trait::async_trait;
use bevy::{prelude::*, reflect::TypeRegistryArc, tasks::futures_lite::StreamExt};
use smol_str::SmolStr;

use crate::{
    save::SaveMetadata,
    store::{deserialize, serialize, DynStore, Store},
};

pub fn init(mut commands: Commands, registry: Res<AppTypeRegistry>) {
    commands.insert_resource(DynStore(Arc::new(
        FsStore::new(registry.0.clone()).expect("failed to initialize storage"),
    )));
}

struct FsStore {
    saves: PathBuf,
    registry: TypeRegistryArc,
}

impl FsStore {
    pub fn new(registry: TypeRegistryArc) -> Result<Self> {
        let dirs =
            directories::ProjectDirs::from("pb.dev.andrewhickman", "", "open-prison-builder")
                .context("failed to find data directory")?;

        let saves = dirs.data_dir().join("saves");

        Ok(FsStore { saves, registry })
    }
}

#[async_trait]
impl Store for FsStore {
    async fn list(&self) -> Result<Vec<SaveMetadata>> {
        let mut files = match async_fs::read_dir(&self.saves).await {
            Ok(files) => files.map(|res| {
                res.with_context(|| format!("failed to read directory '{}'", self.saves.display()))
            }),
            Err(error) if error.kind() == io::ErrorKind::NotFound => return Ok(vec![]),
            Err(error) => {
                return Err(anyhow::Error::from(error).context(format!(
                    "failed to read directory '{}'",
                    self.saves.display()
                )))
            }
        };

        let mut results = Vec::new();
        while let Some(entry) = files.try_next().await? {
            let path = entry.path();
            let Some(name) = path.file_stem().and_then(OsStr::to_str) else {
                continue;
            };

            let metadata = entry
                .metadata()
                .await
                .with_context(|| format!("failed to get metadata for '{}'", path.display()))?;

            results.push(SaveMetadata {
                name: name.into(),
                modified: metadata.modified()?.into(),
            });
        }

        Ok(results)
    }

    async fn save(&self, metadata: SaveMetadata, scene: DynamicScene) -> Result<()> {
        let path = self.saves.join(format!("{}.json", metadata.name));
        let json = serialize(scene, &self.registry).context("failed to serialize JSON")?;

        match async_fs::write(&path, &json).await {
            Ok(_) => (),
            Err(error) if error.kind() == io::ErrorKind::NotFound => {
                async_fs::create_dir_all(&self.saves)
                    .await
                    .with_context(|| {
                        format!("failed to create directory '{}'", self.saves.display())
                    })?;
                async_fs::write(&path, &json)
                    .await
                    .with_context(|| format!("failed to write to '{}'", path.display()))?;
            }
            Err(error) => {
                return Err(anyhow::Error::from(error)
                    .context(format!("failed to write to '{}'", path.display())))
            }
        }

        info!("Saved to '{}'", path.display());
        Ok(())
    }

    async fn load(&self, name: SmolStr) -> Result<DynamicScene> {
        let path = self.saves.join(format!("{name}.json"));
        let json = async_fs::read(&path)
            .await
            .with_context(|| format!("failed to read from '{}'", path.display()))?;

        let save = deserialize(&json, &self.registry)
            .with_context(|| format!("failed to parse JSON at '{}'", path.display()))?;
        info!("Loaded from '{}'", path.display());
        Ok(save)
    }
}
