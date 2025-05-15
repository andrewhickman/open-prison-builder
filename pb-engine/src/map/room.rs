use bevy::{platform::collections::HashMap, prelude::*};
use pb_util::event::Inserted;
use polyanya::{Mesh, Trimesh};
use spade::Triangulation;

use crate::{
    map::{Map, Room},
    root::Root,
};

#[derive(Clone, Debug, Component)]
pub struct RoomMesh {
    #[expect(unused)]
    mesh: Mesh,
}

pub fn update_mesh(
    mut commands: Commands,
    mut room_e: EventReader<Inserted<Room>>,
    room_q: Query<&Room>,
    map_q: Query<&Map>,
    parent_q: Query<&ChildOf>,
    root_q: Query<Has<Root>>,
) -> Result {
    for event in room_e.read() {
        let root = parent_q.root_ancestor(event.target);
        if root_q.get(root).unwrap_or_default() {
            let room = room_q.get(event.target)?;
            let map = map_q.get(parent_q.get(event.target)?.parent())?;

            let mut vertices = Vec::new();
            let mut triangles = Vec::new();
            let mut vertex_indices = HashMap::new();
            for &face in &room.faces {
                let face = map.triangulation.face(face);
                let triangle = face.vertices().map(|vertex| {
                    *vertex_indices.entry(vertex.fix()).or_insert_with(|| {
                        let index = vertices.len();
                        vertices.push(vertex.data().position);
                        index
                    })
                });

                triangles.push(triangle);
            }

            let trimesh = Trimesh {
                vertices,
                triangles,
            };

            let mesh = Mesh::try_from(trimesh)?;

            commands.entity(event.target).insert(RoomMesh { mesh });
        }
    }
    Ok(())
}
