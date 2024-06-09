use bevy::prelude::*;

use bevy_simple_text_input::{TextInputBundle, TextInputSettings};

use crate::{theme::Theme, widget::UiBuilder};

impl<'w, 's> UiBuilder<'w, 's> {
    pub fn input(&mut self, theme: &Theme) -> UiBuilder<'w, '_> {
        self.spawn((
            NodeBundle {
                style: Style {
                    flex_grow: 1.,
                    border: UiRect::all(Val::Px(1.0)),
                    padding: UiRect::all(Val::Px(4.)),
                    ..default()
                },
                border_color: theme.text.into(),
                background_color: theme.panel.into(),
                ..default()
            },
            TextInputBundle::default()
                .with_text_style(theme.normal_text.clone())
                .with_settings(TextInputSettings {
                    retain_on_submit: true,
                    ..default()
                }),
        ))
    }
}
