use bevy::prelude::*;

use crate::{theme::Theme, widget::UiBuilder};

impl<'a> UiBuilder<'a> {
    pub fn panel(&mut self, theme: &Theme, style: Style) -> UiBuilder<'_> {
        self.spawn((
            NodeBundle {
                style: Style {
                    padding: UiRect::all(theme.gutter),
                    ..style
                },
                background_color: theme.panel.into(),
                ..default()
            },
            theme.outline,
        ))
    }
}
