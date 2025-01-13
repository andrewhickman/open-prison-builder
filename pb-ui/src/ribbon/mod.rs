mod wall;

use bevy::prelude::*;

use pb_assets::Assets;

use crate::{layout::Layout, theme::Theme, widget::UiBuilder, UiState};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum RibbonState {
    Shown,
    Hidden,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Component)]
pub enum RibbonButton {
    Architect,
    Staff,
    Schedule,
    Manage,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Component)]
pub enum RibbonPanel {
    Architect,
    Staff,
    Schedule,
    Manage,
}

pub fn show(commands: Commands, layout: Res<Layout>, theme: Res<Theme>, assets: Res<Assets>) {
    UiBuilder::new(commands, layout.ribbon).ribbon(&theme, &assets);
}

pub fn hide(mut commands: Commands, layout: Res<Layout>) {
    commands.entity(layout.ribbon).despawn_descendants();
}

impl ComputedStates for RibbonState {
    type SourceStates = UiState;

    fn compute(source: UiState) -> Option<Self> {
        match source {
            UiState::Startup | UiState::LoadingAssets => None,
            UiState::Menu | UiState::LoadingSave => Some(RibbonState::Hidden),
            UiState::Game => Some(RibbonState::Shown),
        }
    }
}

impl<'w> UiBuilder<'w, '_> {
    pub fn ribbon_root(&mut self) -> UiBuilder<'w, '_> {
        self.container(Node {
            width: Val::Percent(100.),
            position_type: PositionType::Absolute,
            margin: UiRect::new(Val::ZERO, Val::Auto, Val::Auto, Val::ZERO),
            display: Display::Flex,
            flex_direction: FlexDirection::ColumnReverse,
            align_items: AlignItems::Start,
            ..default()
        })
    }

    pub fn ribbon(&mut self, theme: &Theme, assets: &Assets) {
        let mut container = self.container(Node {
            display: Display::Flex,
            flex_direction: FlexDirection::Row,
            align_self: AlignSelf::Stretch,
            ..default()
        });

        container.ribbon_button(theme, assets, RibbonButton::Architect);
        container.ribbon_button(theme, assets, RibbonButton::Staff);
        container.ribbon_button(theme, assets, RibbonButton::Schedule);
        container.ribbon_button(theme, assets, RibbonButton::Manage);
    }

    fn ribbon_button(&mut self, theme: &Theme, assets: &Assets, button: RibbonButton) {
        self.text_button(
            theme,
            assets.ribbon_button_image.clone(),
            button.text(),
            theme.button_text.clone(),
            6.,
            Node {
                flex_grow: 1.,
                ..default()
            },
        )
        .insert(button)
        .on_click(
            move |mut trigger: Trigger<Pointer<Click>>,
                  commands: Commands,
                  theme: Res<Theme>,
                  assets: Res<Assets>,
                  layout: Res<Layout>,
                  panels: Query<(Entity, &RibbonPanel)>| {
                trigger.propagate(false);
                button.on_click(commands, theme, assets, layout, panels)
            },
        );
    }

    fn ribbon_panel(&mut self, theme: &Theme, assets: &Assets, kind: RibbonPanel) {
        let mut panel = match kind {
            RibbonPanel::Architect => self.ribbon_architect_panel(theme, assets),
            RibbonPanel::Staff => self.ribbon_staff_panel(theme, assets),
            RibbonPanel::Schedule => self.ribbon_schedule_panel(theme, assets),
            RibbonPanel::Manage => self.ribbon_manage_panel(theme, assets),
        };

        panel.cancellable().insert(kind);
    }

    fn ribbon_architect_panel(&mut self, theme: &Theme, assets: &Assets) -> UiBuilder<'w, '_> {
        let mut icon_grid = self.container(Node {
            padding: UiRect::new(theme.gutter, theme.gutter, Val::ZERO, theme.gutter),
            display: Display::Grid,
            grid_auto_columns: vec![GridTrack::max_content()],
            grid_auto_rows: vec![GridTrack::max_content()],
            row_gap: theme.gutter,
            column_gap: theme.gutter,
            align_items: AlignItems::Center,
            ..default()
        });

        icon_grid
            .tile_button(theme, "Wall", assets.ribbon_button_wall_image.clone())
            .on_click(wall::wall);

        icon_grid
    }

    fn ribbon_staff_panel(&mut self, theme: &Theme, _assets: &Assets) -> UiBuilder<'w, '_> {
        self.panel(theme, default())
    }

    fn ribbon_schedule_panel(&mut self, theme: &Theme, _assets: &Assets) -> UiBuilder<'w, '_> {
        self.panel(theme, default())
    }

    fn ribbon_manage_panel(&mut self, theme: &Theme, _assets: &Assets) -> UiBuilder<'w, '_> {
        self.panel(theme, default())
    }
}

impl RibbonButton {
    fn text(&self) -> &'static str {
        match self {
            RibbonButton::Architect => "Architect",
            RibbonButton::Staff => "Staff",
            RibbonButton::Schedule => "Schedule",
            RibbonButton::Manage => "Manage",
        }
    }

    fn panel(&self) -> RibbonPanel {
        match self {
            RibbonButton::Architect => RibbonPanel::Architect,
            RibbonButton::Staff => RibbonPanel::Staff,
            RibbonButton::Schedule => RibbonPanel::Schedule,
            RibbonButton::Manage => RibbonPanel::Manage,
        }
    }

    fn on_click(
        &self,
        mut commands: Commands,
        theme: Res<Theme>,
        assets: Res<Assets>,
        layout: Res<Layout>,
        panel_q: Query<(Entity, &RibbonPanel)>,
    ) {
        let requested_panel = self.panel();
        for (id, &panel) in &panel_q {
            commands.entity(id).despawn_recursive();
            if panel == requested_panel {
                return;
            }
        }

        UiBuilder::new(commands, layout.ribbon).ribbon_panel(&theme, &assets, requested_panel)
    }
}
