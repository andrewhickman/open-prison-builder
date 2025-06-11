use bevy::prelude::*;

use pb_engine::map::door::Door;
use pb_render::wall::{MapRenderMode, WallMaterial};

use crate::{
    action::Action,
    input::{
        cancel::Cancellable,
        picking::physics::{
            PhysicsPickingState,
            wall::{CancelWall, ClickWall, SelectWall},
        },
    },
    ribbon::architect::map::MapParam,
};

pub fn remove_wall(_: Trigger<Pointer<Click>>, mut commands: Commands) -> Result {
    commands.spawn((
        RemoveWallAction,
        children![
            Observer::new(select_wall),
            Observer::new(cancel_wall),
            Observer::new(click_wall),
        ],
    ));
    Ok(())
}

#[derive(Default, Debug, Component, TypePath)]
#[require(
    Action,
    Cancellable,
    Name::new(RemoveWallAction::type_path()),
    PhysicsPickingState::Wall,
    Transform,
    Visibility
)]
pub struct RemoveWallAction;

fn select_wall(
    trigger: Trigger<SelectWall>,
    mut material_q: Query<&mut MeshMaterial2d<WallMaterial>>,
    door_q: Query<Entity, With<Door>>,
) -> Result {
    material_q
        .get_mut(trigger.wall)?
        .set_if_neq(MapRenderMode::Removed.material(door_q.contains(trigger.wall)));
    Ok(())
}

fn cancel_wall(
    trigger: Trigger<CancelWall>,
    mut material_q: Query<&mut MeshMaterial2d<WallMaterial>>,
    door_q: Query<Entity, With<Door>>,
) -> Result {
    material_q
        .get_mut(trigger.wall)?
        .set_if_neq(MapRenderMode::Visible.material(door_q.contains(trigger.wall)));
    Ok(())
}

fn click_wall(trigger: Trigger<ClickWall>, mut map: MapParam) -> Result {
    map.remove_wall(trigger.wall)
}
