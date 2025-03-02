use bevy::{
    picking::backend::{ray::RayMap, HitData, PointerHits},
    prelude::*,
    render::view::NoFrustumCulling,
};

use pb_engine::EngineState;
use pb_render::grid::{GridMaterial, GRID_MESH_HANDLE};

use crate::{
    input::{picking::PickingState, GridInput},
    theme::Theme,
};

#[derive(Default, Clone, Copy, Debug, Component)]
pub struct Grid {
    level: i32,
}

#[derive(Default, Clone, Copy, Debug, Component)]
pub struct GridPickingTarget;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default, States)]
pub enum GridPickingState {
    #[default]
    Disabled,
    Enabled,
}

pub fn show(mut commands: Commands, theme: Res<Theme>, mut grids: ResMut<Assets<GridMaterial>>) {
    let grid = grids.add(GridMaterial::new(theme.panel.with_alpha(0.38).into()));

    commands
        .spawn((
            Grid::default(),
            Visibility::default(),
            Mesh2d(GRID_MESH_HANDLE),
            MeshMaterial2d(grid),
            PickingBehavior::IGNORE,
            NoFrustumCulling,
        ))
        .with_children(|builder| {
            builder
                .spawn(GridPickingTarget)
                .observe(over)
                .observe(moved)
                .observe(out)
                .observe(click);
        });
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

pub fn backend(
    ray_map: Res<RayMap>,
    camera_q: Query<(&Camera, &OrthographicProjection)>,
    grid: Single<&Grid>,
    target: Single<Entity, With<GridPickingTarget>>,
    mut hits: EventWriter<PointerHits>,
) {
    let grid = *grid;
    let target = *target;

    for (&ray_id, &ray) in ray_map.iter() {
        if let Ok((camera, projection)) = camera_q.get(ray_id.camera) {
            let mark_x = grid.mark(ray.origin.x, projection.scale);
            let mark_y = grid.mark(ray.origin.y, projection.scale);

            if mark_x.is_none() && mark_y.is_none() {
                return;
            }

            let pos = Vec3::new(
                mark_x.unwrap_or(ray.origin.x),
                mark_y.unwrap_or(ray.origin.y),
                0.,
            );
            let picks = vec![(target, HitData::new(ray_id.camera, 0., Some(pos), None))];
            hits.send(PointerHits::new(ray_id.pointer, picks, camera.order as f32));
        }
    }
}

pub fn over(
    mut trigger: Trigger<Pointer<Over>>,
    mut state: ResMut<PickingState>,
    mut commands: Commands,
) {
    trigger.propagate(false);

    state.vertex_over(&mut commands, trigger.event());
}

pub fn moved(
    mut trigger: Trigger<Pointer<Move>>,
    mut state: ResMut<PickingState>,
    mut commands: Commands,
) {
    trigger.propagate(false);

    state.vertex_move(&mut commands, trigger.event());
}

pub fn out(
    mut trigger: Trigger<Pointer<Out>>,
    mut state: ResMut<PickingState>,
    mut commands: Commands,
) {
    trigger.propagate(false);

    state.vertex_out(&mut commands, trigger.event());
}

pub fn click(
    mut trigger: Trigger<Pointer<Click>>,
    mut state: ResMut<PickingState>,
    engine_state: Res<State<EngineState>>,
    mut commands: Commands,
) {
    trigger.propagate(false);

    state.vertex_click(&mut commands, &engine_state, trigger.event());
}

impl Grid {
    fn increase_size(&mut self) {
        self.level = (self.level - 1).max(-4)
    }

    fn decrease_size(&mut self) {
        self.level = (self.level + 1).min(4)
    }

    fn level(&self) -> f32 {
        2f32.powi(self.level)
    }
    fn mark(&self, p: f32, scale: f32) -> Option<f32> {
        let level = self.level();
        let mark = (p / level).round() * level;
        if (p - mark).abs() < (20. * scale) {
            Some(mark)
        } else {
            None
        }
    }
}
