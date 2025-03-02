use bevy::prelude::*;
use pb_engine::{
    build::Blueprint,
    wall::{VertexBundle, WallBundle},
    EngineState,
};
use pb_util::{try_modify_component, try_res_s};

use crate::input::{
    action::InputAction,
    picking::point::{grid::Grid, CancelPoint, ClickPoint, SelectPoint},
};

pub fn wall(_: Trigger<Pointer<Click>>, mut commands: Commands) {
    commands
        .spawn(WallAction::default())
        .with_children(|builder| {
            builder.spawn(Grid::default());

            let id = builder.parent_entity();
            builder.spawn(Observer::new(
                move |trigger: Trigger<SelectPoint>,
                      mut commands: Commands,
                      mut action_q: Query<&mut WallAction>| {
                    try_res_s!(action_q.get_mut(id)).select_point(
                        id,
                        &mut commands,
                        trigger.event().point,
                    );
                },
            ));
            builder.spawn(Observer::new(
                move |_: Trigger<CancelPoint>,
                      mut commands: Commands,
                      mut action_q: Query<&mut WallAction>| {
                    try_res_s!(action_q.get_mut(id)).cancel_point(&mut commands);
                },
            ));
            builder.spawn(Observer::new(
                move |trigger: Trigger<ClickPoint>,
                      mut commands: Commands,
                      mut action_q: Query<&mut WallAction>,
                      engine_state: Res<State<EngineState>>| {
                    try_res_s!(action_q.get_mut(id)).click_point(
                        id,
                        &mut commands,
                        trigger.event().point,
                        &engine_state,
                    );
                },
            ));
        });
}

#[derive(Default, Debug, Component)]
#[require(InputAction, Transform, Visibility)]
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
                commands.queue(try_modify_component(
                    start,
                    move |mut transform: Mut<Transform>| transform.translation = point.extend(0.),
                ));

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

    pub fn cancel_point(&mut self, commands: &mut Commands) {
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
}

fn set_pos(id: Entity, pos: Vec2) -> impl Command {
    try_modify_component(id, move |mut transform: Mut<Transform>| {
        transform.translation = pos.extend(0.);
    })
}
