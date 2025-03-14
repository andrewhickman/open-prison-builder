use std::{
    f32::consts::{FRAC_PI_2, FRAC_PI_4, PI, SQRT_2, TAU},
    iter,
};

use bevy::{
    math::FloatOrd,
    prelude::*,
    render::{
        mesh::{MeshAabb, PrimitiveTopology},
        primitives::Aabb,
        render_asset::RenderAssetUsages,
    },
    utils::HashSet,
};
use pb_assets::AssetHandles;
use pb_engine::{
    build::Blueprint,
    wall::{self, Vertex, Wall, WallMap},
};
use pb_util::{math::line_intersection, try_modify_component, try_res_s, weak_handle};
use smallvec::SmallVec;

const VERTEX_LOCUS: Vec2 = Vec2::new(0., 0.5 * wall::RADIUS);

#[derive(Debug, Component, PartialEq)]
pub struct VertexGeometry {
    pos: Vec2,
    points: SmallVec<[VertexGeometryPoint; 8]>,
}

#[derive(Copy, Clone, Debug, PartialEq)]
struct VertexGeometryPoint {
    kind: VertexGeometryPointKind,
    point: Vec2,
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum VertexGeometryPointKind {
    Wall(Entity),
    Corner,
}

#[derive(Debug, Default, Component, PartialEq)]
pub struct WallGeometry {
    points: [Vec2; 6],
}

pub const WHITE: Handle<ColorMaterial> = weak_handle!("9644d394-94cd-4fd8-972a-c76026f4d08a");
pub const TRANSLUCENT_WHITE: Handle<ColorMaterial> =
    weak_handle!("8562bf14-56d0-4fa8-ac4a-325b5e2eddef");

#[derive(Debug, Default, Clone, Component)]
pub struct Hidden;

pub fn startup(mut materials: ResMut<Assets<ColorMaterial>>, _assets: Res<AssetHandles>) {
    materials.insert(&WHITE, ColorMaterial::from_color(Color::NONE));
    materials.insert(
        &TRANSLUCENT_WHITE,
        ColorMaterial::from_color(Color::WHITE.with_alpha(0.38)),
    );
    // materials.insert(&WHITE, ColorMaterial::from(assets.brick_image.clone()));
    // materials.insert(
    //     &TRANSLUCENT_WHITE,
    //     ColorMaterial {
    //         color: Color::WHITE.with_alpha(0.38),
    //         alpha_mode: AlphaMode2d::Blend,
    //         texture: Some(assets.brick_image.clone()),
    //     },
    // );
}

pub fn vertex_inserted(
    trigger: Trigger<OnInsert, Vertex>,
    mut commands: Commands,
    transform_q: Query<&Transform>,
    preview_q: Query<&Blueprint>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let color = if preview_q.contains(trigger.entity()) {
        TRANSLUCENT_WHITE.clone()
    } else {
        WHITE.clone()
    };

    let vertex_info =
        VertexGeometry::new(try_res_s!(transform_q.get(trigger.entity())), iter::empty());

    let mesh = vertex_info.mesh();
    let aabb = mesh.compute_aabb().unwrap_or_default();
    let mesh = meshes.add(vertex_info.mesh());

    commands.entity(trigger.entity()).insert((
        vertex_info,
        MeshMaterial2d(color),
        Mesh2d(mesh),
        aabb,
        Visibility::default(),
    ));
}

pub fn wall_inserted(
    trigger: Trigger<OnInsert, Wall>,
    mut commands: Commands,
    preview_q: Query<&Blueprint>,
    meshes: Res<Assets<Mesh>>,
) {
    let color = if preview_q.contains(trigger.entity()) {
        TRANSLUCENT_WHITE.clone()
    } else {
        WHITE.clone()
    };

    let mesh = meshes.reserve_handle();

    commands.entity(trigger.entity()).insert((
        WallGeometry::default(),
        MeshMaterial2d(color.clone()),
        Mesh2d(mesh),
        Aabb::default(),
        Visibility::default(),
    ));
}

pub fn preview_moved(
    vertex_q: Query<Entity, (Changed<Transform>, With<Vertex>, With<Blueprint>)>,
    mut wall_map: ResMut<WallMap>,
) {
    if vertex_q.iter().next().is_some() {
        wall_map.set_changed();
    }
}

pub fn preview_inserted(
    trigger: Trigger<OnInsert, Blueprint>,
    mut commands: Commands,
    mut wall_map: ResMut<WallMap>,
) {
    commands.queue(try_modify_component(
        trigger.entity(),
        |mut color: Mut<MeshMaterial2d<ColorMaterial>>| color.0 = TRANSLUCENT_WHITE,
    ));
    wall_map.set_changed();
}

pub fn preview_removed(
    trigger: Trigger<OnReplace, Blueprint>,
    mut commands: Commands,
    mut wall_map: ResMut<WallMap>,
) {
    commands.queue(try_modify_component(
        trigger.entity(),
        |mut color: Mut<MeshMaterial2d<ColorMaterial>>| color.0 = WHITE,
    ));
    wall_map.set_changed();
}

pub fn hidden_inserted(
    trigger: Trigger<OnInsert, Hidden>,
    mut wall_q: Query<(&mut WallGeometry, &mut Mesh2d, &mut Aabb), With<Wall>>,
    mut wall_map: ResMut<WallMap>,
    assets: Res<Assets<Mesh>>,
) {
    if let Ok((mut info, mut mesh, mut aabb)) = wall_q.get_mut(trigger.entity()) {
        *info = Default::default();
        mesh.0 = assets.reserve_handle();
        *aabb = Default::default();

        wall_map.set_changed();
    }
}

pub fn hidden_removed(
    trigger: Trigger<OnReplace, Hidden>,
    wall_q: Query<Entity, With<Wall>>,
    mut wall_map: ResMut<WallMap>,
) {
    if wall_q.contains(trigger.entity()) {
        wall_map.set_changed();
    }
}

pub fn update_wall(
    mut assets: ResMut<Assets<Mesh>>,
    wall_map: Res<WallMap>,
    transform_q: Query<&Transform, With<Vertex>>,
    hidden_q: Query<Entity, With<Hidden>>,
    mut vertex_q: Query<
        (Entity, &Transform, &mut VertexGeometry, &Mesh2d, &mut Aabb),
        With<Vertex>,
    >,
    mut wall_mesh_q: Query<
        (&Wall, &mut WallGeometry, &Mesh2d, &mut Aabb, Has<Hidden>),
        Without<Vertex>,
    >,
) {
    let mut updated_walls = HashSet::new();

    for (id, transform, mut info, mesh, mut aabb) in vertex_q.iter_mut() {
        let new_info = VertexGeometry::new(
            transform,
            wall_map
                .get(id)
                .filter(|entry| !hidden_q.contains(entry.wall))
                .filter_map(|entry| transform_q.get(entry.end).ok().map(|pos| (entry.wall, pos))),
        );

        if info.set_if_neq(new_info) {
            let new_mesh = info.mesh();
            *aabb = new_mesh.compute_aabb().unwrap_or_default();
            assets.insert(mesh.id(), new_mesh);

            updated_walls.extend(wall_map.get(id).map(|entry| entry.wall));
        }
    }

    for id in updated_walls {
        let (wall, mut info, mesh, mut aabb, hidden) = try_res_s!(wall_mesh_q.get_mut(id));
        if hidden {
            continue;
        }

        let [(_, _, start_info, _, _), (_, _, end_info, _, _)] = vertex_q.many(wall.vertices());

        let new_info = WallGeometry::new(id, start_info, end_info);
        if info.set_if_neq(new_info) {
            let new_mesh = info.mesh();
            *aabb = new_mesh.compute_aabb().unwrap_or_default();
            assets.insert(mesh.id(), new_mesh);
        }
    }
}

impl VertexGeometry {
    fn new<'a>(
        transform: &Transform,
        walls: impl Iterator<Item = (Entity, &'a Transform)>,
    ) -> Self {
        let start = transform.translation.xy();

        let mut angles: SmallVec<[(VertexGeometryPointKind, f32); 4]> = walls
            .map(|(id, end)| {
                (
                    VertexGeometryPointKind::Wall(id),
                    (end.translation.xy() - start).to_angle(),
                )
            })
            .collect();
        angles.sort_by_key(|&(_, angle)| FloatOrd(angle));

        if angles.is_empty() {
            angles.push((VertexGeometryPointKind::Corner, 0.));
        }

        let mut points: SmallVec<[VertexGeometryPoint; 8]> = default();
        for (index, &(wall, a2)) in angles.iter().enumerate() {
            if index == 0 {
                let a1 = wrapping_idx(&angles, index, -1).1;
                points.extend(vertex_intersections(a1, a2).map(VertexGeometryPoint::corner));
            }
            let i1 = points[points.len() - 1].point;

            let mid = points.len();

            let i3 = if index != (angles.len() - 1) {
                let a3 = wrapping_idx(&angles, index, 1).1;
                points.extend(vertex_intersections(a2, a3).map(VertexGeometryPoint::corner));
                points[mid].point
            } else {
                points[0].point
            };

            if let VertexGeometryPointKind::Wall(wall) = wall {
                let i2 = line_intersection(VERTEX_LOCUS, Vec2::from_angle(a2), i1, i3 - i1)
                    .unwrap_or_else(|| i1.midpoint(i3));
                points.insert(mid, VertexGeometryPoint::wall(wall, i2));
            }
        }

        VertexGeometry { pos: start, points }
    }

    fn wall_intersection(&self, id: Entity) -> Option<(Vec2, Vec2, Vec2)> {
        let index = self
            .points
            .iter()
            .position(|p| p.kind == VertexGeometryPointKind::Wall(id))?;
        let i1 = wrapping_idx(&self.points, index, -1).point;
        let i2 = wrapping_idx(&self.points, index, 0).point;
        let i3 = wrapping_idx(&self.points, index, 1).point;

        Some((self.pos + i1, self.pos + i2, self.pos + i3))
    }

    fn mesh(&self) -> Mesh {
        let mut vertices = Vec::with_capacity(self.points.len() * 3);
        for (i, i1) in self.points.iter().enumerate() {
            let i2 = wrapping_idx(&self.points, i, 1);
            vertices.extend([i1.to_vec3(), VERTEX_LOCUS.extend(0.), i2.to_vec3()]);
        }

        Mesh::new(
            PrimitiveTopology::TriangleList,
            RenderAssetUsages::RENDER_WORLD,
        )
        .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, vertices)
    }
}

impl WallGeometry {
    fn new(id: Entity, start: &VertexGeometry, end: &VertexGeometry) -> Self {
        let pos = start.pos.midpoint(end.pos);
        let (start1, start2, start3) = start.wall_intersection(id).unwrap();
        let (end1, end2, end3) = end.wall_intersection(id).unwrap();

        WallGeometry {
            points: [
                start1 - pos,
                start2 - pos,
                start3 - pos,
                end1 - pos,
                end2 - pos,
                end3 - pos,
            ],
        }
    }

    fn mesh(&self) -> Mesh {
        Mesh::new(
            PrimitiveTopology::TriangleStrip,
            RenderAssetUsages::RENDER_WORLD,
        )
        .with_inserted_attribute(
            Mesh::ATTRIBUTE_POSITION,
            vec![
                self.points[0].extend(0.),
                self.points[5].extend(0.),
                self.points[1].extend(0.),
                self.points[4].extend(0.),
                self.points[2].extend(0.),
                self.points[3].extend(0.),
            ],
        )
    }
}

impl VertexGeometryPoint {
    fn wall(wall: Entity, point: Vec2) -> Self {
        VertexGeometryPoint {
            point,
            kind: VertexGeometryPointKind::Wall(wall),
        }
    }

    fn corner(point: Vec2) -> Self {
        VertexGeometryPoint {
            point,
            kind: VertexGeometryPointKind::Corner,
        }
    }

    fn to_vec3(self) -> Vec3 {
        self.point.extend(0.)
    }
}

fn vertex_intersections(mut a1: f32, mut a2: f32) -> impl Iterator<Item = Vec2> {
    let mut da = angle_delta(a1, a2);
    let reflex = da >= PI;

    let mut result = SmallVec::<[Vec2; 8]>::new();

    let threshold = if reflex { FRAC_PI_2 } else { PI };
    while da >= threshold {
        result.insert_from_slice(
            result.len() / 2,
            &[
                right_angle_intersection(a1 + FRAC_PI_4),
                right_angle_intersection(a2 - FRAC_PI_4),
            ],
        );

        a1 += FRAC_PI_2;
        a2 -= FRAC_PI_2;
        da -= PI;
    }

    if da > 0. {
        let mid = a1 + da / 2.;
        if reflex {
            da = PI - da;
        }

        result.insert(result.len() / 2, angle_intersection(mid, da / 2.));
    }

    result.into_iter()
}

fn right_angle_intersection(a: f32) -> Vec2 {
    Vec2::from_angle(a) * wall::RADIUS * SQRT_2
}

fn angle_intersection(mid: f32, da: f32) -> Vec2 {
    Vec2::from_angle(mid) * wall::RADIUS / f32::sin(da)
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
