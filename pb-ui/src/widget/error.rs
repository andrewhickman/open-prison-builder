use std::fmt::Write;

use bevy::prelude::*;

use pb_assets::Assets;

use crate::{theme::Theme, widget::UiBuilder};

impl<'w, 's> UiBuilder<'w, 's> {
    pub fn error_message(
        &mut self,
        theme: &Theme,
        assets: &Assets,
        error: &dyn std::error::Error,
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
            fmt_error(error),
            theme.emphasis_text.clone(),
        ));

        container
    }
}

fn fmt_error(mut error: &dyn std::error::Error) -> String {
    let mut buf = error.to_string();
    while let Some(source) = error.source() {
        write!(buf, ": {}", source).unwrap();
        error = source;
    }
    buf
}
