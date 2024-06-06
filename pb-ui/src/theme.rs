use bevy::prelude::*;

use pb_assets::Assets;

#[derive(Resource)]
pub(crate) struct Theme {
    pub background: Color,
    pub text: Color,
    pub panel: Color,
    #[allow(unused)]
    pub accent: Color,
    pub gutter: Val,
    pub outline: Outline,
    #[allow(unused)]
    pub normal_text: TextStyle,
    #[allow(unused)]
    pub emphasis_text: TextStyle,
    #[allow(unused)]
    pub button_text: TextStyle,
    pub large_button_text: TextStyle,
    pub button_slice: ImageScaleMode,
    pub button_image: UiImage,
    pub button_padding: UiRect,
    pub icon_size: Val,
}

pub fn init(mut commands: Commands, assets: Res<Assets>) {
    let text = Color::hex("dedcdf").unwrap();

    let button_image_border = 64.;
    let button_border = 5.;
    let button_slice = ImageScaleMode::Sliced(TextureSlicer {
        border: BorderRect::square(button_image_border),
        center_scale_mode: SliceScaleMode::Stretch,
        sides_scale_mode: SliceScaleMode::Stretch,
        max_corner_scale: button_border / button_image_border,
    });

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
            font: assets.tomorrow_italic_font.clone(),
            font_size: 16.,
        },
        button_text: TextStyle {
            color: text,
            font: assets.font_graduate.clone(),
            font_size: 14.,
        },
        large_button_text: TextStyle {
            color: text,
            font: assets.font_graduate.clone(),
            font_size: 18.,
        },
        button_slice,
        button_image: UiImage::new(assets.button_image.clone()),
        button_padding: UiRect::all(Val::Px(button_border * 2.)),
        icon_size: Val::Px(24.),
    });
}
