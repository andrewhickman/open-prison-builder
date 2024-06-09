use bevy::prelude::*;

use pb_assets::Assets;

use crate::{theme::Theme, widget::UiBuilder};

impl<'w, 's> UiBuilder<'w, 's> {
    pub fn error_message(
        &mut self,
        theme: &Theme,
        assets: &Assets,
        message: String,
    ) -> UiBuilder<'w, '_> {
        let mut container = self.container(Style {
            display: Display::Flex,
            flex_direction: FlexDirection::Row,
            column_gap: theme.gutter,
            ..default()
        });

        container.spawn(ImageBundle {
            style: Style {
                width: Val::Auto,
                height: theme.icon_size(),
                aspect_ratio: Some(1.),
                ..default()
            },
            image: UiImage::new(assets.error_icon.clone()),
            ..default()
        });
        container.spawn(TextBundle::from_section(
            message,
            theme.emphasis_text.clone(),
        ));

        container
    }
}
