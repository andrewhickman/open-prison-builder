use bevy::prelude::*;
use pb_engine::{
    wall::{VertexBundle, Wall},
    EngineState,
};
use pb_render::Preview;

#[derive(Debug)]
pub enum CreateWallState {
    SelectStart,
    PreviewStart {
        start: Entity,
    },
    SelectEnd {
        start: Entity,
    },
    PreviewEnd {
        start: Entity,
        wall: Entity,
        end: Entity,
    },
    Complete,
}

impl CreateWallState {
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
                let start = commands.spawn((VertexBundle::new(pos), Preview)).id();

                *self = CreateWallState::PreviewStart { start };
            }
            CreateWallState::PreviewStart { start } => {
                commands.entity(start).queue(set_vertex_position(pos));
            }
            CreateWallState::SelectEnd { start } => {
                let end = commands.spawn((VertexBundle::new(pos), Preview)).id();
                let wall = commands.spawn((Wall::new(start, end), Preview)).id();

                *self = CreateWallState::PreviewEnd { start, end, wall };
            }
            CreateWallState::PreviewEnd { end, .. } => {
                commands.entity(end).queue(set_vertex_position(pos));
            }
            CreateWallState::Complete => {}
        }
    }

    pub fn vertex_out(&mut self, commands: &mut Commands, _: &Pointer<Out>) {
        match *self {
            CreateWallState::SelectStart => {}
            CreateWallState::PreviewStart { start } => {
                commands.entity(start).despawn();

                *self = CreateWallState::SelectStart;
            }
            CreateWallState::SelectEnd { .. } => {}
            CreateWallState::PreviewEnd { start, wall, end } => {
                commands.entity(wall).despawn();
                commands.entity(end).despawn();

                *self = CreateWallState::SelectEnd { start };
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
            CreateWallState::PreviewStart { start } => {
                *self = CreateWallState::SelectEnd { start };
            }
            CreateWallState::SelectEnd { .. } => {}
            CreateWallState::PreviewEnd { start, wall, end } => {
                commands.entity(start).set_parent(root).remove::<Preview>();
                commands.entity(wall).set_parent(root).remove::<Preview>();
                commands.entity(end).set_parent(root).remove::<Preview>();

                *self = CreateWallState::Complete;
            }
            CreateWallState::Complete => {}
        }
    }

    pub fn cancel(self, commands: &mut Commands) {
        match self {
            CreateWallState::SelectStart => {}
            CreateWallState::PreviewStart { start } | CreateWallState::SelectEnd { start } => {
                commands.entity(start).despawn()
            }
            CreateWallState::PreviewEnd { start, wall, end } => {
                commands.entity(start).despawn();
                commands.entity(wall).despawn();
                commands.entity(end).despawn();
            }
            CreateWallState::Complete => {}
        }
    }
}

fn set_vertex_position(pos: Vec2) -> impl EntityCommand<World> {
    move |mut world: EntityWorldMut| {
        if let Some(mut transform) = world.get_mut::<Transform>() {
            transform.translation = pos.extend(0.);
        } else {
            warn!("vertex not found");
        }
    }
}
