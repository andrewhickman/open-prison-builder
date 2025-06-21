use std::collections::BTreeMap;

use bevy::prelude::*;

#[derive(Default, Clone, Debug, Component)]
pub struct RoomLinks {
    doors: BTreeMap<Entity, RoomLink>,
}

#[derive(Clone, Debug)]
pub struct RoomLink {
    position: Vec2,
    room: Entity,
}

impl RoomLinks {
    pub fn doors(&self) -> impl ExactSizeIterator<Item = (Entity, Vec2, Entity)> {
        self.doors
            .iter()
            .map(|(&door, link)| (door, link.position, link.room))
    }

    pub fn insert_door(&mut self, door: Entity, position: Vec2, room: Entity) {
        self.doors.insert(door, RoomLink { position, room });
    }

    pub fn remove_door(&mut self, door: Entity) {
        self.doors.remove(&door);
    }
}
