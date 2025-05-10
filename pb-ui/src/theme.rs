use bevy::prelude::*;

use pb_assets::AssetHandles;

#[derive(Resource)]
pub(crate) struct Theme {
    pub background: Color,
    pub text: Color,
    pub panel: Color,
    #[expect(unused)]
    pub accent: Color,
    #[expect(unused)]
    pub error: Color,
    pub gutter: Val,
    pub outline: Outline,
    pub normal_text: TextFont,
    pub emphasis_text: TextFont,
    pub button_text: TextFont,
    pub header_text: TextFont,
    pub large_icon_size_px: f32,
    pub icon_size_px: f32,
    pub tile_icon_size_px: f32,
}

pub fn init(mut commands: Commands, assets: Res<AssetHandles>) -> Result {
    let text = Srgba::hex("dedcdf")?.into();

    commands.insert_resource(Theme {
        background: Srgba::hex("192a28")?.into(),
        text,
        panel: Srgba::hex("5f4754")?.into(),
        accent: Srgba::hex("b45627")?.into(),
        error: Srgba::hex("f2200d")?.into(),
        gutter: Val::Px(8.),
        outline: Outline {
            color: text,
            width: Val::Px(1.),
            ..default()
        },
        normal_text: TextFont {
            font: assets.font_tomorrow.clone(),
            font_size: 16.,
            ..Default::default()
        },
        emphasis_text: TextFont {
            font: assets.tomorrow_italic_font.clone(),
            font_size: 16.,
            ..Default::default()
        },
        button_text: TextFont {
            font: assets.font_graduate.clone(),
            font_size: 14.,
            ..Default::default()
        },
        header_text: TextFont {
            font: assets.font_graduate.clone(),
            font_size: 20.,
            ..Default::default()
        },
        icon_size_px: 18.,
        large_icon_size_px: 26.,
        tile_icon_size_px: 36.,
    });
    Ok(())
}

impl Theme {
    pub fn large_icon_size(&self) -> Val {
        Val::Px(self.large_icon_size_px)
    }

    pub fn icon_size(&self) -> Val {
        Val::Px(self.icon_size_px)
    }

    pub fn tile_icon_size(&self) -> Val {
        Val::Px(self.tile_icon_size_px)
    }
}
