use bevy::prelude::*;
use pb_engine::{
    build::Blueprint,
    wall::{VertexBundle, Wall, WallMap},
    EngineState,
};
use pb_render::wall::Hidden;
use pb_util::{try_modify_component, ChildBuildExt};

use crate::{
    action::Action,
    input::{
        cancel::Cancellable,
        picking::{
            physics::{
                wall::{CancelWall, ClickWall, SelectWall, WallPickKind},
                PhysicsPickingState,
            },
            point::{grid::Grid, CancelPoint, ClickPoint, SelectPoint},
        },
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

#[derive(Default, Debug, Component, TypePath)]
#[require(Action, Cancellable, PhysicsPickingState(|| PhysicsPickingState::Wall), Transform, Visibility, Name(|| Name::new(WallAction::type_path())))]
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
    split: Option<SplitWall>,
}

#[derive(Debug, Copy, Clone)]
pub struct SelectedWall {
    id: Option<Entity>,
}

#[derive(Debug, Copy, Clone)]
pub struct SplitWall {
    start: SelectedWall,
    end: SelectedWall,
    prev: Entity,
}

fn select_point(
    trigger: Trigger<SelectPoint>,
    mut commands: Commands,
    mut wall_map: ResMut<WallMap>,
    mut action: Single<(Entity, &mut WallAction)>,
) {
    let (id, ref mut action) = *action;
    action.select_point(id, &mut commands, &mut wall_map, trigger.point);
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
    mut wall_map: ResMut<WallMap>,
    mut action: Single<(Entity, &mut WallAction)>,
    engine_state: Res<State<EngineState>>,
) {
    let (id, ref mut action) = *action;
    action.select_point(id, &mut commands, &mut wall_map, trigger.point);
    action.click(&mut commands, &engine_state)
}

fn select_wall(
    trigger: Trigger<SelectWall>,
    mut commands: Commands,
    mut wall_map: ResMut<WallMap>,
    mut action: Single<(Entity, &mut WallAction)>,
) {
    let (id, ref mut action) = *action;
    match trigger.kind {
        WallPickKind::Vertex { vertex, position } => {
            action.select_vertex(id, &mut commands, &mut wall_map, vertex, position)
        }
        WallPickKind::Wall {
            wall,
            position,
            start,
            start_position,
            end,
            end_position,
        } => action.select_wall(
            id,
            &mut commands,
            &mut wall_map,
            wall,
            position,
            start,
            start_position,
            end,
            end_position,
        ),
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
    mut wall_map: ResMut<WallMap>,
    mut action: Single<(Entity, &mut WallAction)>,
    engine_state: Res<State<EngineState>>,
) {
    let (id, ref mut action) = *action;
    match trigger.kind {
        WallPickKind::Vertex { vertex, position } => {
            action.select_vertex(id, &mut commands, &mut wall_map, vertex, position);
            action.click(&mut commands, &engine_state)
        }
        WallPickKind::Wall {
            wall,
            position,
            start,
            start_position,
            end,
            end_position,
        } => {
            action.select_wall(
                id,
                &mut commands,
                &mut wall_map,
                wall,
                position,
                start,
                start_position,
                end,
                end_position,
            );
            action.click(&mut commands, &engine_state)
        }
    }
}

pub fn cancel(
    _: Trigger<OnRemove, WallAction>,
    mut commands: Commands,
    hidden_q: Query<Entity, With<Hidden>>,
) {
    for entity in &hidden_q {
        commands.entity(entity).remove::<Hidden>();
    }
}

impl WallAction {
    pub fn select_point(
        &mut self,
        this: Entity,
        commands: &mut Commands,
        wall_map: &mut WallMap,
        point: Vec2,
    ) {
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
                let wall = SelectedWall::spawn(commands, wall_map, this, start, end);

                *self = WallAction::PreviewEnd { start, end, wall };
            }
            WallAction::PreviewEnd {
                start,
                ref mut end,
                ref mut wall,
                ..
            } => {
                if end.update_spawned(commands, this, point) {
                    wall.replace(commands, wall_map, this, start, *end);
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
        wall_map: &mut WallMap,
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
                let wall = SelectedWall::spawn(commands, wall_map, this, start, end);

                *self = WallAction::PreviewEnd { start, end, wall };
            }
            WallAction::PreviewEnd {
                start,
                ref mut end,
                ref mut wall,
                ..
            } => {
                if end.update_existing(commands, vertex, pos) {
                    wall.replace(commands, wall_map, this, start, *end);
                } else {
                    wall.update(commands, start, *end);
                }
            }
        }
    }

    pub fn select_wall(
        &mut self,
        this: Entity,
        commands: &mut Commands,
        wall_map: &mut WallMap,
        prev_wall: Entity,
        pos: Vec2,
        wall_start: Entity,
        wall_start_pos: Vec2,
        wall_end: Entity,
        wall_end_pos: Vec2,
    ) {
        match *self {
            WallAction::SelectStart => {
                *self = WallAction::PreviewStart {
                    start: SelectedVertex::split(
                        commands,
                        wall_map,
                        this,
                        prev_wall,
                        pos,
                        wall_start,
                        wall_start_pos,
                        wall_end,
                        wall_end_pos,
                    ),
                };
            }
            WallAction::PreviewStart { ref mut start } => {
                start.update_split(
                    commands,
                    wall_map,
                    this,
                    prev_wall,
                    pos,
                    wall_start,
                    wall_start_pos,
                    wall_end,
                    wall_end_pos,
                );
            }
            WallAction::SelectEnd { start } => {
                let end = SelectedVertex::split(
                    commands,
                    wall_map,
                    this,
                    prev_wall,
                    pos,
                    wall_start,
                    wall_start_pos,
                    wall_end,
                    wall_end_pos,
                );
                let wall = SelectedWall::spawn(commands, wall_map, this, start, end);

                *self = WallAction::PreviewEnd { start, end, wall };
            }
            WallAction::PreviewEnd {
                start,
                ref mut end,
                ref mut wall,
                ..
            } => {
                if end.update_split(
                    commands,
                    wall_map,
                    this,
                    prev_wall,
                    pos,
                    wall_start,
                    wall_start_pos,
                    wall_end,
                    wall_end_pos,
                ) {
                    wall.replace(commands, wall_map, this, start, *end);
                } else {
                    wall.update(commands, start, *end);
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
            split: None,
        }
    }

    pub fn existing(id: Entity, pos: Vec2) -> Self {
        SelectedVertex {
            id,
            pos,
            spawned: false,
            split: None,
        }
    }

    pub fn split(
        commands: &mut Commands,
        wall_map: &mut WallMap,
        parent: Entity,
        wall: Entity,
        pos: Vec2,
        start: Entity,
        start_pos: Vec2,
        end: Entity,
        end_pos: Vec2,
    ) -> Self {
        let vertex = SelectedVertex::spawn(commands, parent, pos);
        let split = SplitWall::spawn(
            commands,
            wall_map,
            parent,
            wall,
            SelectedVertex::existing(start, start_pos),
            SelectedVertex::existing(end, end_pos),
            vertex,
        );

        SelectedVertex {
            split: Some(split),
            ..vertex
        }
    }

    pub fn update_spawned(&mut self, commands: &mut Commands, parent: Entity, pos: Vec2) -> bool {
        self.unsplit(commands);
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
        self.unsplit(commands);
        if self.id != id {
            self.despawn(commands);
            *self = SelectedVertex::existing(id, pos);
            true
        } else {
            false
        }
    }

    pub fn update_split(
        &mut self,
        commands: &mut Commands,
        wall_map: &mut WallMap,
        parent: Entity,
        wall: Entity,
        pos: Vec2,
        start: Entity,
        start_pos: Vec2,
        end: Entity,
        end_pos: Vec2,
    ) -> bool {
        if self.spawned {
            self.pos = pos;
            commands.queue(try_modify_component(
                self.id,
                move |mut transform: Mut<Transform>| {
                    transform.translation = pos.extend(0.);
                },
            ));

            if let Some(split) = &mut self.split {
                split.update(
                    commands,
                    SelectedVertex::existing(start, start_pos),
                    SelectedVertex::existing(end, end_pos),
                    SelectedVertex::existing(self.id, self.pos),
                );
            } else {
                self.split = Some(SplitWall::spawn(
                    commands,
                    wall_map,
                    parent,
                    wall,
                    SelectedVertex::existing(start, start_pos),
                    SelectedVertex::existing(end, end_pos),
                    SelectedVertex::existing(self.id, self.pos),
                ));
            }

            false
        } else {
            *self = SelectedVertex::split(
                commands, wall_map, parent, wall, pos, start, start_pos, end, end_pos,
            );
            true
        }
    }

    pub fn despawn(&self, commands: &mut Commands) {
        if self.spawned {
            commands.entity(self.id).remove_parent().despawn();
        }
        if let Some(split) = self.split {
            split.despawn(commands);
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
        if let Some(split) = self.split.take() {
            split.commit(commands, root);
        }
    }

    pub fn unsplit(&mut self, commands: &mut Commands) {
        if let Some(split) = self.split.take() {
            split.despawn(commands);
        }
    }
}

impl SelectedWall {
    pub fn spawn(
        commands: &mut Commands,
        wall_map: &mut WallMap,
        parent: Entity,
        start: SelectedVertex,
        end: SelectedVertex,
    ) -> Self {
        if let Some(mut entity) = wall_map.insert(commands, start.id, end.id) {
            entity.insert((Wall::transform(start.pos, end.pos), Blueprint));
            entity.set_parent(parent);
            let id = entity.id();

            SelectedWall { id: Some(id) }
        } else {
            SelectedWall { id: None }
        }
    }

    pub fn replace(
        &mut self,
        commands: &mut Commands,
        wall_map: &mut WallMap,
        parent: Entity,
        start: SelectedVertex,
        end: SelectedVertex,
    ) {
        self.despawn(commands);
        *self = SelectedWall::spawn(commands, wall_map, parent, start, end);
    }

    pub fn update(&mut self, commands: &mut Commands, start: SelectedVertex, end: SelectedVertex) {
        if let Some(id) = self.id {
            commands.queue(try_modify_component(
                id,
                move |mut transform: Mut<Transform>| {
                    transform.set_if_neq(Wall::transform(start.pos, end.pos));
                },
            ));
        }
    }

    pub fn despawn(&self, commands: &mut Commands) {
        if let Some(id) = self.id {
            commands.entity(id).remove_parent().despawn();
        }
    }

    pub fn commit(&self, commands: &mut Commands, root: Entity) {
        if let Some(id) = self.id {
            commands.entity(id).set_parent(root).remove::<Blueprint>();
        }
    }
}

impl SplitWall {
    pub fn spawn(
        commands: &mut Commands,
        wall_map: &mut WallMap,
        parent: Entity,
        prev: Entity,
        start: SelectedVertex,
        end: SelectedVertex,
        mid: SelectedVertex,
    ) -> Self {
        commands.entity(prev).insert(Hidden);

        SplitWall {
            start: SelectedWall::spawn(commands, wall_map, parent, start, mid),
            end: SelectedWall::spawn(commands, wall_map, parent, mid, end),
            prev,
        }
    }

    pub fn update(
        &mut self,
        commands: &mut Commands,
        start: SelectedVertex,
        end: SelectedVertex,
        mid: SelectedVertex,
    ) {
        self.start.update(commands, start, mid);
        self.end.update(commands, mid, end);
    }

    pub fn commit(&self, commands: &mut Commands, root: Entity) {
        self.start.commit(commands, root);
        self.end.commit(commands, root);
        commands.entity(self.prev).despawn();
    }

    pub fn despawn(&self, commands: &mut Commands) {
        self.start.despawn(commands);
        self.end.despawn(commands);
        commands.entity(self.prev).remove::<Hidden>();
    }
}
