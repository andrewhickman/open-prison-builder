use bevy::{prelude::*, utils::HashMap};
use pb_util::{callback::CallbackSender, spawn_io, AsDynError};
use serde::{Deserialize, Serialize};

use crate::store::{DynStore, Store};

pub fn init(store: Res<DynStore>, callback: Res<CallbackSender>) {
    let store = store.clone();
    let callback = callback.clone();
    spawn_io(async move {
        let res = store.load_settings().await;
        let settings = match res {
            Ok(settings) => settings,
            Err(error) => {
                error!(error = error.as_dyn_error(), "Failed to load settings");
                Settings::default()
            }
        };

        callback.send(|world: &mut World| world.insert_resource(settings));
    });
}

#[derive(Serialize, Default, Deserialize, Resource)]
pub struct Settings {
    binds: HashMap<KeyCode, Action>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Action {
    ToggleMenu,
    CameraLeft,
    CameraUp,
    CameraRight,
    CameraDown,
    ZoomIn,
    ZoomOut,
}
