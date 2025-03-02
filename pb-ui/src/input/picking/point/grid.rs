use bevy::{prelude::*, render::view::NoFrustumCulling};

use pb_render::grid::{GridMaterial, GRID_MESH_HANDLE};

use crate::{
    input::{picking::PickingState, GridInput},
    theme::Theme,
};

#[derive(Default, Clone, Copy, Debug, Component)]
pub struct Grid {
    level: i32,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default, States)]
pub enum GridPickingState {
    #[default]
    Disabled,
    Enabled,
}

pub fn show(mut commands: Commands, theme: Res<Theme>, mut grids: ResMut<Assets<GridMaterial>>) {
    let grid = grids.add(GridMaterial::new(theme.panel.with_alpha(0.38).into()));

    commands.spawn((
        Grid::default(),
        Visibility::default(),
        Mesh2d(GRID_MESH_HANDLE),
        MeshMaterial2d(grid),
        PickingBehavior::IGNORE,
        NoFrustumCulling,
    ));
}

pub fn hide(mut commands: Commands, grid_q: Query<Entity, With<Grid>>) {
    for grid in &grid_q {
        commands.entity(grid).despawn_recursive();
    }
}

pub fn input(
    trigger: Trigger<GridInput>,
    mut assets: ResMut<Assets<GridMaterial>>,
    mut grid_q: Query<(&mut Grid, &MeshMaterial2d<GridMaterial>)>,
) {
    for (mut grid, material) in &mut grid_q {
        let Some(material) = assets.get_mut(material.id()) else {
            continue;
        };

        match trigger.event() {
            GridInput::DecreaseSize => grid.decrease_size(),
            GridInput::IncreaseSize => grid.increase_size(),
        };

        material.set_level(grid.level());
    }
}

pub fn update_state_condition(picking_state: Res<PickingState>) -> bool {
    resource_changed(picking_state)
}

pub fn update_state(
    picking_state: Res<PickingState>,
    mut next_state: ResMut<NextState<GridPickingState>>,
) {
    if picking_state.grid_enabled() {
        next_state.set(GridPickingState::Enabled);
    } else {
        next_state.set(GridPickingState::Disabled);
    }
}

impl Grid {
    pub fn increase_size(&mut self) {
        self.level = (self.level - 1).max(-4)
    }

    pub fn decrease_size(&mut self) {
        self.level = (self.level + 1).min(4)
    }

    pub fn level(&self) -> f32 {
        2f32.powi(self.level)
    }

    pub fn mark(&self, p: f32, scale: f32) -> Option<f32> {
        let level = self.level();
        let mark = (p / level).round() * level;
        if (p - mark).abs() < (20. * scale) {
            Some(mark)
        } else {
            None
        }
    }
}
