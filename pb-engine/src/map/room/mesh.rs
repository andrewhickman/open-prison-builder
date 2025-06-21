use std::f32::consts::{FRAC_PI_2, FRAC_PI_4, SQRT_2, TAU};

use bevy::{ecs::entity::EntityHashMap, math::FloatOrd, prelude::*};
use polyanya::{
    Coords, Mesh, Triangulation,
    geo::{
        Area, BooleanOps, Closest, ClosestPoint, Coord, Point, Polygon, StitchTriangles, Triangle,
        unary_union,
    },
};
use smallvec::SmallVec;
use spade::Triangulation as _;

use crate::{
    map::{
        Corner, Map,
        door::Door,
        room::{Room, path::RoomPath},
        wall::Wall,
    },
    pawn::Pawn,
    root::ChildOfRoot,
};

#[derive(Debug, Default, Component)]
pub struct RoomMesh {
    mesh: Mesh,
    polygon: Option<Polygon<f32>>,
}

const RADIUS: f32 = Wall::RADIUS + Pawn::RADIUS;

#[derive(Debug)]
struct CornerGeometry {
    center: Vec2,
    points: SmallVec<[CornerGeometryPoint; 4]>,
}

#[derive(Copy, Clone, Debug, PartialEq)]
struct CornerGeometryPoint {
    kind: CornerGeometryPointKind,
    point: Vec2,
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum CornerGeometryPointKind {
    Wall(Entity),
    Corner,
}

pub fn update(
    mut map_q: Query<&Map, (Changed<Map>, With<ChildOfRoot>)>,
    corner_q: Query<&Corner>,
    wall_q: Query<&Wall>,
    door_q: Query<&Door>,
    mut room_q: Query<(&Room, &mut RoomMesh)>,
) -> Result {
    for map in &mut map_q {
        let mut corners = EntityHashMap::new();
        for entity in map.corners() {
            let corner = corner_q.get(entity.id())?;
            let geometry = CornerGeometry::new(
                corner,
                map.corner_walls(corner)
                    .map(|(wall, end_corner)| Ok((wall, corner_q.get(end_corner)?))),
            )?;

            corners.insert(entity.id(), geometry);
        }

        let mut obstacles = Vec::with_capacity(map.triangulation.num_constraints() * 3);

        for (_, corner) in &corners {
            obstacles.push(Polygon::new(
                corner
                    .points
                    .iter()
                    .map(|point| to_coord(point.point))
                    .collect(),
                vec![],
            ));
        }

        for entity in map.walls() {
            let wall = wall_q.get(entity.id())?;

            let start_points = corners[&wall.start()].wall_intersections(entity.id())?;
            let end_points = corners[&wall.end()].wall_intersections(entity.id())?;

            if door_q.contains(entity.id()) {
                let wall_half_len = wall.length() / 2.;
                let door_start_points = [
                    wall.isometry() * Vec2::new(-wall_half_len + RADIUS, -RADIUS),
                    wall.isometry() * Vec2::new(-wall_half_len + RADIUS, RADIUS),
                ];
                let door_end_points = [
                    wall.isometry() * Vec2::new(wall_half_len - RADIUS, RADIUS),
                    wall.isometry() * Vec2::new(wall_half_len - RADIUS, -RADIUS),
                ];

                obstacles.push(Polygon::new(
                    start_points
                        .into_iter()
                        .chain(door_start_points)
                        .map(to_coord)
                        .collect(),
                    vec![],
                ));
                obstacles.push(Polygon::new(
                    end_points
                        .into_iter()
                        .chain(door_end_points)
                        .map(to_coord)
                        .collect(),
                    vec![],
                ));
            } else {
                obstacles.push(Polygon::new(
                    start_points
                        .into_iter()
                        .chain(end_points)
                        .map(to_coord)
                        .collect(),
                    vec![],
                ));
            }
        }

        let obstacles = unary_union(&obstacles);

        for entity in map.rooms_deduped() {
            let (room, mut room_mesh) = room_q.get_mut(entity.id())?;

            let triangles: Vec<Triangle<f32>> = room
                .faces()
                .iter()
                .filter_map(|face| {
                    face.as_inner().map(|inner_face| {
                        map.triangulation
                            .face(inner_face)
                            .vertices()
                            .map(|vertex| to_coord(vertex.data().position))
                            .into()
                    })
                })
                .collect();
            let polygon = triangles
                .stitch_triangulation()?
                .difference(&obstacles)
                .into_iter()
                .max_by_key(|island| FloatOrd(island.unsigned_area()));

            if room_mesh.polygon != polygon {
                let mesh = polygon
                    .clone()
                    .map(|polygon| {
                        let layer = Triangulation::from_geo_polygon(polygon).as_layer();
                        let mut mesh = Mesh {
                            layers: vec![layer],
                            search_delta: RADIUS / 2.,
                            search_steps: 2,
                        };

                        // TODO: https://github.com/vleue/polyanya/issues/99
                        // mesh.merge_polygons();
                        mesh.bake();
                        mesh
                    })
                    .unwrap_or_default();

                *room_mesh = RoomMesh { polygon, mesh };
            }
        }
    }

    Ok(())
}

impl RoomMesh {
    pub fn path(&self, from: Vec2, to: Vec2) -> Option<RoomPath> {
        let from = self.closest_point(from)?;
        self.path_from(from, to)
    }

    pub fn mesh(&self) -> &Mesh {
        &self.mesh
    }

    fn path_from(&self, from: Coords, to: Vec2) -> Option<RoomPath> {
        let to = self.closest_point(to)?;
        let path = self.mesh.path(from, to)?;
        Some(RoomPath::new(from.position(), path))
    }

    fn closest_point(&self, point: Vec2) -> Option<Coords> {
        if let Some(coords) = self.mesh.get_closest_point(point) {
            return Some(coords);
        }

        let polygon = self.polygon.as_ref()?;
        match polygon.closest_point(&Point::new(point.x, point.y)) {
            Closest::Intersection(closest) | Closest::SinglePoint(closest) => {
                if let Some(coords) = self
                    .mesh
                    .get_closest_point(Vec2::new(closest.x(), closest.y()))
                {
                    Some(coords)
                } else if let Some(coords) = self
                    .mesh
                    .get_closest_point_towards(point, Vec2::new(closest.x(), closest.y()))
                {
                    Some(coords)
                } else {
                    error!(
                        "closest point {closest:?} for target point {point:?} was not found in the mesh"
                    );
                    None
                }
            }
            Closest::Indeterminate => {
                error!("indeterminate closest point to {point:?} on polygon");
                None
            }
        }
    }
}

impl CornerGeometry {
    fn new<'a>(
        start: &Corner,
        walls: impl Iterator<Item = Result<(Entity, &'a Corner)>>,
    ) -> Result<Self> {
        let pos = start.position();

        let mut angles: SmallVec<[(Entity, f32); 4]> = walls
            .map(|res| res.map(|(id, end)| (id, (end.position() - pos).to_angle())))
            .collect::<Result<_>>()?;
        angles.sort_by_key(|&(_, angle)| FloatOrd(angle));

        let mut points: SmallVec<[CornerGeometryPoint; 4]> = default();
        for (index, &(wall, a2)) in angles.iter().enumerate() {
            if index == 0 {
                let a1 = wrapping_idx(&angles, index, -1).1;
                points.extend(corner_intersections(pos, a1, a2).map(CornerGeometryPoint::corner));
            }

            points.push(CornerGeometryPoint::wall(
                pos + Vec2::from_angle(a2) * RADIUS,
                wall,
            ));

            if index != (angles.len() - 1) {
                let a3 = wrapping_idx(&angles, index, 1).1;
                points.extend(corner_intersections(pos, a2, a3).map(CornerGeometryPoint::corner));
            }
        }

        Ok(CornerGeometry {
            points,
            center: pos,
        })
    }

    fn wall_intersections(&self, wall: Entity) -> Result<[Vec2; 3]> {
        let index = self
            .points
            .iter()
            .position(|p| p.kind == CornerGeometryPointKind::Wall(wall))
            .ok_or("wall not found")?;

        Ok([
            wrapping_idx(&self.points, index, 1).point,
            self.center,
            wrapping_idx(&self.points, index, -1).point,
        ])
    }
}

impl CornerGeometryPoint {
    fn wall(point: Vec2, wall: Entity) -> Self {
        CornerGeometryPoint {
            point,
            kind: CornerGeometryPointKind::Wall(wall),
        }
    }

    fn corner(point: Vec2) -> Self {
        CornerGeometryPoint {
            point,
            kind: CornerGeometryPointKind::Corner,
        }
    }
}

fn corner_intersections(pos: Vec2, a1: f32, a2: f32) -> impl Iterator<Item = Vec2> {
    let da = angle_delta(a1, a2);

    let mut result = SmallVec::<[Vec2; 2]>::new();

    if da > 3. * FRAC_PI_2 {
        result.extend_from_slice(&[
            pos + right_angle_intersection(a1 + 3. * FRAC_PI_4),
            pos + right_angle_intersection(a2 - 3. * FRAC_PI_4),
        ]);
    } else {
        let mid = a1 + da / 2.;
        result.push(pos + angle_intersection(mid, da / 2.));
    }

    result.into_iter()
}

fn right_angle_intersection(a: f32) -> Vec2 {
    Vec2::from_angle(a) * RADIUS * SQRT_2
}

fn angle_intersection(mid: f32, da: f32) -> Vec2 {
    Vec2::from_angle(mid) * RADIUS / f32::sin(da)
}

fn angle_delta(a1: f32, a2: f32) -> f32 {
    if a1 == a2 {
        TAU
    } else {
        (a2 - a1).rem_euclid(TAU)
    }
}

fn wrapping_idx<T>(slice: &[T], index: usize, offset: isize) -> &T {
    &slice[(index as isize + offset).rem_euclid(slice.len() as isize) as usize]
}

fn to_coord(point: Vec2) -> Coord<f32> {
    Coord {
        x: point.x,
        y: point.y,
    }
}
