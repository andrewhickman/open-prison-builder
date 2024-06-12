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

#[derive(Serialize, Deserialize, Resource)]
pub struct Settings {
    pub binds: HashMap<KeyCode, Action>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Action {
    Cancel,
    PanLeft,
    PanUp,
    PanRight,
    PanDown,
    ZoomIn,
    ZoomOut,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            binds: HashMap::from_iter([
                (KeyCode::Escape, Action::Cancel),
                (KeyCode::KeyW, Action::PanUp),
                (KeyCode::KeyA, Action::PanLeft),
                (KeyCode::KeyS, Action::PanDown),
                (KeyCode::KeyD, Action::PanRight),
                (KeyCode::KeyQ, Action::ZoomIn),
                (KeyCode::KeyE, Action::ZoomOut),
            ]),
        }
    }
}
