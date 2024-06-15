use std::{ffi::OsStr, io, path::PathBuf};

use anyhow::{Context, Error, Result};
use bevy::{log::info, reflect::TypePath, tasks::futures_lite::StreamExt};
use serde::{de::DeserializeSeed, Serialize};

use crate::{from_json, Metadata};

pub(crate) struct Store {
    path: PathBuf,
}

impl Store {
    pub fn new() -> Result<Self> {
        let dirs =
            directories::ProjectDirs::from("pb.dev.andrewhickman", "", "open-prison-builder")
                .context("failed to find data directory")?;

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
                return Err(
                    Error::from(error).context(format!("failed to read from '{}'", path.display()))
                )
            }
        };

        let value = from_json(seed, &json)
            .with_context(|| format!("failed to parse JSON at '{}'", path.display()))?;
        info!(
            "Loaded value of type '{}' save from '{}'",
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
        let json = serde_json::to_string(&value).context("failed to serialize JSON")?;

        match async_fs::write(&path, &json).await {
            Ok(()) => (),
            Err(error) if error.kind() == io::ErrorKind::NotFound => {
                let Some(dir) = path.parent() else {
                    return Err(Error::from(error)
                        .context(format!("failed to write to '{}'", path.display())));
                };

                async_fs::create_dir_all(&dir)
                    .await
                    .with_context(|| format!("failed to create directory '{}'", dir.display()))?;
                async_fs::write(&path, &json)
                    .await
                    .with_context(|| format!("failed to write to '{}'", path.display()))?;
            }
            Err(error) => {
                return Err(
                    Error::from(error).context(format!("failed to write to '{}'", path.display()))
                )
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
                res.with_context(|| format!("failed to read directory '{}'", path.display()))
            }),
            Err(error) if error.kind() == io::ErrorKind::NotFound => return Ok(vec![]),
            Err(error) => {
                return Err(Error::from(error)
                    .context(format!("failed to read directory '{}'", path.display())))
            }
        };

        let mut results = Vec::new();
        while let Some(entry) = files.try_next().await? {
            let entry_path = entry.path();
            let Some(name) = entry_path.file_stem().and_then(OsStr::to_str) else {
                continue;
            };

            let metadata = entry.metadata().await.with_context(|| {
                format!("failed to get metadata for '{}'", entry_path.display())
            })?;

            results.push(Metadata {
                name: name.into(),
                modified: metadata.modified()?.into(),
            });
        }

        Ok(results)
    }
}
