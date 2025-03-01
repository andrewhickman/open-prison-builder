use bevy::{prelude::*, utils::HashMap};
use pb_store::Store;
use pb_util::{callback::CallbackSender, spawn_io, AsDynError};
use serde::{Deserialize, Serialize};

use crate::input::Action;

pub const KEY: &str = "settings";

pub fn init(store: Res<Store>, callback: Res<CallbackSender>) {
    let store = store.clone();
    let callback = callback.clone();
    spawn_io(async move {
        let res = store.try_get::<SettingsModel>(KEY).await;
        let settings = match res {
            Ok(Some(settings)) => Settings::from(settings),
            Ok(None) => {
                info!("No settings file found, using default settings");
                Settings::default()
            }
            Err(error) => {
                error!(error = error.as_dyn_error(), "Failed to load settings");
                Settings::default()
            }
        };

        callback.send(|world: &mut World| world.insert_resource(settings));
    });
}

#[derive(Resource)]
pub struct Settings {
    pub binds: HashMap<KeyCode, Vec<Binding>>,
}

pub struct Binding {
    pub action: Action,
    pub modifiers: Vec<KeyCode>,
}

#[derive(Serialize, Deserialize, TypePath)]
pub struct SettingsModel {
    binds: HashMap<Action, BindingModel>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct BindingModel {
    key: KeyCode,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    modifiers: Vec<KeyCode>,
}

impl Settings {
    pub fn empty() -> Self {
        Settings {
            binds: HashMap::default(),
        }
    }

    pub fn bind(&mut self, key: KeyCode, action: Action, modifiers: Vec<KeyCode>) {
        self.binds
            .entry(key)
            .or_default()
            .push(Binding { action, modifiers })
    }

    pub fn get_bind(&self, key: KeyCode) -> &[Binding] {
        match self.binds.get(&key) {
            Some(binds) => binds.as_slice(),
            None => &[],
        }
    }
}

impl Default for Settings {
    fn default() -> Self {
        let mut settings = Self::empty();
        settings.bind(KeyCode::Escape, Action::Cancel, vec![]);
        settings.bind(KeyCode::KeyW, Action::PanUp, vec![]);
        settings.bind(KeyCode::KeyA, Action::PanLeft, vec![]);
        settings.bind(KeyCode::KeyS, Action::PanDown, vec![]);
        settings.bind(KeyCode::KeyD, Action::PanRight, vec![]);
        settings.bind(KeyCode::KeyQ, Action::ZoomIn, vec![]);
        settings.bind(KeyCode::KeyE, Action::ZoomOut, vec![]);
        settings.bind(KeyCode::Equal, Action::DecreaseGridSize, vec![]);
        settings.bind(KeyCode::Minus, Action::IncreaseGridSize, vec![]);
        settings
    }
}

impl From<SettingsModel> for Settings {
    fn from(model: SettingsModel) -> Self {
        let mut settings = Self::empty();
        for (action, bind) in model.binds {
            settings.bind(bind.key, action, bind.modifiers);
        }
        settings
    }
}
