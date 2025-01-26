use avian2d::prelude::*;
use bevy::{
    ecs::{entity::MapEntities, reflect::ReflectMapEntities},
    prelude::*,
    utils::HashMap,
};
use pb_util::{swap_remove_item, try_res_s};
use serde::{Deserialize, Serialize};

pub const RADIUS: f32 = 0.125;

#[derive(Debug, Default, Clone, Component, Reflect, Serialize, Deserialize)]
#[reflect(Component, Serialize, Deserialize)]
pub struct Vertex;

#[derive(Debug, Clone, Component, Reflect, Serialize, Deserialize)]
#[reflect(Component, Serialize, Deserialize, MapEntities)]
#[require(
    RigidBody(|| RigidBody::Static)
)]
pub struct Wall {
    start: Entity,
    end: Entity,
}

#[derive(Default, Resource)]
pub struct WallMap {
    map: HashMap<Entity, Vec<Entity>>,
}

#[derive(Bundle)]
pub struct VertexBundle {
    pub vertex: Vertex,
    pub transform: Transform,
}

impl Wall {
    pub fn new(start: Entity, end: Entity) -> Self {
        Wall { start, end }
    }
}

impl MapEntities for Wall {
    fn map_entities<M: EntityMapper>(&mut self, entity_mapper: &mut M) {
        self.start = entity_mapper.map_entity(self.start);
        self.end = entity_mapper.map_entity(self.end);
    }
}

impl WallMap {
    pub fn get(&self, start: Entity) -> &'_ [Entity] {
        match self.map.get(&start) {
            Some(entities) => entities.as_slice(),
            None => &[],
        }
    }

    fn add(&mut self, start: Entity, end: Entity) {
        self.map.entry(start).or_default().push(end);
        self.map.entry(end).or_default().push(start);
    }

    fn remove(&mut self, start: Entity, end: Entity) {
        swap_remove_item(
            self.map.get_mut(&start).expect("wall map not updated"),
            &end,
        );
        swap_remove_item(
            self.map.get_mut(&end).expect("wall map not updated"),
            &start,
        );
    }
}

impl VertexBundle {
    pub fn new(position: Vec2) -> Self {
        Self {
            vertex: Vertex,
            transform: Transform::from_translation(position.extend(0.)),
        }
    }
}

pub fn wall_added(
    trigger: Trigger<OnInsert, Wall>,
    mut commands: Commands,
    mut map: ResMut<WallMap>,
    wall_q: Query<&Wall>,
    mut vertex_q: Query<&Transform, With<Wall>>,
) {
    let wall = try_res_s!(wall_q.get(trigger.entity()));
    let [start, end] = vertex_q.many_mut([wall.start, wall.end]);

    map.add(wall.start, wall.end);

    commands.entity(trigger.entity()).insert((
        // TODO transform needed for capsule?
        Collider::capsule_endpoints(RADIUS, start.translation.xy(), end.translation.xy()),
    ));
}

pub fn wall_removed(
    trigger: Trigger<OnReplace, Wall>,
    mut map: ResMut<WallMap>,
    wall_q: Query<&Wall>,
) {
    let wall = try_res_s!(wall_q.get(trigger.entity()));
    map.remove(wall.start, wall.end);
}
