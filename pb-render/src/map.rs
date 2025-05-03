use std::{
    f32::consts::{FRAC_PI_2, FRAC_PI_4, SQRT_2, TAU},
    iter,
};

use bevy::{
    ecs::entity::{EntityHashMap, EntityHashSet},
    math::FloatOrd,
    prelude::*,
    render::{
        mesh::{Indices, MeshAabb, PrimitiveTopology},
        primitives::Aabb,
        render_asset::RenderAssetUsages,
    },
    sprite::AlphaMode2d,
};
use pb_assets::AssetHandles;
use pb_engine::{wall, EngineState};
use pb_map::{Corner, Map, MapEntity, Wall};
use pb_util::{try_res_s, weak_handle};
use smallvec::SmallVec;

const CORNER_LOCUS: Vec2 = Vec2::new(0.25 * wall::RADIUS, 0.5 * wall::RADIUS);

const TEXTURE_TOP: f32 = 0.0;
const TEXTURE_BOTTOM: f32 = 1.0;

#[derive(Resource, Default)]
pub struct VisibleMap {
    id: Option<Entity>,
    source: Option<Entity>,
}

#[derive(Debug, Component, PartialEq)]
pub struct CornerGeometry {
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
    Wall(Entity),
    Corner,
}

#[derive(Debug, Default, Component, PartialEq)]
pub struct WallGeometry {
    points: [Vec2; 6],
    lens: [f32; 6],
}

#[derive(Copy, Clone, Debug)]
enum MapRenderMode {
    Added,
    Visible,
    Removed,
    Hidden,
}

pub const DEFAULT_MATERIAL: Handle<ColorMaterial> =
    weak_handle!("9644d394-94cd-4fd8-972a-c76026f4d08a");
pub const ADDED_MATERIAL: Handle<ColorMaterial> =
    weak_handle!("8562bf14-56d0-4fa8-ac4a-325b5e2eddef");
pub const REMOVED_MATERIAL: Handle<ColorMaterial> =
    weak_handle!("202d16ae-02f2-4584-a77c-7882a55db5fa");

pub fn startup(mut materials: ResMut<Assets<ColorMaterial>>, assets: Res<AssetHandles>) {
    materials.insert(
        &DEFAULT_MATERIAL,
        ColorMaterial::from(assets.brick_image.clone()),
    );
    materials.insert(
        &ADDED_MATERIAL,
        ColorMaterial {
            color: Color::WHITE.with_alpha(0.38),
            alpha_mode: AlphaMode2d::Blend,
            texture: Some(assets.brick_image.clone()),
        },
    );
    materials.insert(
        &REMOVED_MATERIAL,
        ColorMaterial {
            color: Srgba::hex("f2200d").unwrap().with_alpha(0.38).into(),
            alpha_mode: AlphaMode2d::Blend,
            texture: Some(assets.brick_image.clone()),
        },
    );
}

pub fn corner_inserted(
    trigger: Trigger<OnInsert, Corner>,
    mut commands: Commands,
    corner_q: Query<(&Transform, &Parent)>,
    mut meshes: ResMut<Assets<Mesh>>,
    visible_map: Res<VisibleMap>,
) {
    info!("query corner {:?}", trigger.target());
    let (transform, parent) = corner_q.get(trigger.target()).unwrap();
    let render_mode = MapRenderMode::inserted(&visible_map, parent.get());

    let corner_info = CornerGeometry::new(transform, iter::empty());

    let mesh = corner_info.mesh();
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
        corner_info,
        Mesh2d(mesh),
        aabb,
        render_mode.material(),
        render_mode.visibility(),
    ));
}

pub fn wall_inserted(
    trigger: Trigger<OnInsert, Wall>,
    mut commands: Commands,
    wall_q: Query<&Parent>,
    meshes: Res<Assets<Mesh>>,
    visible_map: Res<VisibleMap>,
) {
    let parent = wall_q.get(trigger.target()).unwrap();
    let render_mode = MapRenderMode::inserted(&visible_map, parent.get());

    let mesh = meshes.reserve_handle();

    commands.entity(trigger.target()).insert((
        WallGeometry::default(),
        Mesh2d(mesh),
        Aabb::default(),
        render_mode.material(),
        render_mode.visibility(),
    ));
}

pub fn update_visibility(
    engine_state: Res<State<EngineState>>,
    mut visible_maps: ResMut<VisibleMap>,
    map_q: Query<&Map>,
    children_q: Query<&Children>,
    mut render_mode_q: Query<(&mut Visibility, &mut MeshMaterial2d<ColorMaterial>)>,
) {
    if engine_state.is_changed() {
        match *engine_state.get() {
            EngineState::Running(root) => {
                for &child in children_q.children(root) {
                    if let Ok(map) = map_q.get(child) {
                        visible_maps.set(map.id(), map.source());
                    }
                }
            }
            EngineState::Disabled => {
                visible_maps.clear();
            }
        }
    }

    if visible_maps.is_changed() {
        let mut render_modes = EntityHashMap::default();
        for map in &map_q {
            if visible_maps.id() != Some(map.id()) && visible_maps.source() != Some(map.id()) {
                for entity in children_q.iter_descendants(map.id()) {
                    render_modes.insert(entity, MapRenderMode::Hidden);
                }
            }
        }

        if let Some(source) = visible_maps.source() {
            let source = map_q.get(source).unwrap();
            for entity in children_q.iter_descendants(source.id()) {
                render_modes.insert(entity, MapRenderMode::Removed);
            }
        }

        if let Some(map) = visible_maps.id() {
            let map = map_q.get(map).unwrap();
            for entity in map.entities() {
                match entity {
                    MapEntity::Cloned(entity) => {
                        render_modes.insert(entity, MapRenderMode::Visible);
                    }
                    MapEntity::Replaced(source, entity) => {
                        render_modes.insert(source, MapRenderMode::Hidden);
                        render_modes.insert(entity, MapRenderMode::Added);
                    }
                    MapEntity::Owned(entity) => {
                        render_modes.insert(entity, MapRenderMode::Added);
                    }
                }
            }
        }

        for (id, render_mode) in render_modes {
            let (mut visibility, mut material) = render_mode_q.get_mut(id).unwrap();
            visibility.set_if_neq(render_mode.visibility());

            let new_material = render_mode.material();
            if material.0 != new_material.0 {
                *material = new_material;
            }
        }
    }
}

pub fn map_removed(
    trigger: Trigger<OnRemove, Map>,
    mut visible_map: ResMut<VisibleMap>,
    map_q: Query<&Map>,
) {
    if visible_map.id == Some(trigger.target()) {
        if let Some(source) = visible_map.source {
            if let Ok(map) = map_q.get(source) {
                visible_map.set(map.id(), map.source());
            } else {
                visible_map.clear()
            }
        } else {
            visible_map.clear();
        }
    }
}

pub fn update_geometry(
    mut meshes: ResMut<Assets<Mesh>>,
    map_q: Query<&Map, Changed<Map>>,
    transform_q: Query<&Transform, With<Corner>>,
    mut corner_q: Query<(
        &Corner,
        &Transform,
        &mut CornerGeometry,
        &mut Mesh2d,
        &mut Aabb,
    )>,
    mut wall_mesh_q: Query<(&Wall, &mut WallGeometry, &mut Mesh2d, &mut Aabb), Without<Corner>>,
) {
    for map in &map_q {
        let mut updated_walls = EntityHashSet::default();

        for id in map.corners() {
            let Ok((corner, transform, mut info, mut mesh, mut aabb)) = corner_q.get_mut(id.id())
            else {
                continue;
            };

            let new_info = CornerGeometry::new(
                transform,
                map.corner_walls(corner).filter_map(|(wall, end_corner)| {
                    transform_q.get(end_corner).ok().map(|pos| (wall, pos))
                }),
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

                updated_walls.extend(map.corner_walls(corner).map(|(wall, _)| wall));
            }
        }

        for id in updated_walls {
            let (wall, mut info, mut mesh, mut aabb) = try_res_s!(wall_mesh_q.get_mut(id));
            let [(_, _, start_info, _, _), (_, _, end_info, _, _)] =
                corner_q.many(map.wall_corners(wall));

            let new_info = WallGeometry::new(id, start_info, end_info);
            if info.set_if_neq(new_info) {
                let new_mesh = info.mesh();
                *aabb = new_mesh.compute_aabb().unwrap_or_default();
                mesh.0 = meshes.add(new_mesh);
            }
        }
    }
}

impl CornerGeometry {
    fn new<'a>(
        transform: &Transform,
        walls: impl Iterator<Item = (Entity, &'a Transform)>,
    ) -> Self {
        let start = transform.translation.xy();

        let mut angles: SmallVec<[(Option<Entity>, f32); 4]> = walls
            .map(|(id, end)| (Some(id), (end.translation.xy() - start).to_angle()))
            .collect();
        angles.sort_by_key(|&(_, angle)| FloatOrd(angle));

        let mut points: SmallVec<[CornerGeometryPoint; 4]> = default();
        for (index, &(wall, a2)) in angles.iter().enumerate() {
            if index == 0 {
                let a1 = wrapping_idx(&angles, index, -1).1;
                points.extend(corner_intersections(a1, a2).map(CornerGeometryPoint::corner));
            }

            if let Some(wall) = wall {
                points.push(CornerGeometryPoint::wall(wall));
            }

            if index != (angles.len() - 1) {
                let a3 = wrapping_idx(&angles, index, 1).1;
                points.extend(corner_intersections(a2, a3).map(CornerGeometryPoint::corner));
            }
        }

        if points.is_empty() {
            points.extend_from_slice(&[
                CornerGeometryPoint::corner(right_angle_intersection(FRAC_PI_4)),
                CornerGeometryPoint::corner(right_angle_intersection(3. * FRAC_PI_4)),
                CornerGeometryPoint::corner(right_angle_intersection(5. * FRAC_PI_4)),
                CornerGeometryPoint::corner(right_angle_intersection(7. * FRAC_PI_4)),
            ]);
        }

        CornerGeometry { pos: start, points }
    }

    fn wall_intersection(&self, id: Entity, pos: Vec2) -> Option<(Vec2, Vec2, Vec2)> {
        let offset = self.pos - pos;
        let index = self
            .points
            .iter()
            .position(|p| p.kind == CornerGeometryPointKind::Wall(id))?;
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

            vertices.extend([i1.to_vec3(), CORNER_LOCUS.extend(0.), i2.to_vec3()]);

            let di = i2.point - i1.point;
            let base_len = di.length();
            let locus_len = (CORNER_LOCUS - i1.point).project_onto(di).length();

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

impl VisibleMap {
    pub fn id(&self) -> Option<Entity> {
        self.id
    }

    pub fn source(&self) -> Option<Entity> {
        self.source
    }

    pub fn set(&mut self, id: Entity, source: Option<Entity>) {
        self.id = Some(id);
        self.source = source;
    }

    pub fn clear(&mut self) {
        self.id = None;
        self.source = None;
    }
}

impl MapRenderMode {
    pub fn inserted(visible_map: &VisibleMap, map: Entity) -> Self {
        if visible_map.id() == Some(map) {
            if visible_map.source().is_some() {
                MapRenderMode::Added
            } else {
                MapRenderMode::Visible
            }
        } else {
            MapRenderMode::Hidden
        }
    }

    pub fn material(self) -> MeshMaterial2d<ColorMaterial> {
        match self {
            MapRenderMode::Added => MeshMaterial2d(ADDED_MATERIAL.clone()),
            MapRenderMode::Visible => MeshMaterial2d(DEFAULT_MATERIAL.clone()),
            MapRenderMode::Removed => MeshMaterial2d(REMOVED_MATERIAL.clone()),
            MapRenderMode::Hidden => MeshMaterial2d(DEFAULT_MATERIAL.clone()),
        }
    }

    pub fn visibility(self) -> Visibility {
        match self {
            MapRenderMode::Added | MapRenderMode::Visible | MapRenderMode::Removed => {
                Visibility::Visible
            }
            MapRenderMode::Hidden => Visibility::Hidden,
        }
    }
}

impl WallGeometry {
    fn new(id: Entity, start: &CornerGeometry, end: &CornerGeometry) -> Self {
        let pos = start.pos.midpoint(end.pos);
        let (start1, start2, start3) = start.wall_intersection(id, pos).unwrap();
        let (end1, end2, end3) = end.wall_intersection(id, pos).unwrap();

        WallGeometry {
            points: [start1, start2, start3, end1, end2, end3],
            lens: [
                start1.project_onto(start2).length(),
                start2.length(),
                start3.project_onto(start2).length(),
                end1.project_onto(end2).length(),
                end2.length(),
                end3.project_onto(end2).length(),
            ],
        }
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

impl CornerGeometryPoint {
    fn wall(wall: Entity) -> Self {
        CornerGeometryPoint {
            point: CORNER_LOCUS,
            kind: CornerGeometryPointKind::Wall(wall),
        }
    }

    fn corner(point: Vec2) -> Self {
        CornerGeometryPoint {
            point,
            kind: CornerGeometryPointKind::Corner,
        }
    }

    fn is_wall(&self) -> bool {
        matches!(self.kind, CornerGeometryPointKind::Wall(_))
    }

    fn to_vec3(self) -> Vec3 {
        self.point.extend(0.)
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
