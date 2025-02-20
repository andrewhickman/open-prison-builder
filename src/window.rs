use std::io;

use bevy::{
    ecs::{entity::Entity, query::With, system::NonSend},
    prelude::Single,
    utils::default,
    window::{PrimaryWindow, Window, WindowPlugin},
    winit::WinitWindows,
};
use winit::window::Icon;

pub fn plugin() -> WindowPlugin {
    WindowPlugin {
        primary_window: Some(Window {
            title: "Prison Manager".to_string(),
            // Bind to canvas included in `index.html`
            canvas: Some("#bevy".to_owned()),
            // Tells wasm not to override default event handling, like F5 and Ctrl+R
            prevent_default_event_handling: false,
            ..default()
        }),
        ..default()
    }
}

pub fn set_icon(
    windows: NonSend<WinitWindows>,
    primary_window: Single<Entity, With<PrimaryWindow>>,
) {
    let Some(primary) = windows.get_window(*primary_window) else {
        return;
    };

    let icon_buf = io::Cursor::new(include_bytes!("../build/icon.png"));
    let image = image::load(icon_buf, image::ImageFormat::Png)
        .unwrap()
        .into_rgba8();
    let (width, height) = image.dimensions();
    let rgba = image.into_raw();
    let icon = Icon::from_rgba(rgba, width, height).unwrap();
    primary.set_window_icon(Some(icon));
}
