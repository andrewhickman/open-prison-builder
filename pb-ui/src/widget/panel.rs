use bevy::{prelude::*, ui::FocusPolicy};

use pb_assets::Assets;

use crate::{theme::Theme, widget::UiBuilder};

#[derive(Default, Resource)]
pub struct PanelStack {
    entities: Vec<Entity>,
}

#[derive(Default, Copy, Clone, Component)]
pub struct Panel;

pub fn update(
    mut stack: ResMut<PanelStack>,
    added: Query<Entity, Added<Panel>>,
    mut removed: RemovedComponents<Panel>,
) {
    for entity in &added {
        stack.entities.push(entity);
    }

    for entity in removed.read() {
        if let Some(pos) = stack.entities.iter().position(|&e| e == entity) {
            stack.entities.remove(pos);
        }
    }
}

impl PanelStack {
    pub fn pop(&mut self) -> Option<Entity> {
        self.entities.pop()
    }
}

impl<'w, 's> UiBuilder<'w, 's> {
    pub fn empty_panel(&mut self, theme: &Theme, style: Style) -> UiBuilder<'w, '_> {
        self.spawn((
            NodeBundle {
                style: Style {
                    padding: UiRect::all(theme.gutter),
                    ..style
                },
                background_color: theme.panel.into(),
                focus_policy: FocusPolicy::Block,
                ..default()
            },
            theme.outline,
        ))
    }

    pub fn panel(
        &mut self,
        theme: &Theme,
        assets: &Assets,
        title: impl Into<String>,
    ) -> UiBuilder<'w, '_> {
        let mut panel = self.empty_panel(
            theme,
            Style {
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                row_gap: theme.gutter,
                ..default()
            },
        );
        panel.insert(Panel);
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
