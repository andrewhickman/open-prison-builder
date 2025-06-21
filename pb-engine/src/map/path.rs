use std::{cmp::Ordering, collections::BinaryHeap};

use bevy::{
    ecs::{entity::EntityHashMap, system::SystemParam},
    math::FloatOrd,
    platform::collections::hash_map::Entry,
    prelude::*,
    utils::Parallel,
};

use crate::{
    map::{
        Map,
        door::DoorLinks,
        room::{
            link::RoomLinks,
            path::{RoomPath, RoomPathCache},
        },
        wall::Wall,
    },
    root::ChildOfRoot,
};

#[derive(Clone, Debug)]
pub struct MapPath {
    pub from_room: Entity,
    pub from_path: RoomPath,
    pub entries: Vec<MapPathEntry>,
}

#[derive(Clone, Debug)]
pub struct MapPathEntry {
    pub room: Entity,
    pub path: RoomPath,
    pub door: Entity,
    pub length: f32,
}

#[derive(Resource, Default)]
pub struct MapPathCache {
    cache: EntityHashMap<EntityHashMap<MapPath>>,
    updates: Parallel<Vec<MapPath>>,
}

#[derive(SystemParam)]
pub struct MapPathParam<'w, 's> {
    room_q: Query<'w, 's, (&'static RoomLinks, &'static RoomPathCache)>,
    door_q: Query<'w, 's, &'static DoorLinks>,
    wall_q: Query<'w, 's, &'static Wall>,
    cache: Res<'w, MapPathCache>,
}

pub fn invalidate_cache(
    map_q: Query<(), (Changed<Map>, With<ChildOfRoot>)>,
    mut cache: ResMut<MapPathCache>,
) {
    if map_q.iter().next().is_some() {
        cache.clear();
    }
}

pub fn update_cache(mut cache: ResMut<MapPathCache>, mut updates: Local<Vec<MapPath>>) {
    cache.updates.drain_into(&mut updates);

    for update in updates.drain(..) {
        if !cache.contains(update.to_door(), update.from_door()) {
            cache.insert_suffixes(update.reversed());
        }

        if !cache.contains(update.from_door(), update.to_door()) {
            cache.insert_suffixes(update);
        }
    }
}

impl MapPath {
    pub fn from_door(&self) -> Entity {
        self.entries.first().expect("empty path").door
    }

    pub fn from_room(&self) -> Entity {
        self.from_room
    }

    pub fn to_door(&self) -> Entity {
        self.entries.last().expect("empty path").door
    }

    pub fn to_room(&self) -> Entity {
        self.entries.last().expect("empty path").room
    }

    pub fn reversed(&self) -> Self {
        let mut entries = Vec::with_capacity(self.entries.len());

        for window in self.entries.windows(2).rev() {
            entries.push(MapPathEntry {
                door: window[1].door,
                room: window[0].room,
                path: window[0].path.reversed(),
                length: window[1].length,
            });
        }

        entries.push(MapPathEntry {
            door: self.from_door(),
            room: self.from_room(),
            path: self.from_path.reversed(),
            length: self.entries.first().expect("empty path").length,
        });

        MapPath {
            from_room: self.to_room(),
            from_path: self.entries.last().expect("empty path").path.reversed(),
            entries,
        }
    }
}

impl MapPathCache {
    pub fn contains(&self, from_door: Entity, to_door: Entity) -> bool {
        match self.cache.get(&from_door) {
            Some(door_cache) => door_cache.contains_key(&to_door),
            None => false,
        }
    }

    pub fn get(&self, from_door: Entity, to_door: Entity) -> Option<&MapPath> {
        match self.cache.get(&from_door) {
            Some(door_cache) => door_cache.get(&to_door),
            None => None,
        }
    }

    pub fn update(&self, path: MapPath) {
        self.updates.borrow_local_mut().push(path)
    }

    fn insert_suffixes(&mut self, path: MapPath) {
        for i in 1..path.entries.len() {
            if self.contains(path.entries[i].door, path.to_door()) {
                break;
            }

            let suffix = MapPath {
                from_room: path.entries[i - 1].room,
                from_path: path.entries[i - 1].path.clone(),
                entries: path.entries[i..].to_vec(),
            };

            self.insert(suffix);
        }

        self.insert(path);
    }

    fn insert(&mut self, path: MapPath) {
        self.cache
            .entry(path.from_door())
            .or_default()
            .insert(path.to_door(), path);
    }

    fn clear(&mut self) {
        self.cache.clear();
    }
}

impl MapPathParam<'_, '_> {
    pub fn path(
        &self,
        from_position: Vec2,
        from_room: Entity,
        to_position: Vec2,
        to_room: Entity,
    ) -> Result<Option<MapPath>> {
        if from_room == to_room {
            // let room
        }

        todo!()
    }

    pub fn door_path(&self, from_door: Entity, to_door: Entity) -> Result<Option<MapPath>> {
        if let Some(path) = self.cache.get(from_door, to_door) {
            Ok(Some(path.clone()))
        } else if let Some(path) = self.find_door_path(from_door, to_door)? {
            self.cache.update(path.clone());
            Ok(Some(path))
        } else {
            Ok(None)
        }
    }

    pub fn find_door_path(&self, from_door: Entity, to_door: Entity) -> Result<Option<MapPath>> {
        let mut open = BinaryHeap::new();
        let mut path = EntityHashMap::new();

        let to_position = self.door_position(to_door)?;

        open.push(DoorNode {
            estimated_length: 0.,
            length: 0.,
            door: from_door,
        });

        while let Some(node) = open.pop() {
            if node.door == to_door {
                return todo!();
            }

            if let Some(&(_, shortest_length)) = path.get(&node.door) {
                if node.length > shortest_length {
                    continue;
                }
            }

            self.for_each_door_link(node.door, |door, position, length| {
                if door == from_door {
                    return;
                }

                let new_length = node.length + length;
                match path.entry(door) {
                    Entry::Vacant(entry) => {
                        entry.insert((node.door, new_length));
                    }
                    Entry::Occupied(mut entry) => {
                        let &(_, shortest_length) = entry.get();
                        if new_length > shortest_length {
                            return;
                        }

                        entry.insert((node.door, new_length));
                    }
                }

                let estimated_length = position.distance(to_position);
                open.push(DoorNode {
                    estimated_length: new_length + estimated_length,
                    length: new_length,
                    door,
                });
            })?;
        }

        Ok(None)
    }

    fn door_position(&self, door: Entity) -> Result<Vec2> {
        Ok(self.wall_q.get(door)?.position())
    }

    fn for_each_door_link(&self, door: Entity, mut f: impl FnMut(Entity, Vec2, f32)) -> Result<()> {
        let door_links = self.door_q.get(door)?;
        for room in door_links.rooms() {
            let (room_links, room_paths) = self.room_q.get(room)?;
            for (linked_door, linked_door_position, _) in room_links.doors() {
                if linked_door != door {
                    if let Some(path) = room_paths.get(door, linked_door) {
                        f(linked_door, linked_door_position, path.length);
                    }
                }
            }
        }

        Ok(())
    }
}

#[derive(Copy, Clone)]
struct DoorNode {
    estimated_length: f32,
    length: f32,
    door: Entity,
}

impl PartialEq for DoorNode {
    fn eq(&self, other: &Self) -> bool {
        FloatOrd(self.estimated_length) == FloatOrd(other.estimated_length)
            && FloatOrd(self.length) == FloatOrd(other.length)
    }
}

impl Eq for DoorNode {}

impl PartialOrd for DoorNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for DoorNode {
    fn cmp(&self, other: &Self) -> Ordering {
        FloatOrd(self.estimated_length)
            .cmp(&FloatOrd(other.estimated_length))
            .then_with(|| FloatOrd(self.length).cmp(&FloatOrd(other.length)))
    }
}
