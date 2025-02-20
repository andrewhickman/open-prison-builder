use bevy::{prelude::*, render::view::NoFrustumCulling};

use pb_render::grid::{GridMaterial, GRID_MESH_HANDLE};

use crate::{input::picking::PickingState, theme::Theme};

#[derive(Clone, Copy, Debug, Component)]
pub struct Grid;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default, States)]
pub enum GridPickingState {
    #[default]
    Disabled,
    Enabled,
}

pub fn show(mut commands: Commands, theme: Res<Theme>, mut grids: ResMut<Assets<GridMaterial>>) {
    info!("show grid");

    let grid = grids.add(GridMaterial {
        color: theme.panel.with_alpha(0.38).into(),
    });

    commands.spawn((
        Grid,
        Visibility::default(),
        Mesh2d(GRID_MESH_HANDLE),
        MeshMaterial2d(grid),
        PickingBehavior::IGNORE,
        NoFrustumCulling,
    ));
}

pub fn hide(mut commands: Commands, grid_q: Query<Entity, With<Grid>>) {
    info!("hide grid");

    for grid in &grid_q {
        commands.entity(grid).despawn();
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
