use bevy::{ecs::system::RunSystemOnce, prelude::*};
use spade::Triangulation;

use crate::map::{self, Corner, CornerDef, Map, MapEntity, MapQueries, Room, Wall};

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
        .run_system_once(move |mut map: Single<&mut Map>, mut queries: MapQueries| {
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
        map.corners().count() + map.walls().count() + map.rooms_deduped().count()
    );

    for vertex in map.triangulation.vertices() {
        if let Some(corner) = vertex.data().corner {
            assert_eq!(
                world.entity(corner.id()).get::<Corner>().unwrap().vertex,
                vertex.fix()
            );
        }
    }

    for edge in map.triangulation.undirected_edges() {
        assert_eq!(edge.is_constraint_edge(), edge.data().data().wall.is_some());

        let directed_edge = edge.as_directed();
        if let Some(wall) = edge.data().data().wall {
            let wall = world.entity(wall.id()).get::<Wall>().unwrap();

            let corner1 = directed_edge.from().data().corner.unwrap().id();
            let corner2 = directed_edge.to().data().corner.unwrap().id();
            let room1 = directed_edge.face().data().room.unwrap().id();
            let room2 = directed_edge.rev().face().data().room.unwrap().id();

            assert_eq!(wall.corners(), [corner1, corner2]);
            assert_eq!(wall.rooms(), [room1, room2]);
        } else if !directed_edge.face().is_outer() && !directed_edge.rev().face().is_outer() {
            let room1 = directed_edge.face().data().room.unwrap().id();
            let room2 = directed_edge.rev().face().data().room.unwrap().id();
            assert_eq!(room1, room2);
        }
    }

    for face in map.triangulation.inner_faces() {
        let room = world
            .entity(face.data().room.unwrap().id())
            .get::<Room>()
            .unwrap();
        assert!(room.faces.contains(&face.fix()));

        for edge in face.adjacent_edges() {
            if !edge.as_undirected().is_constraint_edge() && !edge.rev().face().is_outer() {
                assert_eq!(
                    face.data().room.unwrap().id(),
                    edge.rev().face().data().room.unwrap().id()
                );
            }
        }
    }

    for (corner_id, corner) in world
        .iter_entities()
        .filter_map(|e| e.get::<Corner>().map(|c| (e.id(), c)))
    {
        assert_eq!(
            map.triangulation.vertex(corner.vertex).data().corner,
            Some(MapEntity::Owned(corner_id))
        );
    }

    for (wall_id, wall) in world
        .iter_entities()
        .filter_map(|e| e.get::<Wall>().map(|w| (e.id(), w)))
    {
        let edge = map.triangulation.undirected_edge(wall.edge);
        assert!(edge.is_constraint_edge());
        assert_eq!(edge.data().data().wall, Some(MapEntity::Owned(wall_id)));
    }

    for (room_id, _) in world
        .iter_entities()
        .filter_map(|e| e.get::<Room>().map(|r| (e.id(), r)))
    {
        assert!(
            map.triangulation
                .all_faces()
                .any(|face| face.data().room == Some(MapEntity::Owned(room_id)))
        );
    }
}
