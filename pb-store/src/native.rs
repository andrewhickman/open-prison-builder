use std::{io, path::PathBuf, sync::Arc};

use anyhow::{Context, Result};
use async_trait::async_trait;
use bevy::{prelude::*, reflect::TypeRegistryArc};

use crate::{deserialize, serialize, DynStore, Store};

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
    async fn save(&self, name: String, scene: DynamicScene) -> Result<()> {
        let json = serialize(scene, &self.registry).context("failed to serialize JSON")?;

        let path = self.saves.join(format!("{name}.json"));
        match async_fs::write(&path, &json).await {
            Ok(_) => (),
            Err(error) if error.kind() == io::ErrorKind::NotFound => {
                async_fs::create_dir_all(&self.saves)
                    .await
                    .with_context(|| {
                        format!("failed to create directory '{}'", self.saves.display())
                    })?;
                async_fs::write(&path, &json).await.with_context(|| {
                    format!("failed to create directory '{}'", self.saves.display())
                })?;
            }
            Err(error) => {
                return Err(anyhow::Error::from(error)
                    .context(format!("failed to write to '{}'", path.display())))
            }
        }

        info!("Saved to '{}'", path.display());
        Ok(())
    }

    async fn load(&self, name: String) -> Result<DynamicScene> {
        let path = self.saves.join(format!("{name}.json"));
        let json = async_fs::read(&path)
            .await
            .with_context(|| format!("failed to read from '{}'", path.display()))?;

        let scene = deserialize(&json, &self.registry)
            .with_context(|| format!("failed to parse JSON at '{}'", path.display()))?;
        info!("Loaded from '{}'", path.display());
        Ok(scene)
    }
}
