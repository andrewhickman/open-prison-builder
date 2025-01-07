use bevy::{picking::focus::PickingInteraction, prelude::*, ui::widget::NodeImageMode};

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

impl<'w> UiBuilder<'w, '_> {
    pub fn large_button(
        &mut self,
        theme: &Theme,
        assets: &Assets,
        text: impl Into<String>,
        style: Node,
    ) -> UiBuilder<'w, '_> {
        self.text_button(
            theme,
            assets.button_image.clone(),
            text,
            theme.header_text.clone(),
            8.,
            style,
        )
    }

    pub fn button(
        &mut self,
        theme: &Theme,
        assets: &Assets,
        text: impl Into<String>,
        style: Node,
    ) -> UiBuilder<'w, '_> {
        self.text_button(
            theme,
            assets.button_image.clone(),
            text,
            theme.button_text.clone(),
            4.,
            style,
        )
    }

    pub fn text_button(
        &mut self,
        theme: &Theme,
        image: Handle<Image>,
        text: impl Into<String>,
        text_style: TextFont,
        button_border: f32,
        style: Node,
    ) -> UiBuilder<'w, '_> {
        let button_image_border = 64.;
        let slicer = TextureSlicer {
            border: BorderRect::square(button_image_border),
            center_scale_mode: SliceScaleMode::Stretch,
            sides_scale_mode: SliceScaleMode::Stretch,
            max_corner_scale: button_border / button_image_border,
        };

        let mut button = self.spawn((
            Button,
            Node {
                padding: UiRect::all(Val::Px(button_border * 1.5)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..style
            },
            ImageNode::new(image)
                .with_color(theme.text)
                .with_mode(NodeImageMode::Sliced(slicer)),
            PickingBehavior::default(),
            PickingInteraction::None,
            ButtonStyle::Text,
            Disabled(false),
        ));

        button.spawn((Text::new(text), text_style, PickingBehavior::IGNORE));
        button
    }

    pub fn icon_button(
        &mut self,
        theme: &Theme,
        icon: Handle<Image>,
        size: Val,
    ) -> UiBuilder<'w, '_> {
        self.spawn((
            Button,
            Node {
                width: Val::Auto,
                height: size,
                aspect_ratio: Some(1.),
                ..default()
            },
            ImageNode::new(icon.clone()).with_color(theme.text),
            PickingBehavior::default(),
            PickingInteraction::None,
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
            &mut ImageNode,
            &mut PickingBehavior,
        ),
        (
            With<Button>,
            Or<(Changed<PickingInteraction>, Changed<Disabled>)>,
        ),
    >,
) {
    for (interaction, disabled, style, mut image, mut pickable) in &mut changed {
        if disabled.0 {
            pickable.set_if_neq(PickingBehavior::IGNORE);
            image.flip_x = false;
            image.flip_y = false;
            image.color = theme.text.with_alpha(0.30);
            continue;
        } else {
            pickable.set_if_neq(PickingBehavior::default());
        }

        match style {
            ButtonStyle::Text => match interaction {
                PickingInteraction::None => {
                    image.flip_x = false;
                    image.flip_y = false;
                    image.color = theme.text;
                }
                PickingInteraction::Hovered => {
                    image.flip_x = false;
                    image.flip_y = false;
                    image.color = theme.text.with_alpha(0.88);
                }
                PickingInteraction::Pressed => {
                    image.flip_x = true;
                    image.flip_y = true;
                    image.color = theme.text.with_alpha(0.88);
                }
            },
            ButtonStyle::Icon => match interaction {
                PickingInteraction::None => {
                    image.color = theme.text;
                }
                PickingInteraction::Hovered => {
                    image.color = theme.text.with_alpha(0.80);
                }
                PickingInteraction::Pressed => {
                    image.color = theme.text.with_alpha(0.60);
                }
            },
        }
    }
}
