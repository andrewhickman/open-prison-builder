use std::{ffi::OsStr, io, path::PathBuf};

use bevy::{prelude::*, tasks::futures_lite::StreamExt};
use directories::ProjectDirs;
use serde::{Serialize, de::DeserializeSeed};

use crate::{Metadata, from_json};

pub(crate) struct Store {
    path: PathBuf,
}

impl Store {
    pub fn new() -> Result<Self> {
        let dirs = ProjectDirs::from("pb.dev.andrewhickman", "", "open-prison-builder")
            .ok_or("failed to find data directory")?;

        Ok(Store {
            path: dirs.data_dir().to_owned(),
        })
    }
}

impl Store {
    pub async fn get<S, T>(&self, key: &str, seed: S) -> Result<Option<T>>
    where
        S: for<'de> DeserializeSeed<'de, Value = T>,
        T: TypePath + 'static,
    {
        let path = self.path.join(key);
        let json = match async_fs::read_to_string(&path).await {
            Ok(json) => json,
            Err(error) if error.kind() == io::ErrorKind::NotFound => return Ok(None),
            Err(error) => {
                return Err(format!("failed to read from '{}': {error}", path.display()).into());
            }
        };

        let value = from_json(seed, &json)
            .map_err(|error| format!("failed to parse JSON at '{}': {error}", path.display()))?;
        info!(
            "Loaded value of type '{}' from '{}'",
            T::short_type_path(),
            path.display()
        );
        Ok(Some(value))
    }

    pub async fn set<T>(&self, key: &str, value: T) -> Result<()>
    where
        T: Serialize + TypePath,
    {
        let path = self.path.join(key);
        let json = serde_json::to_string(&value)?;

        match async_fs::write(&path, &json).await {
            Ok(()) => (),
            Err(error) if error.kind() == io::ErrorKind::NotFound => {
                let Some(dir) = path.parent() else {
                    return Err(format!("failed to write to '{}': {error}", path.display()).into());
                };

                async_fs::create_dir_all(&dir).await.map_err(|error| {
                    format!("failed to create directory '{}': {error}", dir.display())
                })?;
                async_fs::write(&path, &json)
                    .await
                    .map_err(|error| format!("failed to write to '{}': {error}", path.display()))?;
            }
            Err(error) => {
                return Err(format!("failed to write to '{}': {error}", path.display()).into());
            }
        };

        info!(
            "Stored value of type '{}' at '{}'",
            T::short_type_path(),
            path.display()
        );
        Ok(())
    }

    pub async fn iter(&self, key: &str) -> Result<Vec<Metadata>> {
        let path = self.path.join(key);

        let mut files = match async_fs::read_dir(&path).await {
            Ok(files) => files.map(|res| {
                res.map_err(|error| {
                    format!("failed to read directory '{}': {error}", path.display())
                })
            }),
            Err(error) if error.kind() == io::ErrorKind::NotFound => return Ok(vec![]),
            Err(error) => {
                return Err(
                    format!("failed to read directory '{}': {error}", path.display()).into(),
                );
            }
        };

        let mut results = Vec::new();
        while let Some(entry) = files.try_next().await? {
            let entry_path = entry.path();
            let Some(name) = entry_path.file_stem().and_then(OsStr::to_str) else {
                continue;
            };

            let metadata = entry.metadata().await.map_err(|error| {
                format!(
                    "failed to get metadata for '{}': {error}",
                    entry_path.display()
                )
            })?;

            results.push(Metadata {
                name: name.into(),
                modified: metadata.modified()?.into(),
            });
        }

        Ok(results)
    }
}
