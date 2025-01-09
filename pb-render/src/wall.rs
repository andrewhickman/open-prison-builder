use std::{
    convert::identity,
    f32::consts::{FRAC_PI_2, FRAC_PI_4, PI, SQRT_2, TAU},
};

use approx::relative_ne;
use bevy::{
    math::FloatOrd,
    prelude::*,
    render::{mesh::PrimitiveTopology, render_asset::RenderAssetUsages},
    utils::hashbrown::HashSet,
};
use pb_engine::wall::{self, Vertex, Wall};
use pb_util::{try_opt, try_res_s};
use smallvec::SmallVec;

#[derive(Debug, Default, Component)]
pub struct VertexMesh {
    pos: Vec2,
    walls: SmallVec<[(Entity, Vec2); 4]>,
}

#[derive(Debug, Default, Component)]
pub struct WallMesh {
    intersections: [Vec2; 4],
}

#[derive(Event, Debug, Clone, Copy)]
pub struct WallChanged {
    id: Entity,
    wall: Wall,
}

pub const WHITE: Handle<ColorMaterial> =
    Handle::weak_from_u128(146543197086297070279747770654600266484);

pub fn startup(mut assets: ResMut<Assets<ColorMaterial>>) {
    assets.insert(&WHITE, ColorMaterial::from_color(Color::WHITE));
}

pub fn init_vertex(
    trigger: Trigger<OnAdd, Vertex>,
    mut commands: Commands,
    transform_q: Query<&Transform>,
) {
    commands.entity(trigger.entity()).insert((
        VertexMesh {
            pos: try_res_s!(transform_q.get(trigger.entity()))
                .translation
                .xy(),
            walls: default(),
        },
        MeshMaterial2d(WHITE.clone()),
        Mesh2d::default(),
        Visibility::default(),
    ));
}

pub fn init_wall(
    trigger: Trigger<OnAdd, Wall>,
    mut commands: Commands,
    mut wall_added_e: EventWriter<WallChanged>,
    wall_q: Query<&Wall>,
) {
    let id = trigger.entity();
    commands.entity(id).insert((
        Transform::default(),
        WallMesh::default(),
        MeshMaterial2d(WHITE.clone()),
        Mesh2d::default(),
        Visibility::default(),
    ));

    wall_added_e.send(WallChanged {
        id,
        wall: *wall_q.get(id).unwrap(),
    });
}

pub fn update_wall(
    assets: Res<AssetServer>,
    mut wall_added_e: EventReader<WallChanged>,
    mut vertex_q: Query<(&Transform, &mut VertexMesh, &mut Mesh2d), With<Vertex>>,
    mut wall_mesh_q: Query<(&Wall, &mut WallMesh, &mut Mesh2d), Without<Vertex>>,
) {
    let mut updated_walls = HashSet::new();
    for event in wall_added_e.read() {
        let [(start_tr, mut start_mesh, mut start_handle), (end_tr, mut end_mesh, mut end_handle)] =
            vertex_q.many_mut([event.wall.start, event.wall.end]);

        let start_position = start_tr.translation.xy();
        let end_position = end_tr.translation.xy();

        start_mesh.insert_wall(event.id, end_position);
        end_mesh.insert_wall(event.id, start_position);

        start_handle.0 = assets.add(start_mesh.mesh());
        end_handle.0 = assets.add(end_mesh.mesh());

        updated_walls.extend(start_mesh.walls.iter().map(|&(id, _)| id));
        updated_walls.extend(end_mesh.walls.iter().map(|&(id, _)| id));
    }

    for id in updated_walls {
        let (wall, mut mesh, mut handle) = try_res_s!(wall_mesh_q.get_mut(id));
        let [(_, start_mesh, _), (_, end_mesh, _)] = vertex_q.many([wall.start, wall.end]);

        mesh.update(id, start_mesh, end_mesh);
        handle.0 = assets.add(mesh.mesh());
    }
}

impl VertexMesh {
    fn insert_wall(&mut self, id: Entity, pos: Vec2) -> usize {
        let index = self
            .walls
            .binary_search_by_key(&FloatOrd((pos - self.pos).to_angle()), |&(_, other)| {
                FloatOrd((other - self.pos).to_angle())
            })
            .unwrap_or_else(identity);

        self.walls.insert(index, (id, pos));
        index
    }

    fn wall_intersection(&self, id: Entity) -> Option<(Vec2, Vec2)> {
        let index = self.walls.iter().position(|&(w, _)| w == id)?;
        let p1 = wrapping_idx(&self.walls, index, -1).1;
        let p2 = wrapping_idx(&self.walls, index, 0).1;
        let p3 = wrapping_idx(&self.walls, index, 1).1;

        let (i1, i2) = wall_intersection(p1 - self.pos, p2 - self.pos, p3 - self.pos);
        Some((self.pos + i1, self.pos + i2))
    }

    fn mesh(&self) -> Mesh {
        let mut intersections = SmallVec::<[Vec2; 4]>::new();

        for (i, &(_, pos1)) in self.walls.iter().enumerate() {
            let pos2 = wrapping_idx(&self.walls, i, 1).1;

            intersections.extend(vertex_intersections(pos1 - self.pos, pos2 - self.pos));
        }

        let mut vertices = Vec::new();
        for (i, v1) in intersections.iter().enumerate() {
            let v2 = intersections.get(i + 1).unwrap_or(&intersections[0]);
            vertices.extend([v1.extend(0.), Vec3::ZERO, v2.extend(0.)]);
        }

        Mesh::new(
            PrimitiveTopology::TriangleList,
            RenderAssetUsages::RENDER_WORLD,
        )
        .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, vertices)
    }
}

impl WallMesh {
    fn update(&mut self, id: Entity, start: &VertexMesh, end: &VertexMesh) {
        let (start1, start2) = try_opt!(start.wall_intersection(id));
        let (end1, end2) = try_opt!(end.wall_intersection(id));

        self.intersections = [start1, start2, end1, end2];
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

fn vertex_intersections(p1: Vec2, p2: Vec2) -> SmallVec<[Vec2; 5]> {
    let mut a1 = p1.to_angle();
    let mut a2 = p2.to_angle();
    let mut da = angle_delta(a1, a2);
    let reflex = da >= PI;

    let mut result = SmallVec::new();

    while da >= PI {
        result.insert(
            result.len() / 2,
            SQRT_2 * Vec2::from_angle(a1 + FRAC_PI_4) * wall::RADIUS,
        );
        result.insert(
            result.len() / 2,
            SQRT_2 * Vec2::from_angle(a2 - FRAC_PI_4) * wall::RADIUS,
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

fn wall_intersection(p1: Vec2, p2: Vec2, p3: Vec2) -> (Vec2, Vec2) {
    let a1 = p1.to_angle();
    let a2 = p2.to_angle();
    let a3 = p3.to_angle();

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
