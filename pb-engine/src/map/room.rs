use bevy::{ecs::entity::EntityHashSet, platform::collections::HashMap, prelude::*};
use pb_util::event::Inserted;
use polyanya::{Mesh, Trimesh};
use spade::Triangulation;

use crate::{
    map::{Map, Room},
    pawn::Pawn,
    root::RootQuery,
};

#[derive(Component, Clone, PartialEq, Eq, Debug)]
#[relationship(relationship_target = RoomContents)]
pub struct ContainingRoom(pub Entity);

#[derive(Component, Default, Debug, PartialEq, Eq)]
#[relationship_target(relationship = ContainingRoom)]
pub struct RoomContents(EntityHashSet);

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
    root_q: RootQuery,
) -> Result {
    for event in room_e.read() {
        if root_q.is_descendant_of_root(event.target) {
            let room = room_q.get(event.target)?;
            let map = map_q.get(root_q.parent(event.target)?)?;

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

            let mut mesh = Mesh::try_from(trimesh)?;
            mesh.merge_polygons();
            mesh.bake();

            commands.entity(event.target).insert(RoomMesh { mesh });
        }
    }
    Ok(())
}

pub fn room_replaced(trigger: Trigger<OnReplace, Room>, mut commands: Commands) {
    commands
        .entity(trigger.target())
        .try_remove::<RoomContents>();
}

pub fn update_containing_room(
    mut commands: Commands,
    root_q: RootQuery,
    map_q: Query<(Entity, &Map)>,
    item_q: Query<(Entity, &Transform), (With<Pawn>, Without<ContainingRoom>)>,
) -> Result {
    'outer: for (item, transform) in &item_q {
        if root_q.is_descendant_of_root(item) {
            for (map_id, map) in &map_q {
                if root_q.is_descendant_of_root(map_id) {
                    if let Some(room) = map.containing_room(transform.translation.xy()) {
                        info!("found containing room {room}");
                        commands.entity(item).insert(ContainingRoom(room));
                        continue 'outer;
                    }
                }
            }

            warn!("no containing room found for {item}");
        }
    }
    Ok(())
}

#[cfg(feature = "dev")]
pub fn debug_draw_room(room_q: Query<&RoomMesh>, mut gizmos: Gizmos) {
    for room in &room_q {
        for layer in &room.mesh.layers {
            for polygon in &layer.polygons {
                gizmos.linestrip(
                    polygon
                        .vertices
                        .iter()
                        .cycle()
                        .take(polygon.vertices.len() + 1)
                        .map(|&index| layer.vertices[index as usize].coords.extend(0.)),
                    bevy::color::palettes::tailwind::GREEN_300,
                );
            }
        }
    }
}
