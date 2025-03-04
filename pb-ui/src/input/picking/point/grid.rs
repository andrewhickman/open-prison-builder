use bevy::{prelude::*, render::view::NoFrustumCulling};

use pb_render::grid::{GridMaterial, GRID_MESH_HANDLE};

use crate::{
    input::{picking::POINT_PICKING_THRESHOLD, GridInput},
    theme::Theme,
};

#[derive(Default, Clone, Copy, Debug, Component)]
#[require(
    Visibility,
    Mesh2d(|| Mesh2d(GRID_MESH_HANDLE)),
    PickingBehavior(|| PickingBehavior::IGNORE),
    NoFrustumCulling,
)]
pub struct Grid {
    level: i32,
}

pub fn on_add(
    trigger: Trigger<OnAdd, Grid>,
    mut commands: Commands,
    theme: Res<Theme>,
    mut grids: ResMut<Assets<GridMaterial>>,
) {
    let grid = grids.add(GridMaterial::new(theme.panel.with_alpha(0.38).into()));

    commands
        .entity(trigger.entity())
        .insert(MeshMaterial2d(grid));
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
        if (p - mark).abs() < (POINT_PICKING_THRESHOLD * scale) {
            Some(mark)
        } else {
            None
        }
    }
}
