use bevy::prelude::*;

use pb_assets::Assets;

#[derive(Resource)]
pub(crate) struct Theme {
    pub background: Color,
    pub text: Color,
    pub panel: Color,
    #[allow(unused)]
    pub accent: Color,
    #[allow(unused)]
    pub error: Color,
    pub gutter: Val,
    pub outline: Outline,
    pub normal_text: TextStyle,
    pub emphasis_text: TextStyle,
    pub button_text: TextStyle,
    pub header_text: TextStyle,
    pub large_icon_size_px: f32,
    pub icon_size_px: f32,
}

pub fn init(mut commands: Commands, assets: Res<Assets>) {
    let text = Srgba::hex("dedcdf").unwrap().into();

    commands.insert_resource(Theme {
        background: Srgba::hex("192a28").unwrap().into(),
        text: Srgba::hex("dedcdf").unwrap().into(),
        panel: Srgba::hex("5f4754").unwrap().into(),
        accent: Srgba::hex("b45627").unwrap().into(),
        error: Srgba::hex("f2200d").unwrap().into(),
        gutter: Val::Px(8.),
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
            font: assets.tomorrow_italic_font.clone(),
            font_size: 16.,
        },
        button_text: TextStyle {
            color: text,
            font: assets.font_graduate.clone(),
            font_size: 14.,
        },
        header_text: TextStyle {
            color: text,
            font: assets.font_graduate.clone(),
            font_size: 20.,
        },
        icon_size_px: 18.,
        large_icon_size_px: 26.,
    });
}

impl Theme {
    pub fn large_icon_size(&self) -> Val {
        Val::Px(self.large_icon_size_px)
    }

    pub fn icon_size(&self) -> Val {
        Val::Px(self.icon_size_px)
    }
}
