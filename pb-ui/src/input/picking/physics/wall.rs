use bevy::prelude::*;
use pb_engine::wall::{self, Wall};
use pb_util::{try_opt, try_res_s};

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
    Vertex {
        vertex: Entity,
        position: Vec2,
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
        .entity(trigger.entity())
        .observe(over)
        .observe(moved)
        .observe(out)
        .observe(click);
}

fn over(
    mut trigger: Trigger<Pointer<Over>>,
    mut commands: Commands,
    wall_q: Query<&Wall>,
    vertex_q: Query<&Transform>,
) {
    trigger.propagate(false);

    let pos = try_opt!(trigger.event().hit.position).xy();
    let wall = try_res_s!(wall_q.get(trigger.entity()));
    let [start, end] = vertex_q.many(wall.vertices());

    commands.trigger(SelectWall {
        kind: WallPickKind::new(
            pos,
            trigger.entity(),
            wall.start(),
            start.translation.xy(),
            wall.end(),
            end.translation.xy(),
        ),
    });
}

fn moved(
    mut trigger: Trigger<Pointer<Move>>,
    mut commands: Commands,
    wall_q: Query<&Wall>,
    vertex_q: Query<&Transform>,
) {
    trigger.propagate(false);

    let pos = try_opt!(trigger.event().hit.position).xy();
    let wall = try_res_s!(wall_q.get(trigger.entity()));
    let [start, end] = vertex_q.many(wall.vertices());

    commands.trigger(SelectWall {
        kind: WallPickKind::new(
            pos,
            trigger.entity(),
            wall.start(),
            start.translation.xy(),
            wall.end(),
            end.translation.xy(),
        ),
    });
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
) {
    trigger.propagate(false);

    let pos = try_opt!(trigger.event().hit.position).xy();
    let wall = try_res_s!(wall_q.get(trigger.entity()));
    let [start, end] = vertex_q.many(wall.vertices());

    commands.trigger(ClickWall {
        kind: WallPickKind::new(
            pos,
            trigger.entity(),
            wall.start(),
            start.translation.xy(),
            wall.end(),
            end.translation.xy(),
        ),
    });
}

impl WallPickKind {
    pub fn new(
        hit_pos: Vec2,
        wall: Entity,
        start: Entity,
        start_pos: Vec2,
        end: Entity,
        end_pos: Vec2,
    ) -> Self {
        let hit_dir = hit_pos - start_pos;
        let wall_dir = end_pos - start_pos;

        let t = hit_dir.dot(wall_dir) / wall_dir.length_squared();

        if t < 0.0 || (t < 0.5 && (t * wall_dir).length_squared() < (wall::RADIUS * wall::RADIUS)) {
            WallPickKind::Vertex {
                vertex: start,
                position: start_pos,
            }
        } else if t > 1.0
            || (((1. - t) * wall_dir).length_squared() < (wall::RADIUS * wall::RADIUS))
        {
            WallPickKind::Vertex {
                vertex: end,
                position: end_pos,
            }
        } else {
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
