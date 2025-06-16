use bevy::{ecs::system::SystemParam, prelude::*};

use crate::map::{
    door::DoorLinks,
    room::{contents::ContainingRoom, links::RoomLinks, mesh::RoomMesh, paths::RoomPaths},
};

#[derive(SystemParam)]
#[expect(unused)]
pub struct PathParam<'w, 's> {
    door_q: Query<'w, 's, &'static DoorLinks>,
    pawn_q: Query<'w, 's, (&'static Transform, &'static ContainingRoom)>,
    room_q: Query<'w, 's, (&'static RoomLinks, &'static RoomMesh, &'static RoomPaths)>,
}

pub struct Path {
    pub steps: Vec<PathStep>,
}

pub enum PathStep {
    Point(Vec2),
    Door(Vec2, Entity),
}

impl PathParam<'_, '_> {
    pub fn path(&mut self, _pawn: Entity) -> Option<Path> {
        None
    }

    pub fn room_path(&mut self) -> Option<Vec<Entity>> {
        None
    }
}
