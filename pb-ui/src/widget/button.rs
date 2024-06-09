use bevy::prelude::*;
use bevy_mod_picking::prelude::*;

use pb_assets::Assets;

use crate::{
    theme::Theme,
    widget::{Disabled, UiBuilder},
};

#[derive(Component, Debug, Clone, PartialEq, Eq)]
pub enum ButtonStyle {
    Text,
    Icon,
}

impl<'w, 's> UiBuilder<'w, 's> {
    pub fn large_button<Marker>(
        &mut self,
        theme: &Theme,
        assets: &Assets,
        text: impl Into<String>,
        style: Style,
        callback: impl IntoSystem<(), (), Marker>,
    ) -> UiBuilder<'w, '_> {
        self.text_button(assets, text, theme.header_text.clone(), 8., style, callback)
    }

    pub fn button<Marker>(
        &mut self,
        theme: &Theme,
        assets: &Assets,
        text: impl Into<String>,
        style: Style,
        callback: impl IntoSystem<(), (), Marker>,
    ) -> UiBuilder<'w, '_> {
        self.text_button(assets, text, theme.button_text.clone(), 4., style, callback)
    }

    fn text_button<Marker>(
        &mut self,
        assets: &Assets,
        text: impl Into<String>,
        text_style: TextStyle,
        button_border: f32,
        style: Style,
        callback: impl IntoSystem<(), (), Marker>,
    ) -> UiBuilder<'w, '_> {
        let button_image_border = 64.;
        let button_slice = ImageScaleMode::Sliced(TextureSlicer {
            border: BorderRect::square(button_image_border),
            center_scale_mode: SliceScaleMode::Stretch,
            sides_scale_mode: SliceScaleMode::Stretch,
            max_corner_scale: button_border / button_image_border,
        });

        let mut button = self.spawn((
            ButtonBundle {
                style: Style {
                    padding: UiRect::all(Val::Px(button_border * 1.5)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..style
                },
                image: UiImage::new(assets.button_image.clone()),
                ..default()
            },
            button_slice,
            On::<Pointer<Click>>::run(callback),
            PickableBundle::default(),
            ButtonStyle::Text,
            Disabled(false),
        ));

        button.spawn((TextBundle::from_section(text, text_style), Pickable::IGNORE));
        button
    }

    pub fn icon_button<Marker>(
        &mut self,
        icon: Handle<Image>,
        size: Val,
        callback: impl IntoSystem<(), (), Marker>,
    ) -> UiBuilder<'w, '_> {
        self.spawn((
            ButtonBundle {
                style: Style {
                    width: Val::Auto,
                    height: size,
                    aspect_ratio: Some(1.),
                    ..default()
                },
                image: UiImage::new(icon.clone()),
                ..default()
            },
            On::<Pointer<Click>>::run(callback),
            PickableBundle::default(),
            ButtonStyle::Icon,
            Disabled(false),
        ))
    }
}

pub fn update(
    theme: Res<Theme>,
    mut changed: Query<
        (
            &PickingInteraction,
            &Disabled,
            &ButtonStyle,
            &mut UiImage,
            &mut BackgroundColor,
            &mut Pickable,
        ),
        (
            With<Button>,
            Or<(Changed<PickingInteraction>, Changed<Disabled>)>,
        ),
    >,
) {
    for (interaction, disabled, style, mut image, mut color, mut pickable) in &mut changed {
        if disabled.0 {
            pickable.set_if_neq(Pickable::IGNORE);
            color.0 = theme.text.with_a(0.30);
            image.flip_x = false;
            image.flip_y = false;
            continue;
        } else {
            pickable.set_if_neq(Pickable::default());
        }

        match style {
            ButtonStyle::Text => match interaction {
                PickingInteraction::None => {
                    image.flip_x = false;
                    image.flip_y = false;
                    color.0 = theme.text;
                }
                PickingInteraction::Hovered => {
                    image.flip_x = false;
                    image.flip_y = false;
                    color.0 = theme.text.with_a(0.88);
                }
                PickingInteraction::Pressed => {
                    image.flip_x = true;
                    image.flip_y = true;
                    color.0 = theme.text.with_a(0.88);
                }
            },
            ButtonStyle::Icon => match interaction {
                PickingInteraction::None => {
                    color.0 = theme.text;
                }
                PickingInteraction::Hovered => {
                    color.0 = theme.text.with_a(0.80);
                }
                PickingInteraction::Pressed => {
                    color.0 = theme.text.with_a(0.60);
                }
            },
        }
    }
}
