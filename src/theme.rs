use bevy::{
    app::{App, Plugin},
    ecs::system::Resource,
    render::color::Color,
};

pub struct ThemePlugin;

impl Plugin for ThemePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Theme::default());
    }
}

#[derive(Resource)]
pub struct Theme {
    background: Color,
    text: Color,
    #[allow(unused)]
    detail1: Color,
    #[allow(unused)]
    detail2: Color,
    #[allow(unused)]
    detail3: Color,
    #[allow(unused)]
    accent1: Color,
    #[allow(unused)]
    accent2: Color,
    #[allow(unused)]
    accent3: Color,
}

impl Theme {
    pub fn background(&self) -> Color {
        self.background
    }

    pub fn text(&self) -> Color {
        self.text
    }

    pub fn button(&self) -> Color {
        self.detail2
    }

    pub fn button_hot(&self) -> Color {
        hot(self.button(), self.text)
    }

    pub fn button_active(&self) -> Color {
        active(self.button(), self.text)
    }
}

impl Default for Theme {
    fn default() -> Self {
        Theme {
            background: Color::rgb(0.031, 0.071, 0.027),
            text: Color::rgb(0.871, 0.863, 0.875),
            detail1: Color::rgb(0.439, 0.431, 0.384),
            detail2: Color::rgb(0.373, 0.278, 0.329),
            detail3: Color::rgb(0.553, 0.635, 0.561),
            accent1: Color::rgb(0.482, 0.698, 0.851),
            accent2: Color::rgb(0.357, 0.729, 0.435),
            accent3: Color::rgb(0.929, 0.278, 0.290),
        }
    }
}

fn active(color: Color, text: Color) -> Color {
    mix(text, color, 0.32)
}

fn hot(color: Color, text: Color) -> Color {
    mix(text, color, 0.08)
}

fn mix(color1: Color, color2: Color, weight: f32) -> Color {
    let normalized_weight = weight * 2.0 - 1.0;
    let alpha_distance = color1.a() - color2.a();

    let mut combined_weight =
        (normalized_weight + alpha_distance) / (1.0 + normalized_weight * alpha_distance);
    if !combined_weight.is_finite() {
        combined_weight = normalized_weight;
    }

    let weight1 = (combined_weight + 1.0) / 2.0;
    let weight2 = 1.0 - weight1;

    Color::rgba(
        color1.r() * weight1 + color2.r() * weight2,
        color1.g() * weight1 + color2.g() * weight2,
        color1.b() * weight1 + color2.b() * weight2,
        color1.a() * weight + color2.a() * (1.0 - weight),
    )
}
