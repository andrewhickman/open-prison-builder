use std::slice;

use avian2d::prelude::*;
use bevy::{
    ecs::{entity::MapEntities, reflect::ReflectMapEntities},
    prelude::*,
    utils::HashMap,
};
use pb_util::try_res_s;
use serde::{Deserialize, Serialize};

pub const RADIUS: f32 = 0.125;

#[derive(Debug, Default, Clone, Component, Reflect, Serialize, Deserialize)]
#[reflect(Component, Serialize, Deserialize)]
pub struct Vertex;

#[derive(Debug, Copy, Clone, Component, Reflect, Serialize, Deserialize)]
#[reflect(Component, Serialize, Deserialize, MapEntities)]
pub struct Wall {
    start: Entity,
    end: Entity,
}

#[derive(Default, Resource)]
pub struct WallMap {
    map: HashMap<Entity, Vec<WallMapEntry>>,
}

pub struct WallMapEntry {
    pub wall: Entity,
    pub end: Entity,
}

#[derive(Bundle)]
pub struct VertexBundle {
    pub vertex: Vertex,
    pub transform: Transform,
}

#[derive(Bundle)]
pub struct WallBundle {
    pub wall: Wall,
    pub transform: Transform,
}

impl Wall {
    pub fn new(start: Entity, end: Entity) -> Self {
        Wall { start, end }
    }

    pub fn start(&self) -> Entity {
        self.start
    }

    pub fn end(&self) -> Entity {
        self.end
    }
}

impl MapEntities for Wall {
    fn map_entities<M: EntityMapper>(&mut self, entity_mapper: &mut M) {
        self.start = entity_mapper.map_entity(self.start);
        self.end = entity_mapper.map_entity(self.end);
    }
}

impl WallMap {
    pub fn get(&self, start: Entity) -> impl Iterator<Item = &'_ WallMapEntry> {
        match self.map.get(&start) {
            Some(entries) => entries.iter(),
            None => slice::Iter::default(),
        }
    }

    fn add(&mut self, id: Entity, wall: Wall) {
        self.map.entry(wall.start).or_default().push(WallMapEntry {
            wall: id,
            end: wall.end,
        });
        self.map.entry(wall.end).or_default().push(WallMapEntry {
            wall: id,
            end: wall.start,
        });
    }

    fn remove(&mut self, id: Entity, wall: Wall) {
        self.map
            .get_mut(&wall.start)
            .expect("wall map not updated")
            .retain(|entry| entry.wall != id);
        self.map
            .get_mut(&wall.end)
            .expect("wall map not updated")
            .retain(|entry| entry.wall != id);
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

impl WallBundle {
    pub fn new(start: Entity, start_pos: Vec2, end: Entity, end_pos: Vec2) -> Self {
        WallBundle {
            wall: Wall::new(start, end),
            transform: Transform::from_translation(start_pos.midpoint(end_pos).extend(0.)),
        }
    }
}

pub fn wall_added(
    trigger: Trigger<OnInsert, Wall>,
    mut map: ResMut<WallMap>,
    wall_q: Query<&Wall>,
) {
    let wall = try_res_s!(wall_q.get(trigger.entity()));

    map.add(trigger.entity(), *wall);
}

pub fn wall_removed(
    trigger: Trigger<OnReplace, Wall>,
    mut map: ResMut<WallMap>,
    wall_q: Query<&Wall>,
) {
    let wall = try_res_s!(wall_q.get(trigger.entity()));
    map.remove(trigger.entity(), *wall);
}

pub fn add_colliders(
    mut commands: Commands,
    wall_q: Query<(Entity, &Wall), (Without<Collider>, With<Parent>)>,
    vertex_q: Query<&Transform, With<Vertex>>,
) {
    for (id, wall) in &wall_q {
        let [start, end] = vertex_q.many([wall.start, wall.end]);

        let midpoint = start.translation.midpoint(end.translation);

        commands.entity(id).insert((
            RigidBody::Static,
            Transform::from_translation(midpoint),
            Collider::capsule_endpoints(
                RADIUS,
                (start.translation - midpoint).xy(),
                (end.translation - midpoint).xy(),
            ),
        ));
    }
}
