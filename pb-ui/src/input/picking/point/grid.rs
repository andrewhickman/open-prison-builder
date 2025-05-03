use bevy::{prelude::*, render::view::NoFrustumCulling};

use pb_render::grid::{GRID_MESH_HANDLE, GridMaterial};

use crate::{
    input::{GridInput, picking::POINT_PICKING_THRESHOLD},
    theme::Theme,
};

#[derive(Default, Clone, Copy, Debug, Component)]
#[require(
    Visibility,
    Mesh2d = Mesh2d(GRID_MESH_HANDLE),
    Pickable = Pickable::IGNORE,
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
        .entity(trigger.target())
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

    pub fn point_mark(&self, point: Vec2, scale: f32) -> Option<Vec2> {
        match (self.mark(point.x, scale), self.mark(point.y, scale)) {
            (None, None) => None,
            (x, y) => Some(Vec2::new(x.unwrap_or(point.x), y.unwrap_or(point.y))),
        }
    }

    pub fn line_mark(&self, origin: Vec2, dir: Vec2, t: f32, scale: f32) -> Option<f32> {
        match (
            self.line_mark_t(origin.x, dir.x, t, scale),
            self.line_mark_t(origin.y, dir.y, t, scale),
        ) {
            (None, None) => None,
            (Some(t), None) | (None, Some(t)) => Some(t),
            (Some(t1), Some(t2)) => {
                if (t1 - t).abs() < (t2 - t).abs() {
                    Some(t1)
                } else {
                    Some(t2)
                }
            }
        }
    }

    fn line_mark_t(&self, origin: f32, dir: f32, t: f32, scale: f32) -> Option<f32> {
        let dir_recip = dir.recip();
        if !dir_recip.is_finite() {
            return None;
        }

        self.mark(origin + t * dir, scale)
            .map(|p| (p - origin) * dir_recip)
    }
}
