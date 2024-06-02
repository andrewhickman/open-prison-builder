use bevy::prelude::*;

use pb_assets::Assets;

#[derive(Resource)]
#[allow(unused)]
pub(crate) struct Theme {
    pub background: Color,
    pub text: Color,
    pub panel: Color,
    pub accent: Color,
    pub gutter: Val,
    pub outline: Outline,
    pub normal_text: TextStyle,
    pub emphasis_text: TextStyle,
    pub button_text: TextStyle,
    pub header_text: TextStyle,
}

pub fn init(mut commands: Commands, assets: Res<Assets>) {
    let text = Color::hex("dedcdf").unwrap();
    commands.insert_resource(Theme {
        background: Color::hex("192a28").unwrap(),
        text: Color::hex("dedcdf").unwrap(),
        panel: Color::hex("5f4754").unwrap(),
        accent: Color::hex("b45627").unwrap(),
        gutter: Val::Px(5.),
        outline: Outline {
            color: text,
            width: Val::Px(1.),
            ..default()
        },
        normal_text: TextStyle {
            color: text,
            font: assets.font_tomorrow.clone(),
            font_size: 16.,
        },
        emphasis_text: TextStyle {
            color: text,
            font: assets.font_tomorrow.clone(),
            font_size: 16.,
        },
        button_text: TextStyle {
            color: text,
            font: assets.font_tomorrow_bold.clone(),
            font_size: 14.,
        },
        header_text: TextStyle {
            color: text,
            font: assets.font_graduate.clone(),
            font_size: 60.,
        },
    });
}
