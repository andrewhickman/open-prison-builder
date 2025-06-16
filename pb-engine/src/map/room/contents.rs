use bevy::{
    ecs::{entity::EntityHashSet, relationship::Relationship},
    prelude::*,
};
use spade::handles::FixedVertexHandle;

use crate::{
    map::{Map, room::Room},
    pawn::Pawn,
    root::ChildOfRoot,
};

#[derive(Component, Clone, PartialEq, Eq, Debug)]
#[relationship(relationship_target = RoomContents)]
pub struct ContainingRoom {
    #[relationship]
    room: Entity,
    hint: Option<FixedVertexHandle>,
}

#[derive(Component, Default, Debug, PartialEq, Eq)]
#[relationship_target(relationship = ContainingRoom)]
pub struct RoomContents(EntityHashSet);

pub fn room_replaced(trigger: Trigger<OnReplace, Room>, mut commands: Commands) {
    commands
        .entity(trigger.target())
        .try_remove::<RoomContents>();
}

pub fn update(
    commands: ParallelCommands,
    map_q: Query<&Map, With<ChildOfRoot>>,
    item_q: Query<
        (Entity, &Transform, Option<&ContainingRoom>),
        (
            With<Pawn>,
            With<ChildOfRoot>,
            Or<(Without<ContainingRoom>, Changed<Transform>)>,
        ),
    >,
) {
    item_q
        .par_iter()
        .for_each(|(id, transform, containing_room)| {
            let hint = containing_room.and_then(|prev_room| prev_room.hint);
            for map in &map_q {
                if let Some((room, hint)) = map.containing_room(transform.translation.xy(), hint) {
                    if containing_room.is_none_or(|prev_room| prev_room.get() != room) {
                        info!("updated containing room {room} for {id}");
                        commands.command_scope(|mut commands| {
                            commands.entity(id).insert(ContainingRoom {
                                room,
                                hint: Some(hint),
                            });
                        });
                    }
                    return;
                }
            }

            warn!("no containing room found for {id}");
            if containing_room.is_some() {
                commands.command_scope(|mut commands| {
                    commands.entity(id).remove::<ContainingRoom>();
                });
            }
        });
}
