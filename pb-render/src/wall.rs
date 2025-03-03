use std::f32::consts::{FRAC_PI_2, FRAC_PI_4, PI, SQRT_2, TAU};

use approx::relative_ne;
use bevy::{
    math::FloatOrd,
    prelude::*,
    render::{
        mesh::{MeshAabb, PrimitiveTopology},
        primitives::Aabb,
        render_asset::RenderAssetUsages,
    },
    utils::hashbrown::HashSet,
};
use pb_engine::{
    build::Blueprint,
    wall::{self, Vertex, Wall, WallMap},
};
use pb_util::{try_modify_component, try_res_s, weak_handle};
use smallvec::SmallVec;

#[derive(Debug, Default, Component, PartialEq)]
pub struct VertexGeometry {
    pos: Vec2,
    walls: SmallVec<[(Entity, f32); 4]>,
}

#[derive(Debug, Default, Component, PartialEq)]
pub struct WallGeometry {
    intersections: [Vec2; 4],
}

pub const WHITE: Handle<ColorMaterial> = weak_handle!("9644d394-94cd-4fd8-972a-c76026f4d08a");
pub const TRANSLUCENT_WHITE: Handle<ColorMaterial> =
    weak_handle!("8562bf14-56d0-4fa8-ac4a-325b5e2eddef");

pub fn startup(mut assets: ResMut<Assets<ColorMaterial>>) {
    assets.insert(&WHITE, ColorMaterial::from_color(Color::WHITE));
    assets.insert(
        &TRANSLUCENT_WHITE,
        ColorMaterial::from_color(Color::WHITE.with_alpha(0.38)),
    );
}

pub fn vertex_inserted(
    trigger: Trigger<OnInsert, Vertex>,
    mut commands: Commands,
    transform_q: Query<&Transform>,
    preview_q: Query<&Blueprint>,
    mut assets: ResMut<Assets<Mesh>>,
) {
    let color = if preview_q.contains(trigger.entity()) {
        TRANSLUCENT_WHITE.clone()
    } else {
        WHITE.clone()
    };

    let vertex_info = VertexGeometry {
        pos: try_res_s!(transform_q.get(trigger.entity()))
            .translation
            .xy(),
        walls: default(),
    };

    let mesh = vertex_info.mesh();
    let aabb = mesh.compute_aabb().unwrap_or_default();
    let mesh = assets.add(vertex_info.mesh());

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
    assets: Res<Assets<Mesh>>,
) {
    let color = if preview_q.contains(trigger.entity()) {
        TRANSLUCENT_WHITE.clone()
    } else {
        WHITE.clone()
    };

    let mesh = assets.reserve_handle();

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

pub fn preview_removed(
    trigger: Trigger<OnRemove, Blueprint>,
    mut commands: Commands,
    mut wall_map: ResMut<WallMap>,
) {
    commands.queue(try_modify_component(
        trigger.entity(),
        |mut color: Mut<MeshMaterial2d<ColorMaterial>>| color.0 = WHITE,
    ));
    wall_map.set_changed();
}

pub fn update_wall(
    mut assets: ResMut<Assets<Mesh>>,
    wall_map: Res<WallMap>,
    transform_q: Query<&Transform, With<Vertex>>,
    mut vertex_q: Query<
        (Entity, &Transform, &mut VertexGeometry, &Mesh2d, &mut Aabb),
        With<Vertex>,
    >,
    mut wall_mesh_q: Query<(&Wall, &mut WallGeometry, &Mesh2d, &mut Aabb), Without<Vertex>>,
) {
    let mut updated_walls = HashSet::new();

    for (id, transform, mut info, mesh, mut aabb) in vertex_q.iter_mut() {
        let new_info = VertexGeometry::new(
            transform,
            wall_map
                .get(id)
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
        let (wall, mut info, mesh, mut aabb) = try_res_s!(wall_mesh_q.get_mut(id));
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
        let mut walls: SmallVec<[(Entity, f32); 4]> = walls
            .map(|(id, end)| (id, (end.translation.xy() - start).to_angle()))
            .collect();
        walls.sort_by_key(|&(_, angle)| FloatOrd(angle));
        VertexGeometry { pos: start, walls }
    }

    fn wall_intersection(&self, id: Entity) -> Option<(Vec2, Vec2)> {
        let index = self.walls.iter().position(|&(w, _)| w == id)?;
        let a1 = wrapping_idx(&self.walls, index, -1).1;
        let a2 = wrapping_idx(&self.walls, index, 0).1;
        let a3 = wrapping_idx(&self.walls, index, 1).1;

        let (i1, i2) = wall_intersection(a1, a2, a3);
        Some((self.pos + i1, self.pos + i2))
    }

    fn mesh(&self) -> Mesh {
        let mut intersections = SmallVec::<[Vec2; 4]>::new();

        if self.walls.is_empty() {
            intersections.extend(vertex_intersections(0., 0.));
        } else {
            for (i, &(_, a1)) in self.walls.iter().enumerate() {
                let a2 = wrapping_idx(&self.walls, i, 1).1;
                intersections.extend(vertex_intersections(a1, a2));
            }
        }

        let mut vertices = Vec::new();
        for (i, v1) in intersections.iter().enumerate() {
            let v2 = wrapping_idx(&intersections, i, 1);
            vertices.extend([v1.extend(0.), Vec3::ZERO, v2.extend(0.)]);
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
        let (start1, start2) = start.wall_intersection(id).unwrap();
        let (end1, end2) = end.wall_intersection(id).unwrap();

        WallGeometry {
            intersections: [start1 - pos, start2 - pos, end1 - pos, end2 - pos],
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
                self.intersections[0].extend(0.),
                self.intersections[1].extend(0.),
                self.intersections[3].extend(0.),
                self.intersections[2].extend(0.),
            ],
        )
    }
}

fn vertex_intersections(mut a1: f32, mut a2: f32) -> SmallVec<[Vec2; 5]> {
    let mut da = angle_delta(a1, a2);
    let reflex = da >= PI;

    let mut result = SmallVec::new();

    while da >= PI {
        result.insert_from_slice(
            result.len() / 2,
            &[
                SQRT_2 * Vec2::from_angle(a1 + FRAC_PI_4) * wall::RADIUS,
                SQRT_2 * Vec2::from_angle(a2 - FRAC_PI_4) * wall::RADIUS,
            ],
        );

        a1 += FRAC_PI_2;
        a2 -= FRAC_PI_2;
        da -= PI;
    }

    if relative_ne!(da, 0.0) {
        let mid = a1 + da / 2.;
        if !reflex {
            da = PI - da;
        }

        let len = wall::RADIUS / f32::cos(da / 2.);
        result.insert(result.len() / 2, len * Vec2::from_angle(mid));
    }

    result
}

fn wall_intersection(a1: f32, a2: f32, a3: f32) -> (Vec2, Vec2) {
    let da1 = angle_delta(a1, a2);
    let da3 = angle_delta(a2, a3);

    let i1 = if da1 >= PI {
        SQRT_2 * Vec2::from_angle(a2 - FRAC_PI_4) * wall::RADIUS
    } else {
        let mid = a2 - da1 / 2.;
        let len = wall::RADIUS / f32::sin(da1 / 2.);

        len * Vec2::from_angle(mid)
    };
    let i2 = if da3 >= PI {
        SQRT_2 * Vec2::from_angle(a2 + FRAC_PI_4) * wall::RADIUS
    } else {
        let mid = a2 + da3 / 2.;
        let len = wall::RADIUS / f32::sin(da3 / 2.);

        len * Vec2::from_angle(mid)
    };

    (i1, i2)
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
