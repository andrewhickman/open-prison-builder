use bevy::{platform::collections::HashMap, prelude::*};

use crate::map::room::{
    links::RoomLinks,
    mesh::{Path, RoomMesh},
};

#[derive(Component, Debug, Default)]
pub struct RoomPaths {
    paths: HashMap<(Entity, Entity), Path>,
}

pub fn update(
    mut room_q: Query<
        (&RoomMesh, &RoomLinks, &mut RoomPaths),
        Or<(Changed<RoomMesh>, Changed<RoomLinks>)>,
    >,
) {
    room_q.par_iter_mut().for_each(|(mesh, links, mut paths)| {
        paths.paths.clear();

        for (index, (from, from_point, _)) in links.doors().enumerate() {
            for (to, to_point, _) in links.doors().take(index) {
                if let Some(mut path) = mesh.path(from_point, to_point) {
                    paths.paths.insert((from, to), path.clone());

                    path.path.pop();
                    path.path.reverse();
                    path.path.push(from_point);

                    paths.paths.insert((to, from), path.clone());
                }
            }
        }
    });
}
