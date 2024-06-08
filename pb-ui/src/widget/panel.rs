use bevy::prelude::*;

use pb_assets::Assets;

use crate::{theme::Theme, widget::UiBuilder};

impl<'w, 's> UiBuilder<'w, 's> {
    pub fn panel(&mut self, theme: &Theme, style: Style) -> UiBuilder<'w, '_> {
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

    pub fn titled_panel(
        &mut self,
        theme: &Theme,
        assets: &Assets,
        title: impl Into<String>,
    ) -> UiBuilder<'w, '_> {
        let mut panel = self.panel(
            theme,
            Style {
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                row_gap: theme.gutter,
                ..default()
            },
        );
        let panel_id = panel.id();

        let mut title_row = panel.container(Style {
            display: Display::Flex,
            flex_direction: FlexDirection::Row,
            justify_content: JustifyContent::SpaceBetween,
            align_items: AlignItems::Center,
            column_gap: theme.gutter,
            ..default()
        });

        title_row.spawn(TextBundle::from_section(title, theme.header_text.clone()));
        title_row.icon_button(
            assets.close_icon.clone(),
            theme.icon_size(),
            move |mut commands: Commands| commands.entity(panel_id).despawn_recursive(),
        );

        panel
    }
}
