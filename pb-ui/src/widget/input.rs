use bevy::prelude::*;

use bevy_simple_text_input::{TextInputBundle, TextInputSettings, TextInputValue};

use crate::{theme::Theme, widget::UiBuilder};

use super::form::{FormField, FormUpdate};

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

pub fn update(
    input_q: Query<(Entity, &FormField, &TextInputValue), Changed<TextInputValue>>,
    mut update_e: EventWriter<FormUpdate>,
) {
    for (target, field, value) in &input_q {
        update_e.send(FormUpdate {
            target,
            name: field.name().into(),
            value: value.0.clone_value(),
        });
    }
}
