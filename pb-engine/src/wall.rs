use std::slice;

use avian2d::prelude::*;
use bevy::{
    ecs::{entity::MapEntities, reflect::ReflectMapEntities},
    platform::collections::HashMap,
    prelude::*,
};
use pb_util::try_res_s;
use serde::{Deserialize, Serialize};

use crate::{build::Blueprint, picking::Layer};

pub const RADIUS: f32 = 0.125;

#[derive(Debug, Default, Clone, Component, Reflect, Serialize, Deserialize)]
#[reflect(Component, Serialize, Deserialize)]
pub struct Vertex;

#[derive(Debug, Component, Reflect, Serialize, Deserialize)]
#[reflect(Component, Serialize, Deserialize, MapEntities)]
pub struct Wall {
    start: Entity,
    end: Entity,
}

#[derive(Default, Debug, Resource)]
pub struct WallMap {
    map: HashMap<Entity, Vec<WallMapEntry>>,
}

#[derive(Debug)]
pub struct WallMapEntry {
    pub wall: Entity,
    pub end: Entity,
}

#[derive(Bundle)]
pub struct VertexBundle {
    pub vertex: Vertex,
    pub transform: Transform,
}

impl Wall {
    pub(crate) fn new(start: Entity, end: Entity) -> Self {
        Wall { start, end }
    }

    pub fn transform(start_pos: Vec2, end_pos: Vec2) -> Transform {
        Transform::from_translation(start_pos.midpoint(end_pos).extend(0.))
    }

    pub fn vertices(&self) -> [Entity; 2] {
        [self.start, self.end]
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
        self.start = entity_mapper.get_mapped(self.start);
        self.end = entity_mapper.get_mapped(self.end);
    }
}

impl WallMap {
    pub fn insert<'a>(
        &mut self,
        commands: &'a mut Commands,
        start: Entity,
        end: Entity,
    ) -> Option<EntityCommands<'a>> {
        if start == end {
            return None;
        }

        if self.get_wall(start, end).is_some() {
            return None;
        }

        let entity = commands.spawn(Wall::new(start, end));
        self.add(entity.id(), start, end);
        Some(entity)
    }

    pub fn get(&self, start: Entity) -> impl Iterator<Item = &'_ WallMapEntry> {
        match self.map.get(&start) {
            Some(entries) => entries.iter(),
            None => slice::Iter::default(),
        }
    }

    pub fn get_wall(&self, start: Entity, end: Entity) -> Option<Entity> {
        self.get(start)
            .find(|entry| entry.end == end)
            .map(|entry| entry.wall)
    }

    fn add(&mut self, id: Entity, start: Entity, end: Entity) {
        match self.get_wall(start, end) {
            Some(existing) if existing == id => return,
            Some(_) => {
                warn!("inserting duplicate wall between {start} and {end}")
            }
            _ => (),
        }

        self.map
            .entry(start)
            .or_default()
            .push(WallMapEntry { wall: id, end });
        self.map.entry(end).or_default().push(WallMapEntry {
            wall: id,
            end: start,
        });
    }

    fn remove(&mut self, id: Entity, start: Entity, end: Entity) {
        self.map
            .get_mut(&start)
            .expect("wall map not updated")
            .retain(|entry| entry.wall != id);
        self.map
            .get_mut(&end)
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

pub fn wall_added(
    trigger: Trigger<OnInsert, Wall>,
    mut map: ResMut<WallMap>,
    wall_q: Query<&Wall>,
) {
    let wall = try_res_s!(wall_q.get(trigger.target()));
    map.add(trigger.target(), wall.start(), wall.end());
}

pub fn wall_removed(
    trigger: Trigger<OnReplace, Wall>,
    mut map: ResMut<WallMap>,
    wall_q: Query<&Wall>,
) {
    let wall = try_res_s!(wall_q.get(trigger.target()));
    map.remove(trigger.target(), wall.start(), wall.end());
}

pub fn add_colliders(
    mut commands: Commands,
    wall_q: Query<(Entity, &Wall), (Without<Collider>, Without<Blueprint>)>,
    vertex_q: Query<&Transform, With<Vertex>>,
) {
    for (id, wall) in &wall_q {
        let [start, end] = vertex_q.get_many(wall.vertices()).unwrap();

        let midpoint = start.translation.midpoint(end.translation);

        commands.entity(id).insert((
            RigidBody::Static,
            Transform::from_translation(midpoint),
            Collider::capsule_endpoints(
                RADIUS,
                (start.translation - midpoint).xy(),
                (end.translation - midpoint).xy(),
            ),
            CollisionLayers::new(Layer::Wall, LayerMask::ALL),
        ));
    }
}
