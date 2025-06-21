use std::{iter::once, sync::Arc};

use bevy::{platform::collections::HashMap, prelude::*};

use crate::map::room::{link::RoomLinks, mesh::RoomMesh};

#[derive(Debug, Clone)]
pub struct RoomPath {
    pub length: f32,
    pub path: Arc<[Vec2]>,
}

#[derive(Component, Debug, Default)]
pub struct RoomPathCache {
    paths: HashMap<(Entity, Entity), RoomPath>,
}

pub fn update(
    mut room_q: Query<
        (&RoomMesh, &RoomLinks, &mut RoomPathCache),
        Or<(Changed<RoomMesh>, Changed<RoomLinks>)>,
    >,
) {
    room_q.par_iter_mut().for_each(|(mesh, links, mut paths)| {
        paths.paths.clear();

        for (index, (from, from_point, _)) in links.doors().enumerate() {
            for (to, to_point, _) in links.doors().take(index) {
                if let Some(path) = mesh.path(from_point, to_point) {
                    paths.paths.insert((from, to), path.clone());
                    paths.paths.insert((to, from), path.reversed());
                } else {
                    error!("failed to find room path from {from_point} to {to_point}")
                }
            }
        }
    });
}

impl RoomPathCache {
    pub fn get(&self, from_door: Entity, to_door: Entity) -> Option<&RoomPath> {
        self.paths.get(&(from_door, to_door))
    }
}

impl RoomPath {
    pub fn new(from_point: Vec2, path: polyanya::Path) -> Self {
        debug_assert!(!path.path.is_empty());
        RoomPath {
            path: once(from_point).chain(path.path).collect(),
            length: path.length,
        }
    }

    pub fn from_point(&self) -> Vec2 {
        *self.path.first().expect("empty path")
    }

    pub fn to_point(&self) -> Vec2 {
        *self.path.last().expect("empty path")
    }

    pub fn reversed(&self) -> Self {
        let mut path = self.path.clone();
        Arc::make_mut(&mut path).reverse();
        RoomPath {
            path,
            length: self.length,
        }
    }
}
