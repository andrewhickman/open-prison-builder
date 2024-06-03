use bevy::prelude::*;
use bevy_mod_picking::prelude::*;

use crate::{theme::Theme, widget::UiBuilder};

#[derive(Component)]
pub enum ButtonStyle {
    Text,
    Icon,
}

impl<'a> UiBuilder<'a> {
    pub fn large_button<Marker>(
        &mut self,
        theme: &Theme,
        text: impl Into<String>,
        callback: impl IntoSystem<(), (), Marker>,
    ) -> UiBuilder<'_> {
        let mut button = self.spawn((
            ButtonBundle {
                style: Style {
                    margin: UiRect::all(theme.gutter),
                    padding: theme.button_padding,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                image: theme.button_image.clone(),
                ..default()
            },
            theme.button_slice.clone(),
            On::<Pointer<Click>>::run(callback),
            PickableBundle::default(),
            ButtonStyle::Text,
        ));

        button.spawn((
            TextBundle::from_section(text, theme.large_button_text.clone()),
            Pickable::IGNORE,
        ));
        button
    }

    pub fn icon_button<Marker>(
        &mut self,
        theme: &Theme,
        icon: Handle<Image>,
        callback: impl IntoSystem<(), (), Marker>,
    ) -> UiBuilder<'_> {
        self.spawn((
            ButtonBundle {
                style: Style {
                    margin: UiRect::all(theme.gutter),
                    width: Val::Auto,
                    height: theme.icon_size,
                    aspect_ratio: Some(1.),
                    ..default()
                },
                image: UiImage::new(icon.clone()),
                ..default()
            },
            On::<Pointer<Click>>::run(callback),
            PickableBundle::default(),
            ButtonStyle::Icon,
        ))
    }
}

pub fn update(
    mut changed: Query<
        (
            &PickingInteraction,
            &ButtonStyle,
            &mut UiImage,
            &mut BackgroundColor,
        ),
        (With<Button>, Changed<PickingInteraction>),
    >,
) {
    for (interaction, style, mut image, mut color) in &mut changed {
        match style {
            ButtonStyle::Text => match interaction {
                PickingInteraction::None => {
                    image.flip_x = false;
                    image.flip_y = false;
                    color.0 = Color::WHITE;
                }
                PickingInteraction::Hovered => {
                    image.flip_x = false;
                    image.flip_y = false;
                    color.0 = Color::WHITE.with_a(0.88);
                }
                PickingInteraction::Pressed => {
                    image.flip_x = true;
                    image.flip_y = true;
                    color.0 = Color::WHITE.with_a(0.88);
                }
            },
            ButtonStyle::Icon => match interaction {
                PickingInteraction::None => {
                    color.0 = Color::WHITE;
                }
                PickingInteraction::Hovered => {
                    color.0 = Color::WHITE.with_a(0.80);
                }
                PickingInteraction::Pressed => {
                    color.0 = Color::WHITE.with_a(0.60);
                }
            },
        }
    }
}
