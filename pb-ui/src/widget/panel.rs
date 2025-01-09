use bevy::{prelude::*, ui::FocusPolicy};

use pb_assets::Assets;

use crate::{theme::Theme, widget::UiBuilder};

#[derive(Default, Resource)]
pub struct PanelStack {
    entities: Vec<Entity>,
}

#[derive(Default, Copy, Clone, Component)]
pub struct Panel;

pub fn on_add(trigger: Trigger<OnAdd, Panel>, mut stack: ResMut<PanelStack>) {
    stack.entities.push(trigger.entity());
}

pub fn on_remove(trigger: Trigger<OnRemove, Panel>, mut stack: ResMut<PanelStack>) {
    if let Some(pos) = stack.entities.iter().position(|&e| e == trigger.entity()) {
        stack.entities.remove(pos);
    }
}

impl PanelStack {
    pub fn pop(&mut self) -> Option<Entity> {
        self.entities.pop()
    }
}

impl<'w> UiBuilder<'w, '_> {
    pub fn empty_panel(&mut self, theme: &Theme, style: Node) -> UiBuilder<'w, '_> {
        self.spawn((
            Node {
                padding: UiRect::all(theme.gutter),
                ..style
            },
            BackgroundColor(theme.panel),
            FocusPolicy::Block,
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
            Node {
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                row_gap: theme.gutter,
                ..default()
            },
        );
        panel.insert(Panel);
        let panel_id = panel.id();

        let mut title_row = panel.container(Node {
            display: Display::Flex,
            flex_direction: FlexDirection::Row,
            justify_content: JustifyContent::SpaceBetween,
            align_items: AlignItems::Center,
            column_gap: theme.gutter,
            ..default()
        });

        title_row.spawn((Text::new(title), theme.header_text.clone()));
        title_row
            .icon_button(theme, assets.close_icon.clone(), theme.icon_size())
            .on_click(
                move |_: Trigger<'_, Pointer<Click>>, mut commands: Commands| {
                    commands.entity(panel_id).despawn_recursive()
                },
            );

        panel
    }
}
