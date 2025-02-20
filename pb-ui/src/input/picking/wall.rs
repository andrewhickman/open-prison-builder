use bevy::prelude::*;
use pb_engine::{
    build::Blueprint,
    wall::{VertexBundle, WallBundle},
    EngineState,
};
use pb_util::try_modify_component;

#[derive(Debug)]
pub enum CreateWallState {
    SelectStart,
    PreviewStart {
        start: Entity,
        start_pos: Vec2,
    },
    SelectEnd {
        start: Entity,
        start_pos: Vec2,
    },
    PreviewEnd {
        start: Entity,
        start_pos: Vec2,
        wall: Entity,
        end: Entity,
        end_pos: Vec2,
    },
    Complete,
}

impl CreateWallState {
    pub fn grid_enabled(&self) -> bool {
        true
    }

    pub fn vertex_over(&mut self, commands: &mut Commands, event: &Pointer<Over>) {
        if let Some(position) = event.hit.position {
            self.set_position(commands, position.xy());
        }
    }

    pub fn vertex_move(&mut self, commands: &mut Commands, event: &Pointer<Move>) {
        if let Some(position) = event.hit.position {
            self.set_position(commands, position.xy());
        }
    }

    fn set_position(&mut self, commands: &mut Commands, pos: Vec2) {
        match *self {
            CreateWallState::SelectStart => {
                let start = commands.spawn((VertexBundle::new(pos), Blueprint)).id();

                *self = CreateWallState::PreviewStart {
                    start,
                    start_pos: pos,
                };
            }
            CreateWallState::PreviewStart {
                start,
                ref mut start_pos,
            } => {
                commands.queue(try_modify_component(
                    start,
                    move |mut transform: Mut<Transform>| transform.translation = pos.extend(0.),
                ));

                *start_pos = pos;
            }
            CreateWallState::SelectEnd { start, start_pos } => {
                let end = commands.spawn((VertexBundle::new(pos), Blueprint)).id();
                let wall = commands
                    .spawn((WallBundle::new(start, start_pos, end, pos), Blueprint))
                    .id();

                *self = CreateWallState::PreviewEnd {
                    start,
                    start_pos,
                    end,
                    wall,
                    end_pos: pos,
                };
            }
            CreateWallState::PreviewEnd {
                start_pos,
                end,
                ref mut end_pos,
                wall,
                ..
            } => {
                commands.queue(set_pos(wall, start_pos.midpoint(pos)));
                commands.queue(set_pos(end, pos));

                *end_pos = pos;
            }
            CreateWallState::Complete => {}
        }
    }

    pub fn vertex_out(&mut self, commands: &mut Commands, _: &Pointer<Out>) {
        match *self {
            CreateWallState::SelectStart => {}
            CreateWallState::PreviewStart { start, .. } => {
                commands.entity(start).despawn();

                *self = CreateWallState::SelectStart;
            }
            CreateWallState::SelectEnd { .. } => {}
            CreateWallState::PreviewEnd {
                start,
                start_pos,
                wall,
                end,
                ..
            } => {
                commands.entity(wall).despawn();
                commands.entity(end).despawn();

                *self = CreateWallState::SelectEnd { start, start_pos };
            }
            CreateWallState::Complete => {}
        }
    }

    pub fn vertex_click(
        &mut self,
        commands: &mut Commands,
        state: &EngineState,
        event: &Pointer<Click>,
    ) {
        if let Some(position) = event.hit.position {
            self.set_position(commands, position.xy());
        }

        let &EngineState::Running(root) = state else {
            warn!("engine not running");
            return;
        };

        match *self {
            CreateWallState::SelectStart => {}
            CreateWallState::PreviewStart { start, start_pos } => {
                *self = CreateWallState::SelectEnd { start, start_pos };
            }
            CreateWallState::SelectEnd { .. } => {}
            CreateWallState::PreviewEnd {
                start, wall, end, ..
            } => {
                commands
                    .entity(start)
                    .set_parent(root)
                    .remove::<Blueprint>();
                commands.entity(wall).set_parent(root).remove::<Blueprint>();
                commands.entity(end).set_parent(root).remove::<Blueprint>();

                *self = CreateWallState::Complete;
            }
            CreateWallState::Complete => {}
        }
    }

    pub fn cancel(self, commands: &mut Commands) {
        match self {
            CreateWallState::SelectStart => {}
            CreateWallState::PreviewStart { start, .. }
            | CreateWallState::SelectEnd { start, .. } => commands.entity(start).despawn(),
            CreateWallState::PreviewEnd {
                start, wall, end, ..
            } => {
                commands.entity(start).despawn();
                commands.entity(wall).despawn();
                commands.entity(end).despawn();
            }
            CreateWallState::Complete => {}
        }
    }
}

fn set_pos(id: Entity, pos: Vec2) -> impl Command {
    try_modify_component(id, move |mut transform: Mut<Transform>| {
        transform.translation = pos.extend(0.);
    })
}
