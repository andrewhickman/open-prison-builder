use avian2d::prelude::*;
use bevy::{
    ecs::{entity::EntityHashMap, system::SystemParam},
    prelude::*,
};
use glam::Vec2;
use serde::{Deserialize, Serialize};

use crate::{
    EngineState,
    map::{Map, corner::Corner, room::Room, wall::Wall},
    pawn::{Pawn, PawnBundle},
    root::Root,
};

#[derive(SystemParam)]
pub struct SaveParam<'w, 's> {
    state: Res<'w, State<EngineState>>,
    pawn_q: Query<
        'w,
        's,
        (
            Entity,
            &'static Pawn,
            &'static ChildOf,
            &'static Position,
            &'static Rotation,
            &'static LinearVelocity,
            &'static AngularVelocity,
        ),
    >,
    map_q: Query<'w, 's, (Entity, &'static Map, &'static ChildOf)>,
    corner_q: Query<'w, 's, &'static Corner>,
    wall_q: Query<'w, 's, &'static Wall>,
    room_q: Query<'w, 's, &'static Room>,
}

#[derive(Debug, Serialize, Deserialize, TypePath)]
pub struct SaveModel {
    pub pawns: Vec<PawnModel>,
    pub maps: Vec<MapModel>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PawnModel {
    pub id: Entity,
    pub position: Vec2,
    pub rotation: f32,
    pub linear_velocity: Vec2,
    pub angular_velocity: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MapModel {
    pub id: Entity,
    pub corners: Vec<CornerModel>,
    pub walls: Vec<WallModel>,
    pub rooms: Vec<RoomModel>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CornerModel {
    pub id: Entity,
    pub position: Vec2,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WallModel {
    pub id: Entity,
    pub corners: [Entity; 2],
    pub rooms: [Entity; 2],
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RoomModel {
    pub id: Entity,
}

impl SaveParam<'_, '_> {
    pub fn save(&self) -> Result<SaveModel> {
        let &EngineState::Running(root) = self.state.get() else {
            return Err("no active game".into());
        };

        let pawns = self
            .pawn_q
            .iter()
            .filter(|(_, _, parent, _, _, _, _)| parent.parent() == root)
            .map(
                |(id, _, _, position, rotation, linear_velocity, angular_velocity)| PawnModel {
                    id,
                    position: position.0,
                    rotation: rotation.as_radians(),
                    linear_velocity: linear_velocity.0,
                    angular_velocity: angular_velocity.0,
                },
            )
            .collect();
        let maps = self
            .map_q
            .iter()
            .filter(|(_, _, parent)| parent.parent() == root)
            .map(|(id, map, _)| {
                let corners = map
                    .corners()
                    .map(|id| {
                        let corner = self.corner_q.get(id.id())?;
                        Ok(CornerModel {
                            id: id.id(),
                            position: corner.position(),
                        })
                    })
                    .collect::<Result<Vec<_>>>()?;
                let walls = map
                    .walls()
                    .map(|id| {
                        let wall = self.wall_q.get(id.id())?;
                        Ok(WallModel {
                            id: id.id(),
                            corners: wall.corners(),
                            rooms: map.wall_rooms(wall),
                        })
                    })
                    .collect::<Result<Vec<_>>>()?;
                let rooms = map
                    .rooms_deduped()
                    .map(|id| {
                        let _room = self.room_q.get(id.id())?;
                        Ok(RoomModel { id: id.id() })
                    })
                    .collect::<Result<Vec<_>>>()?;

                Ok(MapModel {
                    id,
                    corners,
                    walls,
                    rooms,
                })
            })
            .collect::<Result<Vec<_>>>()?;

        Ok(SaveModel { pawns, maps })
    }
}

impl SaveModel {
    pub fn spawn(self, commands: &mut Commands) -> Entity {
        let root = commands.spawn(Root).id();
        let mut entity_map = EntityHashMap::<Entity>::new();

        commands.queue(move |world: &mut World| -> Result {
            for (entity, pawn) in world
                .spawn_batch(
                    self.pawns
                        .iter()
                        .map(|pawn| (PawnBundle::new(pawn.position, pawn.rotation), ChildOf(root))),
                )
                .zip(&self.pawns)
            {
                entity_map.insert(pawn.id, entity);
            }

            for map in &self.maps {
                let map_id = world.spawn(ChildOf(root)).id();

                for (entity, corner) in world
                    .spawn_batch(map.corners.iter().map(|_| ChildOf(map_id)))
                    .zip(&map.corners)
                {
                    entity_map.insert(corner.id, entity);
                }

                for (entity, room) in world
                    .spawn_batch(map.rooms.iter().map(|_| ChildOf(map_id)))
                    .zip(&map.rooms)
                {
                    entity_map.insert(room.id, entity);
                }

                for (entity, wall) in world
                    .spawn_batch(map.walls.iter().map(|_| ChildOf(map_id)))
                    .zip(&map.walls)
                {
                    entity_map.insert(wall.id, entity);
                }

                world
                    .entity_mut(map_id)
                    .insert(Map::from_model(map, &mut entity_map)?);
            }

            Ok(())
        });

        root
    }
}
