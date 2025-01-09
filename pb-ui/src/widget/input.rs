use bevy::prelude::*;

use bevy_simple_text_input::{TextInput, TextInputSettings, TextInputTextFont, TextInputValue};

use crate::{theme::Theme, widget::UiBuilder};

use super::form::{FormField, FormUpdate};

impl<'w> UiBuilder<'w, '_> {
    pub fn input(&mut self, theme: &Theme) -> UiBuilder<'w, '_> {
        self.spawn((
            Node {
                flex_grow: 1.,
                border: UiRect::all(Val::Px(1.0)),
                padding: UiRect::all(Val::Px(4.)),
                ..default()
            },
            BorderColor(theme.text),
            BackgroundColor(theme.panel),
            TextInput,
            TextInputTextFont(theme.normal_text.clone()),
            TextInputSettings {
                retain_on_submit: true,
                ..default()
            },
        ))
    }
}

pub fn update(
    mut commands: Commands,
    input_q: Query<(Entity, &FormField, &TextInputValue), Changed<TextInputValue>>,
) {
    for (target, field, value) in &input_q {
        commands.trigger_targets(
            FormUpdate {
                target,
                name: field.name().into(),
                value: value.0.clone_value(),
            },
            target,
        );
    }
}
