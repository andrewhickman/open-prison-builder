use bevy::prelude::*;

use pb_assets::AssetHandles;

use crate::{theme::Theme, widget::UiBuilder};

impl<'w> UiBuilder<'w, '_> {
    pub fn error_message(
        &mut self,
        theme: &Theme,
        assets: &AssetHandles,
        message: String,
    ) -> UiBuilder<'w, '_> {
        let mut container = self.container(Node {
            display: Display::Flex,
            flex_direction: FlexDirection::Row,
            column_gap: theme.gutter,
            ..default()
        });

        container.spawn((
            Node {
                width: Val::Auto,
                height: theme.icon_size(),
                aspect_ratio: Some(1.),
                ..default()
            },
            ImageNode::new(assets.error_icon.clone()),
        ));
        container.spawn((Text::new(message), theme.emphasis_text.clone()));

        container
    }
}
