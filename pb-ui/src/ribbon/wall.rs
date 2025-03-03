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
        collider::{
            wall::{CancelWall, ClickWall, SelectWall, WallPickKind},
            ColliderPickingState,
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
#[require(InputAction, ColliderPickingState(|| ColliderPickingState::Wall), Transform, Visibility)]
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
        wall: Entity,
    },
}

#[derive(Debug, Copy, Clone)]
pub struct SelectedVertex {
    id: Entity,
    pos: Vec2,
    created: bool,
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
    action.click_point(id, &mut commands, trigger.event().point, &engine_state);
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
            action.click_vertex(id, &mut commands, vertex, position, &engine_state)
        }
        WallPickKind::Wall { wall: _, position } => {
            action.click_point(id, &mut commands, position, &engine_state)
        }
    }
}

impl WallAction {
    pub fn select_point(&mut self, this: Entity, commands: &mut Commands, point: Vec2) {
        match *self {
            WallAction::SelectStart => {
                *self = WallAction::PreviewStart {
                    start: SelectedVertex::create(commands, this, point),
                };
            }
            WallAction::PreviewStart { ref mut start } => {
                start.set_pos(commands, this, point);
            }
            WallAction::SelectEnd { start } => {
                let end = SelectedVertex::create(commands, this, point);
                let wall = commands
                    .spawn((
                        WallBundle::new(start.id, start.pos, end.id, end.pos),
                        Blueprint,
                    ))
                    .set_parent(this)
                    .id();

                *self = WallAction::PreviewEnd { start, end, wall };
            }
            WallAction::PreviewEnd {
                start,
                ref mut end,
                wall,
                ..
            } => {
                end.set_pos(commands, this, point);
                commands.queue(set_pos(wall, start.pos.midpoint(end.pos)));
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
                start.replace(commands, vertex, pos);
            }
            WallAction::SelectEnd { start } => {
                let end = SelectedVertex::existing(vertex, pos);
                let wall = commands
                    .spawn((
                        WallBundle::new(start.id, start.pos, end.id, end.pos),
                        Blueprint,
                    ))
                    .set_parent(this)
                    .id();

                *self = WallAction::PreviewEnd { start, end, wall };
            }
            WallAction::PreviewEnd {
                start,
                ref mut end,
                wall,
                ..
            } => {
                end.replace(commands, vertex, pos);
                commands.queue(set_pos(wall, start.pos.midpoint(end.pos)));
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
                commands.entity(wall).remove_parent().despawn();
                end.despawn(commands);

                *self = WallAction::SelectEnd { start };
            }
        }
    }

    pub fn click_point(
        &mut self,
        this: Entity,
        commands: &mut Commands,
        point: Vec2,
        state: &EngineState,
    ) {
        let &EngineState::Running(root) = state else {
            warn!("engine not running");
            return;
        };

        self.select_point(this, commands, point);

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
                commands
                    .entity(wall)
                    .set_parent_in_place(root)
                    .remove::<Blueprint>();
                end.commit(commands, root);

                *self = WallAction::SelectEnd { start: end };
            }
        }
    }

    pub fn click_vertex(
        &mut self,
        _: Entity,
        _: &mut Commands,
        _: Entity,
        pos: Vec2,
        _: &EngineState,
    ) {
        info!("click_vertex: {pos:?}")
    }
}

impl SelectedVertex {
    pub fn create(commands: &mut Commands, parent: Entity, pos: Vec2) -> Self {
        let id = commands
            .spawn((VertexBundle::new(pos), Blueprint))
            .set_parent(parent)
            .id();

        SelectedVertex {
            id,
            pos,
            created: true,
        }
    }

    pub fn existing(id: Entity, pos: Vec2) -> Self {
        SelectedVertex {
            id,
            pos,
            created: false,
        }
    }

    pub fn set_pos(&mut self, commands: &mut Commands, parent: Entity, pos: Vec2) {
        if self.created {
            self.pos = pos;
            commands.queue(try_modify_component(
                self.id,
                move |mut transform: Mut<Transform>| {
                    transform.translation = pos.extend(0.);
                },
            ));
        } else {
            *self = SelectedVertex::create(commands, parent, pos);
        }
    }

    pub fn replace(&mut self, commands: &mut Commands, id: Entity, pos: Vec2) {
        if self.created {
            self.despawn(commands);
        }

        *self = SelectedVertex::existing(id, pos);
    }

    pub fn despawn(&self, commands: &mut Commands) {
        commands.entity(self.id).remove_parent().despawn();
    }

    pub fn commit(&mut self, commands: &mut Commands, root: Entity) {
        if self.created {
            commands
                .entity(self.id)
                .set_parent_in_place(root)
                .remove::<Blueprint>();
            self.created = false;
        }
    }
}

fn set_pos(id: Entity, pos: Vec2) -> impl Command {
    try_modify_component(id, move |mut transform: Mut<Transform>| {
        transform.translation = pos.extend(0.);
    })
}
