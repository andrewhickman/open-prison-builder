use std::{
    f32::consts::{FRAC_PI_2, FRAC_PI_4, SQRT_2, TAU},
    iter,
};

use bevy::{
    asset::weak_handle,
    ecs::entity::EntityHashMap,
    math::FloatOrd,
    prelude::*,
    render::{
        mesh::{
            Indices, MeshAabb, MeshVertexBufferLayoutRef, PrimitiveTopology, VertexAttributeValues,
        },
        primitives::Aabb,
        render_asset::RenderAssetUsages,
        render_resource::{AsBindGroup, RenderPipelineDescriptor, ShaderRef},
    },
    sprite::{Material2d, Material2dKey},
};
use pb_assets::AssetHandles;
use pb_engine::{
    EngineState,
    map::{
        Map, MapEntity,
        corner::Corner,
        door::{self, Door},
        wall::Wall,
    },
    root::ChildOfRoot,
};
use smallvec::SmallVec;

use crate::layer;

const CORNER_LOCUS: Vec2 = Vec2::new(0., 0.5 * Wall::RADIUS);

const TEXTURE_TOP: f32 = 0.0;
const TEXTURE_BOTTOM: f32 = 1.0;

#[derive(Resource, Default, Debug)]
pub enum VisibleMaps {
    #[default]
    Hidden,
    Visible {
        map: Entity,
    },
    Preview {
        map: Entity,
        source: Entity,
    },
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
    door: bool,
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct WallMaterial {
    #[uniform(0)]
    color: LinearRgba,
    #[texture(1)]
    #[sampler(2)]
    pub texture: Handle<Image>,
}

#[derive(Copy, Clone, Debug)]
pub enum MapRenderMode {
    Added,
    Visible,
    Removed,
    Hidden,
}

pub const DEFAULT_MATERIAL: Handle<WallMaterial> =
    weak_handle!("9644d394-94cd-4fd8-972a-c76026f4d08a");
pub const ADDED_MATERIAL: Handle<WallMaterial> =
    weak_handle!("8562bf14-56d0-4fa8-ac4a-325b5e2eddef");
pub const REMOVED_MATERIAL: Handle<WallMaterial> =
    weak_handle!("202d16ae-02f2-4584-a77c-7882a55db5fa");

pub const DEFAULT_MATERIAL_DOOR_FRAME: Handle<WallMaterial> =
    weak_handle!("f79933c3-4af3-4333-9baf-8c6bbd758463");
pub const ADDED_MATERIAL_DOOR_FRAME: Handle<WallMaterial> =
    weak_handle!("9cf8f4b9-90ad-4a71-9e86-a8c5b8b4567c");
pub const REMOVED_MATERIAL_DOOR_FRAME: Handle<WallMaterial> =
    weak_handle!("000f297d-b754-4828-a46f-64c7fea483de");

const WALL_SHADER_HANDLE: Handle<Shader> = weak_handle!("ac4fc4ae-cc6c-408f-87f2-a75b44bc01b7");

pub fn startup(
    mut materials: ResMut<Assets<WallMaterial>>,
    mut shaders: ResMut<Assets<Shader>>,
    assets: Res<AssetHandles>,
) -> Result {
    materials.insert(
        &DEFAULT_MATERIAL,
        WallMaterial {
            color: LinearRgba::WHITE,
            texture: assets.brick_image.clone(),
        },
    );
    materials.insert(
        &ADDED_MATERIAL,
        WallMaterial {
            color: Srgba::hex("aaaaaa")?.into(),
            texture: assets.brick_image.clone(),
        },
    );
    materials.insert(
        &REMOVED_MATERIAL,
        WallMaterial {
            color: Srgba::hex("ffaaaa")?.into(),
            texture: assets.brick_image.clone(),
        },
    );
    materials.insert(
        &DEFAULT_MATERIAL_DOOR_FRAME,
        WallMaterial {
            color: LinearRgba::WHITE,
            texture: assets.brick_door_frame_image.clone(),
        },
    );
    materials.insert(
        &ADDED_MATERIAL_DOOR_FRAME,
        WallMaterial {
            color: Srgba::hex("aaaaaa")?.into(),
            texture: assets.brick_door_frame_image.clone(),
        },
    );
    materials.insert(
        &REMOVED_MATERIAL_DOOR_FRAME,
        WallMaterial {
            color: Srgba::hex("ffaaaa")?.into(),
            texture: assets.brick_door_frame_image.clone(),
        },
    );
    shaders.insert(
        WALL_SHADER_HANDLE.id(),
        Shader::from_wgsl(
            include_str!("../../assets/shaders/wall.wgsl"),
            "assets/shaders/wall.wgsl",
        ),
    );
    Ok(())
}

pub fn corner_inserted(
    trigger: Trigger<OnInsert, Corner>,
    mut commands: Commands,
    corner_q: Query<&Corner>,
    mut meshes: ResMut<Assets<Mesh>>,
) -> Result {
    let corner = corner_q.get(trigger.target())?;
    let render_mode = MapRenderMode::Hidden;

    let corner_info = CornerGeometry::new(corner, iter::empty());

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
        render_mode.material(false),
        render_mode.visibility(),
    ));
    Ok(())
}

pub fn wall_inserted(trigger: Trigger<OnInsert, Wall>, mut commands: Commands) {
    let render_mode = MapRenderMode::Hidden;
    commands.entity(trigger.target()).insert((
        WallGeometry::default(),
        Mesh2d::default(),
        Aabb::default(),
        render_mode.material(false),
        render_mode.visibility(),
    ));
}

pub fn map_removed(trigger: Trigger<OnRemove, Map>, mut visible_maps: ResMut<VisibleMaps>) {
    visible_maps.remove(trigger.target());
}

pub fn update_visible_maps(
    mut visible_maps: ResMut<VisibleMaps>,
    engine_state: Res<State<EngineState>>,
    root_map_q: Query<Entity, (With<Map>, With<ChildOfRoot>)>,
) -> Result {
    if engine_state.is_changed() {
        match *engine_state.get() {
            EngineState::Running(_) => {
                *visible_maps = VisibleMaps::Visible {
                    map: root_map_q.single()?,
                };
            }
            EngineState::Disabled => *visible_maps = VisibleMaps::Hidden,
        }
    }

    Ok(())
}

pub fn update_render_mode_condition(
    visible_maps: Res<VisibleMaps>,
    map_q: Query<Ref<Map>>,
) -> bool {
    visible_maps.is_changed()
        || visible_maps
            .id()
            .is_some_and(|map| map_q.get(map).is_ok_and(|map| map.is_changed()))
}

pub fn update_render_mode(
    visible_maps: Res<VisibleMaps>,
    map_q: Query<Ref<Map>>,
    children_q: Query<&Children>,
    mut render_mode_q: Query<(&mut Visibility, &mut MeshMaterial2d<WallMaterial>)>,
    door_q: Query<Entity, With<Door>>,
) -> Result {
    let mut render_modes = EntityHashMap::default();
    for map in &map_q {
        if visible_maps.id() != Some(map.id()) {
            for entity in children_q.iter_descendants(map.id()) {
                render_modes.insert(entity, MapRenderMode::Hidden);
            }
        }
    }

    if let Some(map) = visible_maps.id() {
        let map = map_q.get(map)?;
        for corners in map.corners() {
            match corners {
                MapEntity::Cloned(entity) => {
                    render_modes.insert(entity, MapRenderMode::Visible);
                }
                MapEntity::Replaced(source, entity) => {
                    render_modes.insert(source, MapRenderMode::Hidden);
                    render_modes.insert(entity, MapRenderMode::Visible);
                }
                MapEntity::Owned(entity) => {
                    if visible_maps.is_preview() {
                        render_modes.insert(entity, MapRenderMode::Added);
                    } else {
                        render_modes.insert(entity, MapRenderMode::Visible);
                    }
                }
            }
        }

        for wall in map.walls() {
            match wall {
                MapEntity::Cloned(entity) => {
                    render_modes.insert(entity, MapRenderMode::Visible);
                }
                MapEntity::Replaced(source, entity) => {
                    render_modes.insert(source, MapRenderMode::Hidden);
                    render_modes.insert(entity, MapRenderMode::Visible);
                }
                MapEntity::Owned(entity) => {
                    if visible_maps.is_preview() {
                        render_modes.insert(entity, MapRenderMode::Added);
                    } else {
                        render_modes.insert(entity, MapRenderMode::Visible);
                    }
                }
            }
        }
    }

    for (id, render_mode) in render_modes {
        let Ok((mut visibility, mut material)) = render_mode_q.get_mut(id) else {
            continue;
        };
        visibility.set_if_neq(render_mode.visibility());

        let new_material = render_mode.material(door_q.contains(id));
        if material.0 != new_material.0 {
            *material = new_material;
        }
    }

    Ok(())
}

pub fn update_geometry(
    mut meshes: ResMut<Assets<Mesh>>,
    visible_maps: Res<VisibleMaps>,
    map_q: Query<Ref<Map>>,
    corner_position_q: Query<&Corner>,
    mut corner_q: Query<(&Corner, &mut CornerGeometry, &mut Mesh2d, &mut Aabb), Without<Wall>>,
    mut wall_q: Query<
        (
            &Wall,
            Option<&Door>,
            &mut WallGeometry,
            &mut Mesh2d,
            &mut Aabb,
        ),
        Without<Corner>,
    >,
) -> Result {
    for map in &map_q {
        if !map.is_changed() && !visible_maps.is_changed() {
            continue;
        }

        for entity in map.corners() {
            let Ok((corner, mut info, mut mesh, mut aabb)) = corner_q.get_mut(entity.id()) else {
                continue;
            };

            let new_info = CornerGeometry::new(
                corner,
                map.corner_walls(corner).filter_map(|(wall, end_corner)| {
                    corner_position_q
                        .get(end_corner)
                        .ok()
                        .map(|end_corner| (wall, end_corner))
                }),
            );

            if info.set_if_neq(new_info) {
                update_mesh(&mut meshes, &mut mesh, &mut aabb, info.mesh());
            }
        }

        for entity in map.walls() {
            let (wall, door, mut info, mut mesh, mut aabb) = wall_q.get_mut(entity.id())?;
            let [(_, start_info, _, _), (_, end_info, _, _)] = corner_q.get_many(wall.corners())?;

            let new_info = WallGeometry::new(entity.id(), wall, door, start_info, end_info)?;
            if info.set_if_neq(new_info) {
                update_mesh(&mut meshes, &mut mesh, &mut aabb, info.mesh());
            }
        }
    }

    Ok(())
}

impl VisibleMaps {
    pub fn id(&self) -> Option<Entity> {
        match *self {
            VisibleMaps::Hidden => None,
            VisibleMaps::Visible { map } | VisibleMaps::Preview { map, .. } => Some(map),
        }
    }

    pub fn source(&self) -> Option<Entity> {
        match *self {
            VisibleMaps::Hidden => None,
            VisibleMaps::Visible { map: source } | VisibleMaps::Preview { source, .. } => {
                Some(source)
            }
        }
    }

    pub fn is_preview(&self) -> bool {
        matches!(self, VisibleMaps::Preview { .. })
    }

    pub fn remove(&mut self, id: Entity) {
        match *self {
            VisibleMaps::Visible { map } | VisibleMaps::Preview { source: map, .. }
                if map == id =>
            {
                *self = VisibleMaps::Hidden
            }
            VisibleMaps::Preview { map, source } if map == id => {
                *self = VisibleMaps::Visible { map: source }
            }
            _ => {}
        }
    }
}

impl MapRenderMode {
    pub fn material(self, door: bool) -> MeshMaterial2d<WallMaterial> {
        match (self, door) {
            (MapRenderMode::Added, false) => MeshMaterial2d(ADDED_MATERIAL.clone()),
            (MapRenderMode::Added, true) => MeshMaterial2d(ADDED_MATERIAL_DOOR_FRAME.clone()),
            (MapRenderMode::Visible, false) => MeshMaterial2d(DEFAULT_MATERIAL.clone()),
            (MapRenderMode::Visible, true) => MeshMaterial2d(DEFAULT_MATERIAL_DOOR_FRAME.clone()),
            (MapRenderMode::Removed, false) => MeshMaterial2d(REMOVED_MATERIAL.clone()),
            (MapRenderMode::Removed, true) => MeshMaterial2d(REMOVED_MATERIAL_DOOR_FRAME.clone()),
            (MapRenderMode::Hidden, _) => MeshMaterial2d(DEFAULT_MATERIAL.clone()),
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

    fn wall_intersection(&self, id: Entity) -> Option<(Vec2, Vec2, Vec2)> {
        let index = self
            .points
            .iter()
            .position(|p| p.kind == CornerGeometryPointKind::Wall(id))?;

        Some((
            self.pos + wrapping_idx(&self.points, index, -1).point,
            self.pos + wrapping_idx(&self.points, index, 0).point,
            self.pos + wrapping_idx(&self.points, index, 1).point,
        ))
    }

    fn mesh(&self) -> Option<Mesh> {
        let mut vertices = Vec::new();
        let mut uvs = Vec::new();
        for (i, i1) in self.points.iter().enumerate() {
            let i2 = wrapping_idx(&self.points, i, 1);

            if i1.is_wall() || i2.is_wall() {
                continue;
            }

            vertices.extend([i1.to_vec3(), CORNER_LOCUS.extend(layer::WALL), i2.to_vec3()]);

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

impl WallGeometry {
    fn new(
        id: Entity,
        wall: &Wall,
        door: Option<&Door>,
        start: &CornerGeometry,
        end: &CornerGeometry,
    ) -> Result<Self> {
        let (start1, start2, start3) = start
            .wall_intersection(id)
            .ok_or("wall intersection not found")?;
        let (end1, end2, end3) = end
            .wall_intersection(id)
            .ok_or("wall intersection not found")?;

        let wall_inv_isometry = wall.isometry().inverse();
        let points = [
            wall_inv_isometry * start1,
            wall_inv_isometry * start2,
            wall_inv_isometry * start3,
            wall_inv_isometry * end1,
            wall_inv_isometry * end2,
            wall_inv_isometry * end3,
        ];

        Ok(WallGeometry {
            points,
            door: door.is_some(),
        })
    }

    fn mesh(&self) -> Option<Mesh> {
        if self.door {
            let door_points = [
                Vec2::new(-door::HALF_INNER_WIDTH, self.points[0].y),
                Vec2::new(-door::HALF_INNER_WIDTH, self.points[1].y),
                Vec2::new(-door::HALF_INNER_WIDTH, self.points[2].y),
                Vec2::new(door::HALF_INNER_WIDTH, self.points[3].y),
                Vec2::new(door::HALF_INNER_WIDTH, self.points[4].y),
                Vec2::new(door::HALF_INNER_WIDTH, self.points[5].y),
            ];

            // Vertex layout:
            // 2 ┌──────────────────────┐ 8    9 ┌──────────────────────┐ 3
            // 1 ├──────────────────────┤ 7   10 ├──────────────────────┤ 4
            // 0 └──────────────────────┘ 6   11 └──────────────────────┘ 5
            Some(
                Mesh::new(
                    PrimitiveTopology::TriangleList,
                    RenderAssetUsages::RENDER_WORLD,
                )
                .with_inserted_attribute(
                    Mesh::ATTRIBUTE_POSITION,
                    VertexAttributeValues::Float32x3(
                        self.points
                            .iter()
                            .chain(&door_points)
                            .map(|p| [p.x, p.y, layer::WALL])
                            .collect(),
                    ),
                )
                .with_inserted_attribute(
                    Mesh::ATTRIBUTE_UV_0,
                    VertexAttributeValues::Float32x2(vec![
                        [0.5 + self.points[0].x - door_points[0].x, TEXTURE_BOTTOM],
                        [0.5 + self.points[1].x - door_points[1].x, TEXTURE_TOP],
                        [0.5 + self.points[2].x - door_points[2].x, TEXTURE_BOTTOM],
                        [0.5 - self.points[3].x + door_points[3].x, TEXTURE_BOTTOM],
                        [0.5 - self.points[4].x + door_points[4].x, TEXTURE_TOP],
                        [0.5 - self.points[5].x + door_points[5].x, TEXTURE_BOTTOM],
                        [0.5, TEXTURE_BOTTOM],
                        [0.5, TEXTURE_TOP],
                        [0.5, TEXTURE_BOTTOM],
                        [0.5, TEXTURE_BOTTOM],
                        [0.5, TEXTURE_TOP],
                        [0.5, TEXTURE_BOTTOM],
                    ]),
                )
                .with_inserted_indices(Indices::U16(vec![
                    0, 6, 1, 6, 1, 7, 1, 7, 2, 7, 2, 8, 11, 5, 10, 5, 10, 4, 10, 4, 9, 4, 9, 3,
                ])),
            )
        } else {
            // Vertex layout:
            // 2 ┌──────────────────────┐ 3
            // 1 ├──────────────────────┤ 4
            // 0 └──────────────────────┘ 5
            Some(
                Mesh::new(
                    PrimitiveTopology::TriangleList,
                    RenderAssetUsages::RENDER_WORLD,
                )
                .with_inserted_attribute(
                    Mesh::ATTRIBUTE_POSITION,
                    VertexAttributeValues::Float32x3(
                        self.points.map(|p| [p.x, p.y, layer::WALL]).to_vec(),
                    ),
                )
                .with_inserted_attribute(
                    Mesh::ATTRIBUTE_UV_0,
                    VertexAttributeValues::Float32x2(vec![
                        [self.points[0].x, TEXTURE_BOTTOM],
                        [self.points[1].x, TEXTURE_TOP],
                        [self.points[2].x, TEXTURE_BOTTOM],
                        [self.points[3].x, TEXTURE_BOTTOM],
                        [self.points[4].x, TEXTURE_TOP],
                        [self.points[5].x, TEXTURE_BOTTOM],
                    ]),
                )
                .with_inserted_indices(Indices::U16(vec![0, 1, 5, 1, 5, 4, 1, 2, 4, 2, 4, 3])),
            )
        }
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
        self.point.extend(layer::WALL)
    }
}

impl Material2d for WallMaterial {
    fn fragment_shader() -> ShaderRef {
        ShaderRef::Handle(WALL_SHADER_HANDLE)
    }

    fn specialize(
        descriptor: &mut RenderPipelineDescriptor,
        _: &MeshVertexBufferLayoutRef,
        _: Material2dKey<Self>,
    ) -> Result<(), bevy::render::render_resource::SpecializedMeshPipelineError> {
        descriptor
            .depth_stencil
            .as_mut()
            .expect("no depth stencil for Mesh2d pipeline")
            .depth_write_enabled = true;
        Ok(())
    }
}

fn update_mesh(
    meshes: &mut Assets<Mesh>,
    mesh: &mut Mesh2d,
    aabb: &mut Aabb,
    new_mesh: Option<Mesh>,
) {
    if let Some(new_mesh) = new_mesh {
        *aabb = new_mesh.compute_aabb().unwrap_or_default();
        *mesh = Mesh2d(meshes.add(new_mesh));
    } else {
        *aabb = default();
        *mesh = default();
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
    Vec2::from_angle(a) * Wall::RADIUS * SQRT_2
}

fn angle_intersection(mid: f32, da: f32) -> Vec2 {
    Vec2::from_angle(mid) * Wall::RADIUS / f32::sin(da)
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
