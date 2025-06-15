use bevy::{ecs::system::RunSystemOnce, prelude::*};
use spade::Triangulation;

use crate::map::{self, Corner, CornerDef, Map, MapParam, Room, Wall, perimeter::Perimeter};

#[test]
fn test_empty() {
    let (world, _) = create_map();
    assert_consistency(&world);
}

#[test]
fn test_add_wall() {
    let (mut world, map_id) = create_map();

    insert_wall(
        &mut world,
        CornerDef::Position(Vec2::new(0., 0.)),
        CornerDef::Position(Vec2::new(1., 0.)),
    );

    let map = world.entity(map_id).get::<Map>().unwrap();
    assert_eq!(map.corners().count(), 2);
    assert_eq!(map.walls().count(), 1);
    assert_eq!(map.rooms_deduped().count(), 1);

    assert_consistency(&world);
}

#[test]
fn test_add_two_walls() {
    let (mut world, map_id) = create_map();

    insert_wall(
        &mut world,
        CornerDef::Position(Vec2::new(0., 0.)),
        CornerDef::Position(Vec2::new(1., 0.)),
    );
    insert_wall(
        &mut world,
        CornerDef::Position(Vec2::new(-1., 0.)),
        CornerDef::Position(Vec2::new(0., 0.)),
    );

    let map = world.entity(map_id).get::<Map>().unwrap();
    assert_eq!(map.corners().count(), 3);
    assert_eq!(map.walls().count(), 2);
    assert_eq!(map.rooms_deduped().count(), 1);

    assert_consistency(&world);
}

#[test]
fn test_add_room() {
    let (mut world, map_id) = create_map();

    insert_wall(
        &mut world,
        CornerDef::Position(Vec2::new(-1., 1.)),
        CornerDef::Position(Vec2::new(1., 1.)),
    );
    insert_wall(
        &mut world,
        CornerDef::Position(Vec2::new(1., 1.)),
        CornerDef::Position(Vec2::new(1., -1.)),
    );
    insert_wall(
        &mut world,
        CornerDef::Position(Vec2::new(1., -1.)),
        CornerDef::Position(Vec2::new(-1., -1.)),
    );
    insert_wall(
        &mut world,
        CornerDef::Position(Vec2::new(-1., -1.)),
        CornerDef::Position(Vec2::new(-1., 1.)),
    );

    let map = world.entity(map_id).get::<Map>().unwrap();
    assert_eq!(map.corners().count(), 4);
    assert_eq!(map.walls().count(), 4);
    assert_eq!(map.rooms_deduped().count(), 2);

    assert_consistency(&world);
}

#[test]
fn test_add_two_room() {
    let (mut world, map_id) = create_map();

    insert_wall(
        &mut world,
        CornerDef::Position(Vec2::new(-1., 1.)),
        CornerDef::Position(Vec2::new(0., 1.)),
    );
    insert_wall(
        &mut world,
        CornerDef::Position(Vec2::new(0., 1.)),
        CornerDef::Position(Vec2::new(0., -1.)),
    );
    insert_wall(
        &mut world,
        CornerDef::Position(Vec2::new(0., -1.)),
        CornerDef::Position(Vec2::new(-1., -1.)),
    );
    insert_wall(
        &mut world,
        CornerDef::Position(Vec2::new(-1., -1.)),
        CornerDef::Position(Vec2::new(-1., 1.)),
    );
    insert_wall(
        &mut world,
        CornerDef::Position(Vec2::new(0., 1.)),
        CornerDef::Position(Vec2::new(1., 1.)),
    );
    insert_wall(
        &mut world,
        CornerDef::Position(Vec2::new(0., 1.)),
        CornerDef::Position(Vec2::new(1., 1.)),
    );
    insert_wall(
        &mut world,
        CornerDef::Position(Vec2::new(1., 1.)),
        CornerDef::Position(Vec2::new(1., -1.)),
    );
    insert_wall(
        &mut world,
        CornerDef::Position(Vec2::new(1., -1.)),
        CornerDef::Position(Vec2::new(0., -1.)),
    );

    let map = world.entity(map_id).get::<Map>().unwrap();
    assert_eq!(map.corners().count(), 6);
    assert_eq!(map.walls().count(), 7);
    assert_eq!(map.rooms_deduped().count(), 3);

    assert_consistency(&world);
}

#[test]
fn test_add_wall_conflict() {
    let (mut world, map_id) = create_map();

    insert_wall(
        &mut world,
        CornerDef::Position(Vec2::new(0., 1.)),
        CornerDef::Position(Vec2::new(0., -1.)),
    );
    insert_wall(
        &mut world,
        CornerDef::Position(Vec2::new(-1., 0.)),
        CornerDef::Position(Vec2::new(1., 0.)),
    );

    let map = world.entity(map_id).get::<Map>().unwrap();
    assert_eq!(map.corners().count(), 5);
    assert_eq!(map.walls().count(), 4);
    assert_eq!(map.rooms_deduped().count(), 1);

    assert_consistency(&world);
}

#[test]
fn test_split_room() {
    let (mut world, map_id) = create_map();

    insert_wall(
        &mut world,
        CornerDef::Position(Vec2::new(-1., 1.)),
        CornerDef::Position(Vec2::new(0., 1.)),
    );
    insert_wall(
        &mut world,
        CornerDef::Position(Vec2::new(0., -1.)),
        CornerDef::Position(Vec2::new(-1., -1.)),
    );
    insert_wall(
        &mut world,
        CornerDef::Position(Vec2::new(-1., -1.)),
        CornerDef::Position(Vec2::new(-1., 1.)),
    );
    insert_wall(
        &mut world,
        CornerDef::Position(Vec2::new(0., 1.)),
        CornerDef::Position(Vec2::new(1., 1.)),
    );
    insert_wall(
        &mut world,
        CornerDef::Position(Vec2::new(0., 1.)),
        CornerDef::Position(Vec2::new(1., 1.)),
    );
    insert_wall(
        &mut world,
        CornerDef::Position(Vec2::new(1., 1.)),
        CornerDef::Position(Vec2::new(1., -1.)),
    );
    insert_wall(
        &mut world,
        CornerDef::Position(Vec2::new(1., -1.)),
        CornerDef::Position(Vec2::new(0., -1.)),
    );
    insert_wall(
        &mut world,
        CornerDef::Position(Vec2::new(0., 1.)),
        CornerDef::Position(Vec2::new(0., -1.)),
    );

    let map = world.entity(map_id).get::<Map>().unwrap();
    assert_eq!(map.corners().count(), 6);
    assert_eq!(map.walls().count(), 7);
    assert_eq!(map.rooms_deduped().count(), 3);

    assert_consistency(&world);
}

fn create_map() -> (World, Entity) {
    let mut world = World::new();
    world.add_observer(map::map_inserted);
    let map = world.spawn(Map::new()).id();
    (world, map)
}

fn insert_wall(world: &mut World, start: CornerDef, end: CornerDef) {
    world
        .run_system_once(move |mut map: Single<&mut Map>, mut queries: MapParam| {
            map.insert_wall(&mut queries, start, end).unwrap();
        })
        .unwrap();
}

fn assert_consistency(world: &World) {
    let map = world.iter_entities().find(|e| e.contains::<Map>()).unwrap();
    let children = map
        .get::<Children>()
        .map(|c| c.to_vec())
        .unwrap_or_default();
    let map = map.get::<Map>().unwrap();

    assert_eq!(
        map.corners().count(),
        world
            .iter_entities()
            .filter(|e| e.contains::<Corner>())
            .count()
    );
    assert_eq!(
        map.walls().count(),
        world
            .iter_entities()
            .filter(|e| e.contains::<Wall>())
            .count()
    );
    assert_eq!(
        map.rooms_deduped().count(),
        world
            .iter_entities()
            .filter(|e| e.contains::<Room>())
            .count()
    );

    assert_eq!(children.len(), map.children.len());
    assert_eq!(
        children.len(),
        map.corners().count()
            + map.walls().count()
            + map.rooms_deduped().count()
            + map.perimeter().count()
    );

    for vertex in map.triangulation.vertices() {
        if let Some(corner) = vertex.data().corner {
            assert_eq!(
                world.entity(corner.id()).get::<Corner>().unwrap().vertex(),
                vertex.fix()
            );
        }
    }

    for edge in map.triangulation.undirected_edges() {
        assert_eq!(
            edge.is_constraint_edge() || edge.is_part_of_convex_hull(),
            edge.data().data().wall.is_some()
        );

        if edge.is_constraint_edge() {
            let directed_edge = edge.as_directed();
            let wall = world
                .entity(edge.data().data().wall())
                .get::<Wall>()
                .unwrap();

            let corner1 = directed_edge.from().data().corner();
            let corner2 = directed_edge.to().data().corner();

            assert_eq!(wall.corners(), [corner1, corner2]);
        }
    }

    for edge in map.triangulation.convex_hull() {
        let undirected_edge = edge.as_undirected();

        assert!(!undirected_edge.is_constraint_edge());

        let perimeter = world
            .entity(undirected_edge.data().data().wall())
            .get::<Perimeter>()
            .unwrap();
        assert_eq!(perimeter.start(), edge.from().data().position);
        assert_eq!(perimeter.end(), edge.to().data().position);
    }

    for face in map.triangulation.all_faces() {
        if let Some(inner_face) = face.as_inner() {
            for edge in inner_face.adjacent_edges() {
                if !edge.as_undirected().is_constraint_edge() && !edge.is_part_of_convex_hull() {
                    assert_eq!(face.data().room(), edge.rev().face().data().room());
                }
            }
        }
    }

    for (corner_id, corner) in world
        .iter_entities()
        .filter_map(|e| e.get::<Corner>().map(|c| (e.id(), c)))
    {
        assert_eq!(
            map.triangulation.vertex(corner.vertex()).data().corner(),
            corner_id
        );
    }

    for (wall_id, wall) in world
        .iter_entities()
        .filter_map(|e| e.get::<Wall>().map(|w| (e.id(), w)))
    {
        let edge = map.triangulation.undirected_edge(wall.edge());
        assert!(edge.is_constraint_edge());
        assert_eq!(edge.data().data().wall(), wall_id);
    }

    for (room_id, _) in world
        .iter_entities()
        .filter_map(|e| e.get::<Room>().map(|r| (e.id(), r)))
    {
        assert!(
            map.triangulation
                .all_faces()
                .any(|face| face.data().room() == room_id)
        );
    }
}
