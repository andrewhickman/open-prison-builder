use avian2d::parry::shape::Capsule;
use bevy::{ecs::entity::EntityHashSet, platform::collections::HashMap, prelude::*};
use pb_util::event::Inserted;
use polyanya::{Mesh, Path, Triangulation, Trimesh};
use spade::Triangulation as _;

use crate::{
    map::{Map, Room, wall},
    pawn::{self, Pawn},
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

            let mesh = Mesh::try_from(trimesh)?;
            let mut triangulation = Triangulation::from_mesh(&mesh, 0);

            triangulation.set_agent_radius(wall::RADIUS + pawn::RADIUS);
            // triangulation.set_agent_radius_simplification(0.05);
            triangulation.set_agent_radius_segments(3);

            triangulation.add_obstacles(
                room.faces
                    .iter()
                    .flat_map(|&face| map.triangulation.face(face).adjacent_edges())
                    .filter(|edge| edge.is_constraint_edge())
                    .map(|edge| {
                        let start = edge.from().data().position;
                        let end = edge.to().data().position;
                        Capsule::new(start.into(), end.into(), wall::RADIUS)
                            .to_polyline(5)
                            .into_iter()
                            .map(Vec2::from)
                    }),
            );

            let mut mesh = triangulation.as_navmesh();

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

impl RoomMesh {
    pub fn path(&self, from: Vec2, to: Vec2) -> Option<Path> {
        self.mesh.path(from, to)
    }
}
