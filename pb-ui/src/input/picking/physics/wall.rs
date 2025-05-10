use bevy::prelude::*;
use pb_engine::map::{Wall, wall};
use pb_render::projection::ProjectionExt;

use crate::input::picking::point::grid::Grid;

#[derive(Event, Debug, Clone, Copy)]
pub struct SelectWall {
    pub kind: WallPickKind,
}

#[derive(Event, Debug, Clone, Copy)]
pub struct CancelWall;

#[derive(Event, Debug, Clone, Copy)]
pub struct ClickWall {
    pub kind: WallPickKind,
}

#[derive(Event, Debug, Clone, Copy)]
pub enum WallPickKind {
    Corner {
        corner: Entity,
    },
    Wall {
        wall: Entity,
        position: Vec2,
        start: Entity,
        start_position: Vec2,
        end: Entity,
        end_position: Vec2,
    },
}

pub fn wall_added(trigger: Trigger<OnAdd, Wall>, mut commands: Commands) {
    commands
        .entity(trigger.target())
        .observe(over)
        .observe(moved)
        .observe(out)
        .observe(click);
}

fn over(
    mut trigger: Trigger<Pointer<Over>>,
    mut commands: Commands,
    wall_q: Query<&Wall>,
    corner_q: Query<&Transform>,
    grid_q: Query<&Grid>,
    projection_q: Query<&Projection>,
) -> Result {
    trigger.propagate(false);

    let Some(pos) = trigger.event().hit.position else {
        return Ok(());
    };
    let wall = wall_q.get(trigger.target())?;
    let [start, end] = corner_q.get_many(wall.corners())?;

    commands.trigger(SelectWall {
        kind: WallPickKind::new(
            pos.xy(),
            trigger.target(),
            wall.corners()[0],
            start.translation.xy(),
            wall.corners()[1],
            end.translation.xy(),
            &grid_q,
            projection_q.get(trigger.event().hit.camera)?.scale(),
        ),
    });
    Ok(())
}

fn moved(
    mut trigger: Trigger<Pointer<Move>>,
    mut commands: Commands,
    wall_q: Query<&Wall>,
    vertex_q: Query<&Transform>,
    grid_q: Query<&Grid>,
    projection_q: Query<&Projection>,
) -> Result {
    trigger.propagate(false);

    let Some(pos) = trigger.event().hit.position else {
        return Ok(());
    };
    let wall = wall_q.get(trigger.target())?;
    let [start, end] = vertex_q.get_many(wall.corners())?;

    commands.trigger(SelectWall {
        kind: WallPickKind::new(
            pos.xy(),
            trigger.target(),
            wall.corners()[0],
            start.translation.xy(),
            wall.corners()[1],
            end.translation.xy(),
            &grid_q,
            projection_q.get(trigger.event().hit.camera)?.scale(),
        ),
    });
    Ok(())
}

fn out(mut trigger: Trigger<Pointer<Out>>, mut commands: Commands) {
    trigger.propagate(false);

    commands.trigger(CancelWall);
}

fn click(
    mut trigger: Trigger<Pointer<Click>>,
    mut commands: Commands,
    wall_q: Query<&Wall>,
    vertex_q: Query<&Transform>,
    grid_q: Query<&Grid>,
    projection_q: Query<&Projection>,
) -> Result {
    trigger.propagate(false);

    let Some(pos) = trigger.event().hit.position else {
        return Ok(());
    };
    let wall = wall_q.get(trigger.target())?;
    let [start, end] = vertex_q.get_many(wall.corners())?;

    commands.trigger(ClickWall {
        kind: WallPickKind::new(
            pos.xy(),
            trigger.target(),
            wall.corners()[0],
            start.translation.xy(),
            wall.corners()[1],
            end.translation.xy(),
            &grid_q,
            projection_q.get(trigger.event().hit.camera)?.scale(),
        ),
    });
    Ok(())
}

impl WallPickKind {
    pub fn new(
        hit_pos: Vec2,
        wall: Entity,
        start: Entity,
        start_pos: Vec2,
        end: Entity,
        end_pos: Vec2,
        grid_q: &Query<&Grid>,
        scale: f32,
    ) -> Self {
        let hit_dir = hit_pos - start_pos;
        let wall_dir = end_pos - start_pos;

        let mut t = hit_dir.dot(wall_dir) / wall_dir.length_squared();

        if t < 0.0 || (t < 0.5 && (t * wall_dir).length_squared() < (wall::RADIUS * wall::RADIUS)) {
            WallPickKind::Corner { corner: start }
        } else if t > 1.0
            || (((1. - t) * wall_dir).length_squared() < (wall::RADIUS * wall::RADIUS))
        {
            WallPickKind::Corner { corner: end }
        } else {
            for grid in grid_q {
                if let Some(grid_t) = grid.line_mark(start_pos, wall_dir, t, scale) {
                    t = grid_t;
                    break;
                }
            }

            let closest = start_pos + t * wall_dir;

            WallPickKind::Wall {
                wall,
                position: closest,
                start,
                start_position: start_pos,
                end,
                end_position: end_pos,
            }
        }
    }
}
