use bevy::{
    ecs::{entity::MapEntities, reflect::ReflectMapEntities},
    prelude::*,
};
use serde::{Deserialize, Serialize};

pub const RADIUS: f32 = 0.125;

#[derive(Copy, Clone, Component, Reflect, Serialize, Deserialize)]
#[reflect(Component, Serialize, Deserialize)]
pub struct Vertex;

#[derive(Copy, Clone, Component, Reflect, Serialize, Deserialize)]
#[reflect(Component, Serialize, Deserialize, MapEntities)]
pub struct Wall {
    pub start: Entity,
    pub end: Entity,
}

#[derive(Bundle)]
pub struct VertexBundle {
    pub vertex: Vertex,
    pub transform: TransformBundle,
}

impl MapEntities for Wall {
    fn map_entities<M: EntityMapper>(&mut self, entity_mapper: &mut M) {
        self.start = entity_mapper.map_entity(self.start);
        self.end = entity_mapper.map_entity(self.end);
    }
}

impl VertexBundle {
    pub fn new(position: Vec2) -> Self {
        Self {
            vertex: Vertex,
            transform: Transform::from_translation(position.extend(0.)).into(),
        }
    }
}
