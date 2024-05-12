use bevy::prelude::*;

pub struct ThemePlugin;

impl Plugin for ThemePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Theme::default())
            .add_systems(Update, update_button_color);
    }
}

#[derive(Resource)]
pub struct Theme {
    game_background: Color,
    text: Color,
    ui_background: Color,
    button: Color,
    bold_button: Color,
}

#[derive(Default, Component)]
pub enum ButtonTheme {
    #[default]
    Normal,
    Bold,
}

impl Theme {
    pub fn game_background(&self) -> Color {
        self.game_background
    }

    pub fn ui_background(&self) -> Color {
        self.ui_background
    }

    pub fn text(&self) -> Color {
        self.text
    }

    pub fn button(&self) -> Color {
        self.button
    }

    pub fn bold_button(&self) -> Color {
        self.bold_button
    }
}

impl Default for Theme {
    fn default() -> Self {
        Theme {
            game_background: Color::hex("#081207").unwrap(),
            text: Color::hex("#f5f5f5").unwrap(),
            ui_background: Color::hex("#706e62").unwrap(),
            button: Color::hex("#5F4754").unwrap(),
            bold_button: Color::hex("#485921").unwrap(),
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

pub fn update_button_color(
    theme: ResMut<Theme>,
    mut interaction_query: Query<
        (&Interaction, &ButtonTheme, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, style, mut color) in &mut interaction_query {
        let base_color = match style {
            ButtonTheme::Normal => theme.button(),
            ButtonTheme::Bold => theme.bold_button(),
        };

        color.0 = match interaction {
            Interaction::None => base_color,
            Interaction::Hovered => hot(base_color, theme.text()),
            Interaction::Pressed => active(base_color, theme.text()),
        };
    }
}
