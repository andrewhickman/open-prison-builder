use bevy::{platform::collections::HashMap, prelude::*};
use pb_store::Store;
use serde::{Deserialize, Serialize};

use crate::input::Input;

pub const KEY: &str = "settings";

#[derive(Resource)]
pub struct Settings {
    pub binds: HashMap<KeyCode, Vec<Binding>>,
}

pub struct Binding {
    pub action: Input,
    pub modifiers: Vec<KeyCode>,
}

#[derive(Serialize, Deserialize, TypePath)]
pub struct SettingsModel {
    binds: HashMap<Input, BindingModel>,
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

    pub async fn load(store: &Store) -> Self {
        let res = store.try_get::<SettingsModel>(KEY).await;
        match res {
            Ok(Some(settings)) => Settings::from(settings),
            Ok(None) => {
                info!("No settings file found, using default settings");
                Settings::default()
            }
            Err(error) => {
                error!("Failed to load settings: {error}");
                Settings::default()
            }
        }
    }

    pub fn bind(&mut self, key: KeyCode, action: Input, modifiers: Vec<KeyCode>) {
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
        settings.bind(KeyCode::Escape, Input::Cancel, vec![]);
        settings.bind(KeyCode::KeyW, Input::PanUp, vec![]);
        settings.bind(KeyCode::KeyA, Input::PanLeft, vec![]);
        settings.bind(KeyCode::KeyS, Input::PanDown, vec![]);
        settings.bind(KeyCode::KeyD, Input::PanRight, vec![]);
        settings.bind(KeyCode::KeyQ, Input::ZoomIn, vec![]);
        settings.bind(KeyCode::KeyE, Input::ZoomOut, vec![]);
        settings.bind(KeyCode::Equal, Input::DecreaseGridSize, vec![]);
        settings.bind(KeyCode::Minus, Input::IncreaseGridSize, vec![]);
        settings.bind(KeyCode::ArrowLeft, Input::MoveLeft, vec![]);
        settings.bind(KeyCode::ArrowUp, Input::MoveForward, vec![]);
        settings.bind(KeyCode::ArrowRight, Input::MoveRight, vec![]);
        settings.bind(KeyCode::ArrowDown, Input::MoveBackward, vec![]);
        settings.bind(KeyCode::KeyP, Input::TogglePause, vec![]);
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
