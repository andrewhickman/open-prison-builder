use std::{
    ffi::OsStr,
    io,
    path::{Path, PathBuf},
    sync::Arc,
};

use anyhow::{Context, Error, Result};
use async_trait::async_trait;
use bevy::{prelude::*, reflect::TypeRegistryArc, tasks::futures_lite::StreamExt};
use smol_str::SmolStr;

use crate::{
    save::SaveMetadata,
    settings::Settings,
    store::{deserialize, serialize, DynStore, Store},
};

pub fn init(mut commands: Commands, registry: Res<AppTypeRegistry>) {
    commands.insert_resource(DynStore(Arc::new(
        FsStore::new(registry.0.clone()).expect("failed to initialize storage"),
    )));
}

struct FsStore {
    saves: PathBuf,
    settings: PathBuf,
    registry: TypeRegistryArc,
}

impl FsStore {
    pub fn new(registry: TypeRegistryArc) -> Result<Self> {
        let dirs =
            directories::ProjectDirs::from("pb.dev.andrewhickman", "", "open-prison-builder")
                .context("failed to find data directory")?;

        let saves = dirs.data_dir().join("saves");
        let settings = dirs.data_dir().join("settings.json");

        Ok(FsStore {
            saves,
            settings,
            registry,
        })
    }
}

#[async_trait]
impl Store for FsStore {
    async fn list_saves(&self) -> Result<Vec<SaveMetadata>> {
        let mut files = match async_fs::read_dir(&self.saves).await {
            Ok(files) => files.map(|res| {
                res.with_context(|| format!("failed to read directory '{}'", self.saves.display()))
            }),
            Err(error) if error.kind() == io::ErrorKind::NotFound => return Ok(vec![]),
            Err(error) => {
                return Err(Error::from(error).context(format!(
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

    async fn store_save(&self, metadata: SaveMetadata, scene: DynamicScene) -> Result<()> {
        let path = self.saves.join(format!("{}.json", metadata.name));
        let json = serialize(scene, &self.registry).context("failed to serialize JSON")?;

        write_create_dir(&path, &json).await?;
        info!("Stored save at '{}'", path.display());
        Ok(())
    }

    async fn load_save(&self, name: SmolStr) -> Result<DynamicScene> {
        let path = self.saves.join(format!("{name}.json"));
        let json = async_fs::read(&path)
            .await
            .with_context(|| format!("failed to read from '{}'", path.display()))?;

        let save = deserialize(&json, &self.registry)
            .with_context(|| format!("failed to parse JSON at '{}'", path.display()))?;
        info!("Loaded save from '{}'", path.display());
        Ok(save)
    }

    async fn store_settings(&self, settings: Settings) -> Result<()> {
        let json = serde_json::to_string_pretty(&settings).context("failed to serialize JSON")?;
        write_create_dir(&self.settings, &json).await?;
        info!("Stored settings to '{}'", self.settings.display());
        Ok(())
    }

    async fn load_settings(&self) -> Result<Settings> {
        let json = match async_fs::read(&self.settings).await {
            Ok(json) => json,
            Err(error) if error.kind() == io::ErrorKind::NotFound => {
                info!("Settings file not found at '{}'", self.settings.display());
                return Ok(Settings::default());
            }
            Err(error) => {
                return Err(Error::from(error)
                    .context(format!("failed to read from '{}'", self.settings.display())))
            }
        };

        let save = serde_json::from_slice::<Settings>(&json)
            .with_context(|| format!("failed to parse JSON at '{}'", self.settings.display()))?;
        info!("Loaded settings from '{}'", self.settings.display());
        Ok(save)
    }
}

async fn write_create_dir(path: &Path, content: &str) -> Result<()> {
    match async_fs::write(&path, content).await {
        Ok(()) => Ok(()),
        Err(error) if error.kind() == io::ErrorKind::NotFound => {
            let Some(dir) = path.parent() else {
                return Err(anyhow::Error::from(error)
                    .context(format!("failed to write to '{}'", path.display())));
            };

            async_fs::create_dir_all(&dir)
                .await
                .with_context(|| format!("failed to create directory '{}'", dir.display()))?;
            async_fs::write(&path, content)
                .await
                .with_context(|| format!("failed to write to '{}'", path.display()))
        }
        Err(error) => {
            Err(anyhow::Error::from(error)
                .context(format!("failed to write to '{}'", path.display())))
        }
    }
}
