use bevy::{
    ecs::{
        component::HookContext, entity::EntityHashMap, query::QueryEntityError,
        world::DeferredWorld,
    },
    prelude::*,
};
use pb_util::event::ComponentEvent;

use crate::{
    map::{Map, wall::Wall},
    root::ChildOfRoot,
};

pub const INNER_WIDTH: f32 = WIDTH - (2. * Wall::RADIUS);
pub const MIN_WIDTH: f32 = 0.9;
pub const WIDTH: f32 = 1.0;
pub const DEPTH: f32 = 0.2;
pub const MAX_WIDTH: f32 = 1.1;

pub const HALF_INNER_WIDTH: f32 = INNER_WIDTH / 2.;
pub const HALF_WIDTH: f32 = WIDTH / 2.;
pub const HALF_DEPTH: f32 = DEPTH / 2.;

#[derive(Clone, Debug, Component)]
#[component(immutable)]
pub struct Door;

#[derive(Clone, Debug, Component)]
#[component(immutable, on_insert = DoorLinks::on_insert, on_remove = DoorLinks::on_remove)]
pub struct DoorLinks {
    left: Entity,
    right: Entity,
}

#[derive(Default, Clone, Debug, Component)]
pub struct RoomLinks {
    doors: EntityHashMap<RoomLink>,
}

#[derive(Clone, Debug)]
pub struct RoomLink {
    position: Vec2,
    room: Entity,
}

pub fn validate(
    mut commands: Commands,
    mut door_e: EventReader<ComponentEvent<OnInsert, Door>>,
    wall_q: Query<&Wall>,
) {
    for door in door_e.read() {
        match wall_q.get(door.target) {
            Ok(wall) if wall.length() >= MIN_WIDTH => {}
            Err(QueryEntityError::EntityDoesNotExist(..)) => {}
            Err(QueryEntityError::AliasedMutability(..)) => unreachable!(),
            Ok(_) | Err(QueryEntityError::QueryDoesNotMatch(..)) => {
                commands.entity(door.target).try_remove::<Door>();
            }
        }
    }
}

pub fn wall_replaced(trigger: Trigger<OnReplace, Wall>, mut commands: Commands) {
    commands.entity(trigger.target()).try_remove::<DoorLinks>();
}

pub fn remove_links(
    map_q: Query<&Map, (Changed<Map>, With<ChildOfRoot>)>,
    door_q: Query<Entity, With<DoorLinks>>,
    mut commands: Commands,
) {
    map_q.iter().for_each(|map| {
        for wall in map.walls() {
            if door_q.contains(wall.id()) {
                commands.entity(wall.id()).remove::<DoorLinks>();
            }
        }
    });
}

pub fn add_links(
    door_q: Query<(Entity, &Wall, &ChildOf), (With<Door>, With<ChildOfRoot>, Without<DoorLinks>)>,
    map_q: Query<&Map>,
    mut commands: Commands,
) -> Result {
    for (id, wall, parent) in door_q {
        let map = map_q.get(parent.parent())?;
        let [left, right] = map.wall_rooms(wall);
        commands.entity(id).insert(DoorLinks { left, right });
    }

    Ok(())
}

impl RoomLinks {
    pub fn doors(&self) -> impl Iterator<Item = (Entity, Entity, Vec2)> {
        self.doors
            .iter()
            .map(|(&door, link)| (door, link.room, link.position))
    }
}

impl DoorLinks {
    fn on_insert(mut world: DeferredWorld, context: HookContext) {
        let door = world.entity(context.entity);
        let wall = door.get::<Wall>().unwrap().clone();
        let links = door.get::<DoorLinks>().unwrap().clone();

        world
            .entity_mut(links.left)
            .get_mut::<RoomLinks>()
            .unwrap()
            .doors
            .insert(
                context.entity,
                RoomLink {
                    position: wall.position(),
                    room: links.right,
                },
            );
        world
            .entity_mut(links.right)
            .get_mut::<RoomLinks>()
            .unwrap()
            .doors
            .insert(
                context.entity,
                RoomLink {
                    position: wall.position(),
                    room: links.left,
                },
            );
    }

    fn on_remove(mut world: DeferredWorld, context: HookContext) {
        let links = world
            .entity(context.entity)
            .get::<DoorLinks>()
            .unwrap()
            .clone();

        if let Ok(mut room) = world.get_entity_mut(links.left) {
            room.get_mut::<RoomLinks>()
                .unwrap()
                .doors
                .remove(&context.entity);
        }
        if let Ok(mut room) = world.get_entity_mut(links.right) {
            room.get_mut::<RoomLinks>()
                .unwrap()
                .doors
                .remove(&context.entity);
        }
    }
}
