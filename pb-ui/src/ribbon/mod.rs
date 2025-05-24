pub mod architect;
pub mod dev_tools;

use bevy::prelude::*;

use pb_assets::AssetHandles;

use crate::{UiState, layout::Layout, theme::Theme, widget::UiBuilder};

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
    DevTools,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Component)]
pub enum RibbonPanel {
    Architect,
    Staff,
    Schedule,
    Manage,
    DevTools,
}

pub fn show(commands: Commands, layout: Res<Layout>, theme: Res<Theme>, assets: Res<AssetHandles>) {
    UiBuilder::new(commands, layout.ribbon).ribbon(&theme, &assets);
}

pub fn hide(mut commands: Commands, layout: Res<Layout>) {
    commands.entity(layout.ribbon).despawn_related::<Children>();
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

    pub fn ribbon(&mut self, theme: &Theme, assets: &AssetHandles) {
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
        container.ribbon_button(theme, assets, RibbonButton::DevTools);
    }

    fn ribbon_button(&mut self, theme: &Theme, assets: &AssetHandles, button: RibbonButton) {
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
            move |_: Trigger<Pointer<Click>>,
                  commands: Commands,
                  theme: Res<Theme>,
                  assets: Res<AssetHandles>,
                  layout: Res<Layout>,
                  panels: Query<(Entity, &RibbonPanel)>| {
                button.on_click(commands, theme, assets, layout, panels)
            },
        );
    }

    fn ribbon_panel(&mut self, theme: &Theme, assets: &AssetHandles, kind: RibbonPanel) {
        let mut panel = match kind {
            RibbonPanel::Architect => self.ribbon_architect_panel(theme, assets),
            RibbonPanel::Staff => self.ribbon_staff_panel(theme, assets),
            RibbonPanel::Schedule => self.ribbon_schedule_panel(theme, assets),
            RibbonPanel::Manage => self.ribbon_manage_panel(theme, assets),
            RibbonPanel::DevTools => self.ribbon_dev_tools_panel(theme, assets),
        };

        panel.cancellable().insert(kind);
    }

    fn ribbon_architect_panel(
        &mut self,
        theme: &Theme,
        assets: &AssetHandles,
    ) -> UiBuilder<'w, '_> {
        let mut icon_grid = self.container(Node {
            padding: UiRect::new(theme.gutter, theme.gutter, Val::ZERO, theme.gutter),
            display: Display::Grid,
            grid_auto_flow: GridAutoFlow::Column,
            grid_auto_columns: vec![GridTrack::max_content()],
            grid_auto_rows: vec![GridTrack::max_content()],
            row_gap: theme.gutter,
            column_gap: theme.gutter,
            align_items: AlignItems::Center,
            ..default()
        });

        icon_grid
            .tile_button(theme, "Build wall", assets.ribbon_button_wall_image.clone())
            .on_click(architect::wall::add::add_wall);
        icon_grid
            .tile_button(
                theme,
                "Remove wall",
                assets.ribbon_button_delete_wall_image.clone(),
            )
            .on_click(architect::wall::remove::remove_wall);
        icon_grid
            .tile_button(theme, "Pawn", assets.pawn_image.clone())
            .on_click(architect::pawn::pawn);

        icon_grid
    }

    fn ribbon_staff_panel(&mut self, theme: &Theme, _assets: &AssetHandles) -> UiBuilder<'w, '_> {
        self.panel(theme, default())
    }

    fn ribbon_schedule_panel(
        &mut self,
        theme: &Theme,
        _assets: &AssetHandles,
    ) -> UiBuilder<'w, '_> {
        self.panel(theme, default())
    }

    fn ribbon_manage_panel(&mut self, theme: &Theme, _assets: &AssetHandles) -> UiBuilder<'w, '_> {
        self.panel(theme, default())
    }

    fn ribbon_dev_tools_panel(
        &mut self,
        theme: &Theme,
        assets: &AssetHandles,
    ) -> UiBuilder<'w, '_> {
        let mut icon_grid = self.container(Node {
            padding: UiRect::new(theme.gutter, theme.gutter, Val::ZERO, theme.gutter),
            display: Display::Grid,
            grid_auto_flow: GridAutoFlow::Column,
            grid_auto_columns: vec![GridTrack::max_content()],
            grid_auto_rows: vec![GridTrack::max_content()],
            row_gap: theme.gutter,
            column_gap: theme.gutter,
            align_items: AlignItems::Center,
            ..default()
        });

        icon_grid
            .tile_button(theme, "Draw Paths", assets.ribbon_button_wall_image.clone())
            .on_click(dev_tools::toggle_draw_paths);
        icon_grid
            .tile_button(
                theme,
                "Draw Meshes",
                assets.ribbon_button_wall_image.clone(),
            )
            .on_click(dev_tools::toggle_draw_meshes);
        icon_grid
            .tile_button(theme, "Spawn 1000 Pawns", assets.pawn_image.clone())
            .on_click(dev_tools::path_stress_test::spawn_1000_pawns);
        icon_grid
            .tile_button(theme, "Create path tasks", assets.pawn_image.clone())
            .on_click(dev_tools::path_stress_test::create_path_tasks);

        icon_grid
    }
}

impl RibbonButton {
    fn text(&self) -> &'static str {
        match self {
            RibbonButton::Architect => "Architect",
            RibbonButton::Staff => "Staff",
            RibbonButton::Schedule => "Schedule",
            RibbonButton::Manage => "Manage",
            RibbonButton::DevTools => "Dev Tools",
        }
    }

    fn panel(&self) -> RibbonPanel {
        match self {
            RibbonButton::Architect => RibbonPanel::Architect,
            RibbonButton::Staff => RibbonPanel::Staff,
            RibbonButton::Schedule => RibbonPanel::Schedule,
            RibbonButton::Manage => RibbonPanel::Manage,
            RibbonButton::DevTools => RibbonPanel::DevTools,
        }
    }

    fn on_click(
        &self,
        mut commands: Commands,
        theme: Res<Theme>,
        assets: Res<AssetHandles>,
        layout: Res<Layout>,
        panel_q: Query<(Entity, &RibbonPanel)>,
    ) -> Result {
        let requested_panel = self.panel();
        for (id, &panel) in &panel_q {
            commands.entity(id).despawn();
            if panel == requested_panel {
                return Ok(());
            }
        }

        UiBuilder::new(commands, layout.ribbon).ribbon_panel(&theme, &assets, requested_panel);
        Ok(())
    }
}
