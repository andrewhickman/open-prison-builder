use std::{
    f32::consts::{FRAC_PI_2, FRAC_PI_4, SQRT_2, TAU},
    iter,
};

use bevy::{
    asset::weak_handle,
    ecs::entity::EntityHashSet,
    math::{Affine2, FloatOrd},
    prelude::*,
    render::{
        mesh::{Indices, MeshAabb, PrimitiveTopology},
        primitives::Aabb,
        render_asset::RenderAssetUsages,
    },
    sprite::AlphaMode2d,
};
use pb_assets::AssetHandles;
use pb_engine::{
    build::Blueprint,
    wall::{self, Vertex, Wall, WallMap},
};
use pb_util::try_modify_component;
use smallvec::SmallVec;

const VERTEX_LOCUS: Vec2 = Vec2::new(0.25 * wall::RADIUS, 0.5 * wall::RADIUS);

const TEXTURE_TOP: f32 = 0.0;
const TEXTURE_BOTTOM: f32 = 1.0;

#[derive(Debug, Component, PartialEq)]
pub struct VertexGeometry {
    pos: Vec2,
    points: SmallVec<[VertexGeometryPoint; 4]>,
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
    lens: [f32; 6],
}

pub const WHITE: Handle<ColorMaterial> = weak_handle!("9644d394-94cd-4fd8-972a-c76026f4d08a");
pub const TRANSLUCENT_WHITE: Handle<ColorMaterial> =
    weak_handle!("8562bf14-56d0-4fa8-ac4a-325b5e2eddef");

#[derive(Debug, Default, Clone, Component)]
pub struct Hidden;

pub fn startup(mut materials: ResMut<Assets<ColorMaterial>>, assets: Res<AssetHandles>) {
    materials.insert(&WHITE, ColorMaterial::from(assets.brick_image.clone()));
    materials.insert(
        &TRANSLUCENT_WHITE,
        ColorMaterial {
            color: Color::WHITE.with_alpha(0.38),
            alpha_mode: AlphaMode2d::Blend,
            texture: Some(assets.brick_image.clone()),
            uv_transform: Affine2::IDENTITY,
        },
    );
}

pub fn vertex_inserted(
    trigger: Trigger<OnInsert, Vertex>,
    mut commands: Commands,
    transform_q: Query<&Transform>,
    preview_q: Query<&Blueprint>,
    mut meshes: ResMut<Assets<Mesh>>,
) -> Result {
    let color = if preview_q.contains(trigger.target()) {
        TRANSLUCENT_WHITE.clone()
    } else {
        WHITE.clone()
    };

    let vertex_info = VertexGeometry::new(transform_q.get(trigger.target())?, iter::empty());

    let mesh = vertex_info.mesh();
    let aabb = mesh
        .as_ref()
        .and_then(|m| m.compute_aabb())
        .unwrap_or_default();
    let mesh = if let Some(mesh) = mesh {
        meshes.add(mesh)
    } else {
        default()
    };

    commands.entity(trigger.target()).insert((
        vertex_info,
        MeshMaterial2d(color),
        Mesh2d(mesh),
        aabb,
        Visibility::default(),
    ));
    Ok(())
}

pub fn wall_inserted(
    trigger: Trigger<OnInsert, Wall>,
    mut commands: Commands,
    preview_q: Query<&Blueprint>,
    meshes: Res<Assets<Mesh>>,
) {
    let color = if preview_q.contains(trigger.target()) {
        TRANSLUCENT_WHITE.clone()
    } else {
        WHITE.clone()
    };

    let mesh = meshes.reserve_handle();

    commands.entity(trigger.target()).insert((
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
        trigger.target(),
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
        trigger.target(),
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
    if let Ok((mut info, mut mesh, mut aabb)) = wall_q.get_mut(trigger.target()) {
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
    if wall_q.contains(trigger.target()) {
        wall_map.set_changed();
    }
}

pub fn update_wall(
    mut meshes: ResMut<Assets<Mesh>>,
    wall_map: Res<WallMap>,
    transform_q: Query<&Transform, With<Vertex>>,
    hidden_q: Query<Entity, With<Hidden>>,
    mut vertex_q: Query<
        (
            Entity,
            &Transform,
            &mut VertexGeometry,
            &mut Mesh2d,
            &mut Aabb,
        ),
        With<Vertex>,
    >,
    mut wall_mesh_q: Query<
        (
            &Wall,
            &mut WallGeometry,
            &mut Mesh2d,
            &mut Aabb,
            Has<Hidden>,
        ),
        Without<Vertex>,
    >,
) -> Result {
    let mut updated_walls = EntityHashSet::new();

    for (id, transform, mut info, mut mesh, mut aabb) in vertex_q.iter_mut() {
        let new_info = VertexGeometry::new(
            transform,
            wall_map
                .get(id)
                .filter(|entry| !hidden_q.contains(entry.wall))
                .filter_map(|entry| transform_q.get(entry.end).ok().map(|pos| (entry.wall, pos))),
        );

        if info.set_if_neq(new_info) {
            let new_mesh = info.mesh();
            *aabb = new_mesh
                .as_ref()
                .and_then(|m| m.compute_aabb())
                .unwrap_or_default();

            mesh.0 = if let Some(new_mesh) = new_mesh {
                meshes.add(new_mesh)
            } else {
                default()
            };

            updated_walls.extend(wall_map.get(id).map(|entry| entry.wall));
        }
    }

    for id in updated_walls {
        let (wall, mut info, mut mesh, mut aabb, hidden) = wall_mesh_q.get_mut(id)?;
        if hidden {
            continue;
        }

        let [(_, _, start_info, _, _), (_, _, end_info, _, _)] =
            vertex_q.get_many(wall.vertices())?;

        let new_info = WallGeometry::new(id, start_info, end_info)?;
        if info.set_if_neq(new_info) {
            let new_mesh = info.mesh();
            *aabb = new_mesh.compute_aabb().unwrap_or_default();
            mesh.0 = meshes.add(new_mesh);
        }
    }
    Ok(())
}

impl VertexGeometry {
    fn new<'a>(
        transform: &Transform,
        walls: impl Iterator<Item = (Entity, &'a Transform)>,
    ) -> Self {
        let start = transform.translation.xy();

        let mut angles: SmallVec<[(Option<Entity>, f32); 4]> = walls
            .map(|(id, end)| (Some(id), (end.translation.xy() - start).to_angle()))
            .collect();
        angles.sort_by_key(|&(_, angle)| FloatOrd(angle));

        let mut points: SmallVec<[VertexGeometryPoint; 4]> = default();
        for (index, &(wall, a2)) in angles.iter().enumerate() {
            if index == 0 {
                let a1 = wrapping_idx(&angles, index, -1).1;
                points.extend(vertex_intersections(a1, a2).map(VertexGeometryPoint::corner));
            }

            if let Some(wall) = wall {
                points.push(VertexGeometryPoint::wall(wall));
            }

            if index != (angles.len() - 1) {
                let a3 = wrapping_idx(&angles, index, 1).1;
                points.extend(vertex_intersections(a2, a3).map(VertexGeometryPoint::corner));
            }
        }

        if points.is_empty() {
            points.extend_from_slice(&[
                VertexGeometryPoint::corner(right_angle_intersection(FRAC_PI_4)),
                VertexGeometryPoint::corner(right_angle_intersection(3. * FRAC_PI_4)),
                VertexGeometryPoint::corner(right_angle_intersection(5. * FRAC_PI_4)),
                VertexGeometryPoint::corner(right_angle_intersection(7. * FRAC_PI_4)),
            ]);
        }

        VertexGeometry { pos: start, points }
    }

    fn wall_intersection(&self, id: Entity, pos: Vec2) -> Option<(Vec2, Vec2, Vec2)> {
        let offset = self.pos - pos;
        let index = self
            .points
            .iter()
            .position(|p| p.kind == VertexGeometryPointKind::Wall(id))?;
        let i1 = wrapping_idx(&self.points, index, -1).point;
        let i2 = wrapping_idx(&self.points, index, 0).point;
        let i3 = wrapping_idx(&self.points, index, 1).point;

        Some((i1 + offset, i2 + offset, i3 + offset))
    }

    fn mesh(&self) -> Option<Mesh> {
        let mut vertices = Vec::new();
        let mut uvs = Vec::new();
        for (i, i1) in self.points.iter().enumerate() {
            let i2 = wrapping_idx(&self.points, i, 1);

            if i1.is_wall() || i2.is_wall() {
                continue;
            }

            vertices.extend([i1.to_vec3(), VERTEX_LOCUS.extend(0.), i2.to_vec3()]);

            let di = i2.point - i1.point;
            let base_len = di.length();
            let locus_len = (VERTEX_LOCUS - i1.point).project_onto(di).length();

            uvs.extend([
                Vec2::new(0.0, TEXTURE_BOTTOM),
                Vec2::new(locus_len, TEXTURE_TOP),
                Vec2::new(base_len, TEXTURE_BOTTOM),
            ]);
        }

        if vertices.is_empty() {
            None
        } else {
            Some(
                Mesh::new(
                    PrimitiveTopology::TriangleList,
                    RenderAssetUsages::RENDER_WORLD,
                )
                .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, vertices)
                .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, uvs),
            )
        }
    }
}

impl WallGeometry {
    fn new(id: Entity, start: &VertexGeometry, end: &VertexGeometry) -> Result<Self> {
        let pos = start.pos.midpoint(end.pos);
        let (start1, start2, start3) = start
            .wall_intersection(id, pos)
            .ok_or("wall intersection not found")?;
        let (end1, end2, end3) = end
            .wall_intersection(id, pos)
            .ok_or("wall intersection not found")?;

        Ok(WallGeometry {
            points: [start1, start2, start3, end1, end2, end3],
            lens: [
                start1.project_onto(start2).length(),
                start2.length(),
                start3.project_onto(start2).length(),
                end1.project_onto(end2).length(),
                end2.length(),
                end3.project_onto(end2).length(),
            ],
        })
    }

    fn mesh(&self) -> Mesh {
        Mesh::new(
            PrimitiveTopology::TriangleList,
            RenderAssetUsages::RENDER_WORLD,
        )
        .with_inserted_attribute(
            Mesh::ATTRIBUTE_POSITION,
            self.points.iter().map(|p| p.extend(0.)).collect::<Vec<_>>(),
        )
        .with_inserted_attribute(
            Mesh::ATTRIBUTE_UV_0,
            vec![
                Vec2::new(-self.lens[0], TEXTURE_BOTTOM),
                Vec2::new(-self.lens[1], TEXTURE_TOP),
                Vec2::new(-self.lens[2], TEXTURE_BOTTOM),
                Vec2::new(self.lens[3], TEXTURE_BOTTOM),
                Vec2::new(self.lens[4], TEXTURE_TOP),
                Vec2::new(self.lens[5], TEXTURE_BOTTOM),
            ],
        )
        .with_inserted_indices(Indices::U16(vec![0, 1, 5, 1, 5, 4, 1, 2, 4, 2, 4, 3]))
    }
}

impl VertexGeometryPoint {
    fn wall(wall: Entity) -> Self {
        VertexGeometryPoint {
            point: VERTEX_LOCUS,
            kind: VertexGeometryPointKind::Wall(wall),
        }
    }

    fn corner(point: Vec2) -> Self {
        VertexGeometryPoint {
            point,
            kind: VertexGeometryPointKind::Corner,
        }
    }

    fn is_wall(&self) -> bool {
        matches!(self.kind, VertexGeometryPointKind::Wall(_))
    }

    fn to_vec3(self) -> Vec3 {
        self.point.extend(0.)
    }
}

fn vertex_intersections(a1: f32, a2: f32) -> impl Iterator<Item = Vec2> {
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
