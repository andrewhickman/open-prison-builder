use bevy::{prelude::*, ui::widget::NodeImageMode};

use pb_assets::AssetHandles;

use crate::{theme::Theme, widget::UiBuilder};

use super::disabled::DisabledChanged;

#[derive(Component, Debug, Clone, PartialEq, Eq)]
#[require(Button)]
pub enum ButtonStyle {
    Text,
    Icon,
    Tile,
}

impl<'w> UiBuilder<'w, '_> {
    pub fn large_button(
        &mut self,
        theme: &Theme,
        assets: &AssetHandles,
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
        assets: &AssetHandles,
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
            border: BorderRect::all(button_image_border),
            center_scale_mode: SliceScaleMode::Stretch,
            sides_scale_mode: SliceScaleMode::Stretch,
            max_corner_scale: button_border / button_image_border,
        };

        let mut button = self.base_button(
            ButtonStyle::Text,
            (
                Node {
                    padding: UiRect::all(Val::Px(button_border * 1.5)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..style
                },
                ImageNode::new(image)
                    .with_color(theme.text)
                    .with_mode(NodeImageMode::Sliced(slicer)),
            ),
        );

        button.spawn((Text::new(text), text_style, Pickable::IGNORE));
        button
    }

    pub fn icon_button(
        &mut self,
        theme: &Theme,
        icon: Handle<Image>,
        size: Val,
    ) -> UiBuilder<'w, '_> {
        self.base_button(
            ButtonStyle::Icon,
            (
                Node {
                    width: Val::Auto,
                    height: size,
                    aspect_ratio: Some(1.),
                    ..default()
                },
                ImageNode::new(icon.clone()).with_color(theme.text),
            ),
        )
    }

    pub fn tile_button(
        &mut self,
        theme: &Theme,
        title: impl Into<String>,
        icon: Handle<Image>,
    ) -> UiBuilder<'w, '_> {
        let mut container = self.base_button(
            ButtonStyle::Tile,
            Node {
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                ..default()
            },
        );

        container.spawn((
            ImageNode::new(icon),
            Node {
                width: theme.tile_icon_size(),
                height: theme.tile_icon_size(),
                aspect_ratio: Some(1.),
                ..default()
            },
            Pickable::IGNORE,
        ));
        container.spawn((
            Node {
                max_width: Val::Px(60.),
                ..default()
            },
            Text::new(title),
            theme.button_text.clone(),
            TextLayout {
                justify: JustifyText::Center,
                linebreak: LineBreak::WordBoundary,
            },
            Pickable::IGNORE,
        ));

        container
    }

    fn base_button(&mut self, style: ButtonStyle, bundle: impl Bundle) -> UiBuilder<'w, '_> {
        let mut builder = self.spawn((style, bundle));

        builder
            .observe(over)
            .observe(out)
            .observe(pressed)
            .observe(released)
            .observe(disabled);

        builder
    }
}

fn over(
    mut trigger: Trigger<Pointer<Over>>,
    theme: Res<Theme>,
    mut button_q: Query<(
        &ButtonStyle,
        Option<&mut ImageNode>,
        Option<&mut BackgroundColor>,
    )>,
) -> Result<()> {
    trigger.propagate(false);

    let (style, mut image, mut bg) = button_q.get_mut(trigger.target())?;
    style.hovered(&theme, image.as_deref_mut(), bg.as_deref_mut());
    Ok(())
}

fn out(
    mut trigger: Trigger<Pointer<Out>>,
    theme: Res<Theme>,
    mut button_q: Query<(
        &ButtonStyle,
        Option<&mut ImageNode>,
        Option<&mut BackgroundColor>,
    )>,
) -> Result {
    trigger.propagate(false);

    let (style, mut image, mut bg) = button_q.get_mut(trigger.target())?;
    style.normal(&theme, image.as_deref_mut(), bg.as_deref_mut());
    Ok(())
}

fn pressed(
    mut trigger: Trigger<Pointer<Pressed>>,
    theme: Res<Theme>,
    mut button_q: Query<(
        &ButtonStyle,
        Option<&mut ImageNode>,
        Option<&mut BackgroundColor>,
    )>,
) -> Result {
    trigger.propagate(false);

    if trigger.button == PointerButton::Primary {
        let (style, mut image, mut bg) = button_q.get_mut(trigger.target())?;
        style.active(&theme, image.as_deref_mut(), bg.as_deref_mut());
    }
    Ok(())
}

fn released(
    mut trigger: Trigger<Pointer<Released>>,
    theme: Res<Theme>,
    mut button_q: Query<(
        &ButtonStyle,
        Option<&mut ImageNode>,
        Option<&mut BackgroundColor>,
    )>,
) -> Result {
    trigger.propagate(false);

    if trigger.button == PointerButton::Primary {
        let (style, mut image, mut bg) = button_q.get_mut(trigger.target())?;
        style.hovered(&theme, image.as_deref_mut(), bg.as_deref_mut());
    }
    Ok(())
}

fn disabled(
    mut trigger: Trigger<DisabledChanged>,
    theme: Res<Theme>,
    mut button_q: Query<(
        &ButtonStyle,
        &mut Pickable,
        Option<&mut ImageNode>,
        Option<&mut BackgroundColor>,
    )>,
) -> Result {
    trigger.propagate(false);

    let (style, mut behaviour, mut image, mut bg) = button_q.get_mut(trigger.target())?;
    if trigger.0 {
        behaviour.set_if_neq(Pickable::IGNORE);
        style.disabled(&theme, image.as_deref_mut(), bg.as_deref_mut());
    } else {
        behaviour.set_if_neq(Pickable::default());
        style.normal(&theme, image.as_deref_mut(), bg.as_deref_mut());
    }

    Ok(())
}

impl ButtonStyle {
    fn normal(
        &self,
        theme: &Theme,
        image: Option<&mut ImageNode>,
        bg: Option<&mut BackgroundColor>,
    ) {
        match self {
            ButtonStyle::Text => {
                if let Some(image) = image {
                    image.flip_x = false;
                    image.flip_y = false;
                    image.color = theme.text;
                }
            }
            ButtonStyle::Icon => {
                if let Some(image) = image {
                    image.color = theme.text;
                }
            }
            ButtonStyle::Tile => {
                if let Some(bg) = bg {
                    bg.0 = Color::NONE;
                }
            }
        }
    }

    fn hovered(
        &self,
        theme: &Theme,
        image: Option<&mut ImageNode>,
        bg: Option<&mut BackgroundColor>,
    ) {
        match self {
            ButtonStyle::Text => {
                if let Some(image) = image {
                    image.flip_x = false;
                    image.flip_y = false;
                    image.color = theme.text.with_alpha(0.88);
                }
            }
            ButtonStyle::Icon => {
                if let Some(image) = image {
                    image.color = theme.text.with_alpha(0.80);
                }
            }
            ButtonStyle::Tile => {
                if let Some(bg) = bg {
                    bg.0 = theme.text.with_alpha(0.40);
                }
            }
        }
    }

    fn active(
        &self,
        theme: &Theme,
        image: Option<&mut ImageNode>,
        bg: Option<&mut BackgroundColor>,
    ) {
        match self {
            ButtonStyle::Text => {
                if let Some(image) = image {
                    image.flip_x = true;
                    image.flip_y = true;
                    image.color = theme.text.with_alpha(0.88);
                }
            }
            ButtonStyle::Icon => {
                if let Some(image) = image {
                    image.color = theme.text.with_alpha(0.60);
                }
            }
            ButtonStyle::Tile => {
                if let Some(bg) = bg {
                    bg.0 = theme.text.with_alpha(0.60);
                }
            }
        }
    }

    fn disabled(
        &self,
        theme: &Theme,
        image: Option<&mut ImageNode>,
        _: Option<&mut BackgroundColor>,
    ) {
        match self {
            ButtonStyle::Text => {
                if let Some(image) = image {
                    image.flip_x = false;
                    image.flip_y = false;
                    image.color = theme.text.with_alpha(0.30);
                }
            }
            ButtonStyle::Icon => {
                if let Some(image) = image {
                    image.color = theme.text.with_alpha(0.30);
                }
            }
            ButtonStyle::Tile => {}
        }
    }
}
