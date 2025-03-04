use bevy::prelude::*;
use pb_engine::{
    build::Blueprint,
    wall::{VertexBundle, WallBundle},
    EngineState,
};
use pb_util::{try_modify_component, ChildBuildExt};

use crate::input::{
    action::InputAction,
    picking::{
        physics::{
            wall::{CancelWall, ClickWall, SelectWall, WallPickKind},
            PhysicsPickingState,
        },
        point::{grid::Grid, CancelPoint, ClickPoint, SelectPoint},
    },
};

pub fn wall(_: Trigger<Pointer<Click>>, mut commands: Commands) {
    commands
        .spawn(WallAction::default())
        .with_children(|builder| {
            builder.spawn(Grid::default());

            builder
                .add_observer(select_point)
                .add_observer(cancel_point)
                .add_observer(click_point)
                .add_observer(select_wall)
                .add_observer(cancel_wall)
                .add_observer(click_wall);
        });
}

#[derive(Default, Debug, Component)]
#[require(InputAction, PhysicsPickingState(|| PhysicsPickingState::Wall), Transform, Visibility)]
pub enum WallAction {
    #[default]
    SelectStart,
    PreviewStart {
        start: SelectedVertex,
    },
    SelectEnd {
        start: SelectedVertex,
    },
    PreviewEnd {
        start: SelectedVertex,
        end: SelectedVertex,
        wall: SelectedWall,
    },
}

#[derive(Debug, Copy, Clone)]
pub struct SelectedVertex {
    id: Entity,
    pos: Vec2,
    spawned: bool,
}

#[derive(Debug, Copy, Clone)]
pub struct SelectedWall {
    id: Entity,
}

fn select_point(
    trigger: Trigger<SelectPoint>,
    mut commands: Commands,
    mut action: Single<(Entity, &mut WallAction)>,
) {
    let (id, ref mut action) = *action;
    action.select_point(id, &mut commands, trigger.event().point);
}

fn cancel_point(
    _: Trigger<CancelPoint>,
    mut commands: Commands,
    mut action: Single<(Entity, &mut WallAction)>,
) {
    let (_, ref mut action) = *action;
    action.cancel(&mut commands);
}

fn click_point(
    trigger: Trigger<ClickPoint>,
    mut commands: Commands,
    mut action: Single<(Entity, &mut WallAction)>,
    engine_state: Res<State<EngineState>>,
) {
    let (id, ref mut action) = *action;
    action.select_point(id, &mut commands, trigger.event().point);
    action.click(&mut commands, &engine_state)
}

fn select_wall(
    trigger: Trigger<SelectWall>,
    mut commands: Commands,
    mut action: Single<(Entity, &mut WallAction)>,
) {
    let (id, ref mut action) = *action;
    match trigger.event().kind {
        WallPickKind::Vertex { vertex, position } => {
            action.select_vertex(id, &mut commands, vertex, position)
        }
        WallPickKind::Wall { wall: _, position } => {
            action.select_point(id, &mut commands, position)
        }
    }
}

fn cancel_wall(
    _: Trigger<CancelWall>,
    mut commands: Commands,
    mut action: Single<(Entity, &mut WallAction)>,
) {
    let (_, ref mut action) = *action;
    action.cancel(&mut commands);
}

fn click_wall(
    trigger: Trigger<ClickWall>,
    mut commands: Commands,
    mut action: Single<(Entity, &mut WallAction)>,
    engine_state: Res<State<EngineState>>,
) {
    let (id, ref mut action) = *action;
    match trigger.event().kind {
        WallPickKind::Vertex { vertex, position } => {
            action.select_vertex(id, &mut commands, vertex, position);
            action.click(&mut commands, &engine_state)
        }
        WallPickKind::Wall { wall: _, position } => {
            action.select_point(id, &mut commands, position);
            action.click(&mut commands, &engine_state)
        }
    }
}

impl WallAction {
    pub fn select_point(&mut self, this: Entity, commands: &mut Commands, point: Vec2) {
        match *self {
            WallAction::SelectStart => {
                *self = WallAction::PreviewStart {
                    start: SelectedVertex::spawn(commands, this, point),
                };
            }
            WallAction::PreviewStart { ref mut start } => {
                start.update_spawned(commands, this, point);
            }
            WallAction::SelectEnd { start } => {
                let end = SelectedVertex::spawn(commands, this, point);
                let wall = SelectedWall::spawn(commands, this, start, end);

                *self = WallAction::PreviewEnd { start, end, wall };
            }
            WallAction::PreviewEnd {
                start,
                ref mut end,
                ref mut wall,
                ..
            } => {
                if end.update_spawned(commands, this, point) {
                    wall.replace(commands, this, start, *end);
                } else {
                    wall.update(commands, start, *end);
                }
            }
        }
    }

    pub fn select_vertex(
        &mut self,
        this: Entity,
        commands: &mut Commands,
        vertex: Entity,
        pos: Vec2,
    ) {
        match *self {
            WallAction::SelectStart => {
                *self = WallAction::PreviewStart {
                    start: SelectedVertex::existing(vertex, pos),
                };
            }
            WallAction::PreviewStart { ref mut start } => {
                start.update_existing(commands, vertex, pos);
            }
            WallAction::SelectEnd { start } => {
                let end = SelectedVertex::existing(vertex, pos);
                let wall = SelectedWall::spawn(commands, this, start, end);

                *self = WallAction::PreviewEnd { start, end, wall };
            }
            WallAction::PreviewEnd {
                start,
                ref mut end,
                ref mut wall,
                ..
            } => {
                if end.update_existing(commands, vertex, pos) {
                    wall.replace(commands, this, start, *end);
                }
            }
        }
    }

    pub fn cancel(&mut self, commands: &mut Commands) {
        match *self {
            WallAction::SelectStart => {}
            WallAction::PreviewStart { start } => {
                start.despawn(commands);
                *self = WallAction::SelectStart;
            }
            WallAction::SelectEnd { .. } => {}
            WallAction::PreviewEnd {
                start, wall, end, ..
            } => {
                wall.despawn(commands);
                end.despawn(commands);

                *self = WallAction::SelectEnd { start };
            }
        }
    }

    pub fn click(&mut self, commands: &mut Commands, state: &EngineState) {
        let &EngineState::Running(root) = state else {
            warn!("engine not running");
            return;
        };

        match *self {
            WallAction::SelectStart => {}
            WallAction::PreviewStart { start } => {
                *self = WallAction::SelectEnd { start };
            }
            WallAction::SelectEnd { .. } => {}
            WallAction::PreviewEnd {
                mut start,
                wall,
                mut end,
            } => {
                start.commit(commands, root);
                wall.commit(commands, root);
                end.commit(commands, root);

                *self = WallAction::SelectEnd { start: end };
            }
        }
    }
}

impl SelectedVertex {
    pub fn spawn(commands: &mut Commands, parent: Entity, pos: Vec2) -> Self {
        let id = commands
            .spawn((VertexBundle::new(pos), Blueprint))
            .set_parent(parent)
            .id();

        SelectedVertex {
            id,
            pos,
            spawned: true,
        }
    }

    pub fn existing(id: Entity, pos: Vec2) -> Self {
        SelectedVertex {
            id,
            pos,
            spawned: false,
        }
    }

    pub fn update_spawned(&mut self, commands: &mut Commands, parent: Entity, pos: Vec2) -> bool {
        if self.spawned {
            self.pos = pos;
            commands.queue(try_modify_component(
                self.id,
                move |mut transform: Mut<Transform>| {
                    transform.translation = pos.extend(0.);
                },
            ));
            false
        } else {
            *self = SelectedVertex::spawn(commands, parent, pos);
            true
        }
    }

    pub fn update_existing(&mut self, commands: &mut Commands, id: Entity, pos: Vec2) -> bool {
        if self.id != id {
            self.despawn(commands);
            *self = SelectedVertex::existing(id, pos);
            true
        } else {
            false
        }
    }

    pub fn despawn(&self, commands: &mut Commands) {
        if self.spawned {
            commands.entity(self.id).remove_parent().despawn();
        }
    }

    pub fn commit(&mut self, commands: &mut Commands, root: Entity) {
        if self.spawned {
            commands
                .entity(self.id)
                .set_parent_in_place(root)
                .remove::<Blueprint>();
            self.spawned = false;
        }
    }
}

impl SelectedWall {
    pub fn spawn(
        commands: &mut Commands,
        parent: Entity,
        start: SelectedVertex,
        end: SelectedVertex,
    ) -> Self {
        let id = commands
            .spawn((
                WallBundle::new(start.id, start.pos, end.id, end.pos),
                Blueprint,
            ))
            .set_parent(parent)
            .id();

        SelectedWall { id }
    }

    pub fn replace(
        &mut self,
        commands: &mut Commands,
        parent: Entity,
        start: SelectedVertex,
        end: SelectedVertex,
    ) {
        self.despawn(commands);
        *self = SelectedWall::spawn(commands, parent, start, end);
    }

    pub fn update(&mut self, commands: &mut Commands, start: SelectedVertex, end: SelectedVertex) {
        commands.queue(try_modify_component(
            self.id,
            move |mut transform: Mut<Transform>| {
                transform.translation = start.pos.midpoint(end.pos).extend(0.);
            },
        ));
    }

    pub fn despawn(&self, commands: &mut Commands) {
        commands.entity(self.id).remove_parent().despawn();
    }

    pub fn commit(&self, commands: &mut Commands, root: Entity) {
        commands
            .entity(self.id)
            .set_parent(root)
            .remove::<Blueprint>();
    }
}
