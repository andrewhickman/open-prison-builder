use bevy::{ecs::system::Resource, render::color::Color};

#[derive(Resource)]
pub(crate) struct Theme {
    pub background: Color,
    pub text: Color,
    pub panel: Color,
    pub accent: Color,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            background: Color::hex("192a28").unwrap(),
            text: Color::hex("dedcdf").unwrap(),
            panel: Color::hex("5f4754").unwrap(),
            accent: Color::hex("b45627").unwrap(),
        }
    }
}
