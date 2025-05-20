use std::{
    f32::consts::{FRAC_PI_2, FRAC_PI_4, SQRT_2, TAU},
    sync::Arc,
};

use bevy::{
    math::FloatOrd,
    platform::collections::{HashMap, HashSet},
    prelude::*,
};
use polyanya::{
    Mesh, Path, Triangulation,
    geo::{
        Coord, LineString, Polygon,
        winding_order::{Winding, WindingOrder},
    },
};
use smallvec::SmallVec;
use spade::{
    CdtEdge, ConstrainedDelaunayTriangulation, Triangulation as _,
    handles::{DirectedEdgeHandle, FixedDirectedEdgeHandle, FixedVertexHandle},
};

use crate::{
    map::{Corner, FaceData, Map, UndirectedEdgeData, VertexData, wall},
    pawn,
    root::RootQuery,
};

#[derive(Debug, Default, Component)]
pub struct RoomMesh {
    mesh: Arc<Mesh>,
}

const RADIUS: f32 = wall::RADIUS + pawn::RADIUS;

#[derive(Debug)]
struct CornerGeometry {
    pos: Vec2,
    points: SmallVec<[CornerGeometryPoint; 4]>,
}

#[derive(Copy, Clone, Debug, PartialEq)]
struct CornerGeometryPoint {
    kind: CornerGeometryPointKind,
    point: Vec2,
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum CornerGeometryPointKind {
    Edge(Entity),
    Corner,
}

pub fn update_mesh(
    mut commands: Commands,
    mut map_q: Query<&Map, Changed<Map>>,
    corner_q: Query<&Corner>,
    root_q: RootQuery,
) -> Result {
    for map in &mut map_q {
        if !root_q.is_descendant_of_root(map.id()) {
            continue;
        }

        let corner_geos: HashMap<FixedVertexHandle, CornerGeometry> = map
            .corners()
            .map(|entity| {
                let corner = corner_q.get(entity.id())?;
                let geometry = CornerGeometry::new(
                    corner,
                    map.corner_walls(corner).filter_map(|(wall, end_corner)| {
                        corner_q
                            .get(end_corner)
                            .ok()
                            .map(|end_corner| (wall, end_corner))
                    }),
                );

                Ok((corner.vertex, geometry))
            })
            .collect::<Result<_>>()?;

        let mut edges: HashSet<FixedDirectedEdgeHandle> = map
            .triangulation
            .directed_edges()
            .filter(|edge| edge.is_constraint_edge())
            .map(|edge| edge.fix())
            .collect();

        let mut polygons: HashMap<Vec<Entity>, (Option<LineString<f32>>, Vec<LineString<f32>>)> =
            default();

        while let Some(&start) = edges.iter().next() {
            edges.remove(&start);

            let (line_string, rooms) =
                interior_polygon(&mut edges, &map.triangulation, &corner_geos, start)?;

            let (exterior, interior) = polygons.entry(rooms).or_default();
            match line_string.winding_order().unwrap() {
                WindingOrder::CounterClockwise => interior.push(line_string),
                WindingOrder::Clockwise => {
                    assert!(exterior.is_none());
                    *exterior = Some(line_string);
                }
            }
        }

        for (rooms, (exterior, interiors)) in polygons {
            let exterior = match exterior {
                Some(exterior) => exterior,
                None => map
                    .triangulation
                    .convex_hull()
                    .map(|edge| edge.from().data().position.to_array())
                    .collect(),
            };

            let polygon = Polygon::new(exterior, interiors);
            let triangulation = Triangulation::from_geo_polygon(polygon);
            let mut mesh = triangulation.as_navmesh();

            mesh.merge_polygons();
            mesh.bake();

            let mesh = Arc::new(mesh);

            for room in rooms {
                commands
                    .entity(room)
                    .insert(RoomMesh { mesh: mesh.clone() });
            }
        }
    }

    Ok(())
}

impl RoomMesh {
    pub fn path(&self, from: Vec2, to: Vec2) -> Option<Path> {
        self.mesh.path(from, to)
    }

    pub fn mesh(&self) -> &Mesh {
        &self.mesh
    }
}

fn interior_polygon(
    edges: &mut HashSet<FixedDirectedEdgeHandle>,
    triangulation: &ConstrainedDelaunayTriangulation<VertexData, (), UndirectedEdgeData, FaceData>,
    corner_geos: &HashMap<FixedVertexHandle, CornerGeometry>,
    start: FixedDirectedEdgeHandle,
) -> Result<(LineString<f32>, Vec<Entity>)> {
    let mut coords = Vec::new();
    let mut rooms = Vec::new();

    let mut current = triangulation.directed_edge(start);
    rooms.push(current.rev().face().data().room.unwrap().id());
    loop {
        let next = next_edge(current);
        add_corner_coords(&mut coords, current, next, corner_geos);
        rooms.push(next.rev().face().data().room.unwrap().id());

        if !edges.remove(&next.fix()) {
            debug_assert_eq!(next.fix(), start);
            break;
        }

        current = next;
    }

    rooms.sort_unstable();
    rooms.dedup();

    let mut line_string = LineString::new(coords);
    line_string.close();

    Ok((line_string, rooms))
}

fn add_corner_coords(
    coords: &mut Vec<Coord<f32>>,
    start: DirectedEdgeHandle<'_, VertexData, (), CdtEdge<UndirectedEdgeData>, FaceData>,
    end: DirectedEdgeHandle<'_, VertexData, (), CdtEdge<UndirectedEdgeData>, FaceData>,
    corner_geos: &HashMap<FixedVertexHandle, CornerGeometry>,
) {
    assert_eq!(start.to(), end.from());
    let corner = &corner_geos[&start.to().fix()];
    let start_wall = start.as_undirected().data().data().wall.unwrap().id();
    let end_wall = end.as_undirected().data().data().wall.unwrap().id();

    corner.wall_intersections(coords, start_wall, end_wall);
}

fn next_edge<'a>(
    start: DirectedEdgeHandle<'a, VertexData, (), CdtEdge<UndirectedEdgeData>, FaceData>,
) -> DirectedEdgeHandle<'a, VertexData, (), CdtEdge<UndirectedEdgeData>, FaceData> {
    let mut iter = start.rev();
    loop {
        iter = iter.ccw();
        if iter.is_constraint_edge() {
            return iter;
        }
    }
}

impl CornerGeometry {
    fn new<'a>(start: &Corner, walls: impl Iterator<Item = (Entity, &'a Corner)>) -> Self {
        let start = start.position();

        let mut angles: SmallVec<[(Entity, f32); 4]> = walls
            .map(|(id, end)| (id, (end.position() - start).to_angle()))
            .collect();
        angles.sort_by_key(|&(_, angle)| FloatOrd(angle));

        let mut points: SmallVec<[CornerGeometryPoint; 4]> = default();
        for (index, &(wall, a2)) in angles.iter().enumerate() {
            if index == 0 {
                let a1 = wrapping_idx(&angles, index, -1).1;
                points.extend(corner_intersections(a1, a2).map(CornerGeometryPoint::corner));
            }

            points.push(CornerGeometryPoint::wall(wall));

            if index != (angles.len() - 1) {
                let a3 = wrapping_idx(&angles, index, 1).1;
                points.extend(corner_intersections(a2, a3).map(CornerGeometryPoint::corner));
            }
        }

        CornerGeometry { pos: start, points }
    }

    fn wall_intersections(&self, result: &mut Vec<Coord<f32>>, start: Entity, end: Entity) {
        let start = self
            .points
            .iter()
            .position(|p| p.kind == CornerGeometryPointKind::Edge(start))
            .unwrap();
        let end = self
            .points
            .iter()
            .position(|p| p.kind == CornerGeometryPointKind::Edge(end))
            .unwrap();

        let mut i = (start + 1) % self.points.len();
        while i != end {
            result.push((self.pos + self.points[i].point).to_array().into());
            i = (i + 1) % self.points.len();
        }
    }
}

impl CornerGeometryPoint {
    fn wall(wall: Entity) -> Self {
        CornerGeometryPoint {
            point: Vec2::ZERO,
            kind: CornerGeometryPointKind::Edge(wall),
        }
    }

    fn corner(point: Vec2) -> Self {
        CornerGeometryPoint {
            point,
            kind: CornerGeometryPointKind::Corner,
        }
    }
}

fn corner_intersections(a1: f32, a2: f32) -> impl Iterator<Item = Vec2> {
    let da = angle_delta(a1, a2);

    let mut result = SmallVec::<[Vec2; 2]>::new();

    if da > 3. * FRAC_PI_2 {
        result.extend_from_slice(&[
            right_angle_intersection(a1 + 3. * FRAC_PI_4),
            right_angle_intersection(a2 - 3. * FRAC_PI_4),
        ]);
    } else {
        let mid = a1 + da / 2.;
        result.push(angle_intersection(mid, da / 2.));
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
