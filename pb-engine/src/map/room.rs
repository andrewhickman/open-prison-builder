use bevy::{ecs::entity::EntityHashSet, prelude::*};

use crate::{
    map::{Map, Room},
    pawn::Pawn,
    root::RootQuery,
};

#[derive(Component, Clone, PartialEq, Eq, Debug)]
#[relationship(relationship_target = RoomContents)]
pub struct ContainingRoom(pub Entity);

#[derive(Component, Default, Debug, PartialEq, Eq)]
#[relationship_target(relationship = ContainingRoom)]
pub struct RoomContents(EntityHashSet);

pub fn room_replaced(trigger: Trigger<OnReplace, Room>, mut commands: Commands) {
    commands
        .entity(trigger.target())
        .try_remove::<RoomContents>();
}

pub fn update_containing_room(
    mut commands: Commands,
    root_q: RootQuery,
    map_q: Query<(Entity, &Map)>,
    item_q: Query<(Entity, &Transform), (With<Pawn>, Without<ContainingRoom>)>,
) -> Result {
    'outer: for (item, transform) in &item_q {
        if root_q.is_descendant_of_root(item) {
            for (map_id, map) in &map_q {
                if root_q.is_descendant_of_root(map_id) {
                    if let Some(room) = map.containing_room(transform.translation.xy()) {
                        info!("found containing room {room}");
                        commands.entity(item).insert(ContainingRoom(room));
                        continue 'outer;
                    }
                }
            }

            warn!("no containing room found for {item}");
        }
    }
    Ok(())
}
