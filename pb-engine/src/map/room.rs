use bevy::{ecs::entity::EntityHashSet, prelude::*};
use spade::handles::{FixedFaceHandle, PossiblyOuterTag};

use crate::{map::Map, pawn::Pawn, root::ChildOfRoot};

#[derive(Clone, Debug, Component)]
#[require(Transform, Visibility)]
#[component(immutable)]
pub struct Room {
    faces: Vec<FixedFaceHandle<PossiblyOuterTag>>,
}

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
    map_q: Query<&Map, With<ChildOfRoot>>,
    item_q: Query<(Entity, &Transform), (With<Pawn>, Without<ContainingRoom>, With<ChildOfRoot>)>,
) -> Result {
    'outer: for (item, transform) in &item_q {
        for map in &map_q {
            if let Some(room) = map.containing_room(transform.translation.xy()) {
                info!("found containing room {room}");
                commands.entity(item).insert(ContainingRoom(room));
                continue 'outer;
            }
        }

        warn!("no containing room found for {item}");
    }
    Ok(())
}

impl Room {
    pub(crate) fn faces(&self) -> &[FixedFaceHandle<PossiblyOuterTag>] {
        &self.faces
    }

    pub(crate) fn bundle(faces: Vec<FixedFaceHandle<PossiblyOuterTag>>) -> impl Bundle {
        (Name::new("room"), Room { faces })
    }
}
