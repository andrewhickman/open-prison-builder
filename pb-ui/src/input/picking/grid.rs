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
        commands.entity(grid).despawn();
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
            GridInput::DecreaseSize => grid.level -= 1,
            GridInput::IncreaseSize => grid.level += 1,
        };

        grid.level = grid.level.clamp(-4, 4);

        material.set_level(grid.level);
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
