use bevy::prelude::*;

use pb_engine::map::Map;
use pb_render::wall::{MapRenderMode, VisibleMap, WallMaterial};

use crate::{
    action::Action,
    input::{
        cancel::Cancellable,
        picking::physics::{
            PhysicsPickingState,
            wall::{CancelWall, ClickWall, SelectWall},
        },
    },
    ribbon::architect::wall::MapParam,
};

pub fn remove_wall(
    _: Trigger<Pointer<Click>>,
    mut commands: Commands,
    mut visible_map: ResMut<VisibleMap>,
    map_q: Query<&Map>,
) -> Result {
    let Some(source_id) = visible_map.source().or_else(|| visible_map.id()) else {
        return Ok(());
    };
    let source = map_q.get(source_id)?;
    assert_eq!(source.id(), source_id);

    let id = commands
        .spawn((
            RemoveWallAction,
            children![
                Observer::new(select_wall),
                Observer::new(cancel_wall),
                Observer::new(click_wall),
            ],
        ))
        .id();
    let map = commands.spawn((source.cloned(), ChildOf(id))).id();
    visible_map.set(map, Some(source.id()));
    Ok(())
}

#[derive(Default, Debug, Component, TypePath)]
#[require(
    Action,
    Cancellable,
    Name = Name::new(RemoveWallAction::type_path()),
    PhysicsPickingState = PhysicsPickingState::Wall,
    Transform,
    Visibility,
)]
pub struct RemoveWallAction;

fn select_wall(
    trigger: Trigger<SelectWall>,
    mut render_mode_q: Query<&mut MeshMaterial2d<WallMaterial>>,
) -> Result {
    render_mode_q
        .get_mut(trigger.wall)?
        .set_if_neq(MapRenderMode::Removed.material());
    Ok(())
}

fn cancel_wall(
    trigger: Trigger<CancelWall>,
    mut render_mode_q: Query<&mut MeshMaterial2d<WallMaterial>>,
) -> Result {
    render_mode_q
        .get_mut(trigger.wall)?
        .set_if_neq(MapRenderMode::Visible.material());
    Ok(())
}

fn click_wall(trigger: Trigger<ClickWall>, mut map: MapParam) -> Result {
    map.reset()?;
    map.remove_wall(trigger.wall)?;
    map.commit()
}
