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
        start: Entity,
        start_point: Vec2,
    },
    SelectEnd {
        start: Entity,
        start_point: Vec2,
    },
    PreviewEnd {
        start: Entity,
        start_point: Vec2,
        wall: Entity,
        end: Entity,
        end_point: Vec2,
    },
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
                let start = commands
                    .spawn((VertexBundle::new(point), Blueprint))
                    .set_parent(this)
                    .id();

                *self = WallAction::PreviewStart {
                    start,
                    start_point: point,
                };
            }
            WallAction::PreviewStart {
                start,
                start_point: ref mut start_pos,
            } => {
                commands.queue(set_pos(start, point));

                *start_pos = point;
            }
            WallAction::SelectEnd {
                start,
                start_point: start_pos,
            } => {
                let end = commands
                    .spawn((VertexBundle::new(point), Blueprint))
                    .set_parent(this)
                    .id();
                let wall = commands
                    .spawn((WallBundle::new(start, start_pos, end, point), Blueprint))
                    .set_parent(this)
                    .id();

                *self = WallAction::PreviewEnd {
                    start,
                    start_point: start_pos,
                    end,
                    wall,
                    end_point: point,
                };
            }
            WallAction::PreviewEnd {
                start_point: start_pos,
                end,
                end_point: ref mut end_pos,
                wall,
                ..
            } => {
                commands.queue(set_pos(wall, start_pos.midpoint(point)));
                commands.queue(set_pos(end, point));

                *end_pos = point;
            }
        }
    }

    pub fn select_vertex(&mut self, _: Entity, _: &mut Commands, _: Entity, pos: Vec2) {
        info!("select_vertex {pos:?}")
    }

    pub fn cancel(&mut self, commands: &mut Commands) {
        match *self {
            WallAction::SelectStart => {}
            WallAction::PreviewStart { start, .. } => {
                commands.entity(start).remove_parent().despawn();

                *self = WallAction::SelectStart;
            }
            WallAction::SelectEnd { .. } => {}
            WallAction::PreviewEnd {
                start,
                start_point: start_pos,
                wall,
                end,
                ..
            } => {
                commands.entity(wall).remove_parent().despawn();
                commands.entity(end).remove_parent().despawn();

                *self = WallAction::SelectEnd {
                    start,
                    start_point: start_pos,
                };
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
            WallAction::PreviewStart {
                start,
                start_point: start_pos,
            } => {
                *self = WallAction::SelectEnd {
                    start,
                    start_point: start_pos,
                };
            }
            WallAction::SelectEnd { .. } => {}
            WallAction::PreviewEnd {
                start,
                wall,
                end,
                end_point,
                ..
            } => {
                commands
                    .entity(start)
                    .set_parent_in_place(root)
                    .remove::<Blueprint>();
                commands
                    .entity(wall)
                    .set_parent_in_place(root)
                    .remove::<Blueprint>();
                commands
                    .entity(end)
                    .set_parent_in_place(root)
                    .remove::<Blueprint>();

                *self = WallAction::SelectEnd {
                    start: end,
                    start_point: end_point,
                };
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

fn set_pos(id: Entity, pos: Vec2) -> impl Command {
    try_modify_component(id, move |mut transform: Mut<Transform>| {
        transform.translation = pos.extend(0.);
    })
}
