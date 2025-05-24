use bevy::prelude::*;
use pb_engine::map::{CornerDef, Map};
use pb_render::wall::VisibleMap;

use crate::{
    action::Action,
    input::{
        cancel::Cancellable,
        picking::{
            physics::{
                PhysicsPickingState,
                wall::{CancelWall, ClickWall, SelectWall, WallPickKind},
            },
            point::grid::Grid,
        },
    },
    ribbon::architect::map::MapParam,
};

pub fn add_door(
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
            AddDoorAction::default(),
            children![
                Grid::default(),
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
    Name = Name::new(AddDoorAction::type_path()),
    PhysicsPickingState = PhysicsPickingState::Wall,
    Transform,
    Visibility,
)]
pub enum AddDoorAction {
    #[default]
    SelectWall,
}

fn select_wall(
    trigger: Trigger<SelectWall>,
    mut action: Single<&mut AddDoorAction>,
    mut map: MapParam,
) -> Result {
    match trigger.kind {
        WallPickKind::Corner { corner, .. } => {
            action.select_corner(&mut map, CornerDef::Corner(corner))
        }
        WallPickKind::Wall { position, .. } => {
            action.select_corner(&mut map, CornerDef::Wall(trigger.wall, position))
        }
    }
}

fn cancel_wall(
    _: Trigger<CancelWall>,
    mut action: Single<&mut AddDoorAction>,
    mut map: MapParam,
) -> Result {
    action.cancel(&mut map)
}

fn click_wall(
    trigger: Trigger<ClickWall>,
    mut action: Single<&mut AddDoorAction>,
    mut map: MapParam,
) -> Result {
    match trigger.kind {
        WallPickKind::Corner { corner, .. } => action.click(&mut map, CornerDef::Corner(corner)),
        WallPickKind::Wall { position, .. } => {
            action.click(&mut map, CornerDef::Wall(trigger.wall, position))
        }
    }
}

impl AddDoorAction {
    fn select_corner(&mut self, _map: &mut MapParam, _corner: CornerDef) -> Result {
        Ok(())
    }

    fn click(&mut self, _map: &mut MapParam, _corner: CornerDef) -> Result {
        Ok(())
    }

    fn cancel(&mut self, _map: &mut MapParam) -> Result {
        Ok(())
    }
}
