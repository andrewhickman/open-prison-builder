use bevy::prelude::*;
use pb_engine::map::{CornerDef, Map};
use pb_render::wall::VisibleMaps;

use crate::{
    action::Action,
    input::{
        cancel::Cancellable,
        picking::{
            physics::{
                PhysicsPickingState,
                corner::{CancelCorner, ClickCorner, SelectCorner},
                wall::{CancelWall, ClickWall, SelectWall},
            },
            point::{CancelPoint, ClickPoint, SelectPoint, grid::Grid},
        },
    },
    ribbon::architect::map::MapParam,
};

pub fn add_wall(
    _: Trigger<Pointer<Click>>,
    mut commands: Commands,
    mut visible_map: ResMut<VisibleMaps>,
    map_q: Query<&Map>,
) -> Result {
    let Some(source_id) = visible_map.source() else {
        return Ok(());
    };
    let source = map_q.get(source_id)?;
    assert_eq!(source.id(), source_id);

    let id = commands
        .spawn((
            AddWallAction::default(),
            children![
                Grid::default(),
                Observer::new(select_point),
                Observer::new(cancel_point),
                Observer::new(click_point),
                Observer::new(select_corner),
                Observer::new(cancel_corner),
                Observer::new(click_corner),
                Observer::new(select_wall),
                Observer::new(cancel_wall),
                Observer::new(click_wall),
            ],
        ))
        .id();
    let map = commands.spawn((source.cloned(), ChildOf(id))).id();
    *visible_map = VisibleMaps::Preview {
        map,
        source: source.id(),
    };
    Ok(())
}

#[derive(Default, Debug, Component, TypePath)]
#[require(
    Action,
    Cancellable,
    Name::new(AddWallAction::type_path()),
    PhysicsPickingState::SnapWall,
    Transform,
    Visibility
)]
pub enum AddWallAction {
    #[default]
    SelectStart,
    SelectEnd {
        start: CornerDef,
    },
}

fn select_point(
    trigger: Trigger<SelectPoint>,
    mut action: Single<&mut AddWallAction>,
    mut map: MapParam,
) -> Result {
    action.select_corner(&mut map, CornerDef::Position(trigger.point))
}

fn cancel_point(
    _: Trigger<CancelPoint>,
    mut action: Single<&mut AddWallAction>,
    mut map: MapParam,
) -> Result {
    action.cancel(&mut map)
}

fn click_point(
    trigger: Trigger<ClickPoint>,
    mut action: Single<&mut AddWallAction>,
    mut map: MapParam,
) -> Result {
    action.click(&mut map, CornerDef::Position(trigger.point))
}

fn select_wall(
    trigger: Trigger<SelectWall>,
    mut action: Single<&mut AddWallAction>,
    mut map: MapParam,
) -> Result {
    action.select_corner(&mut map, CornerDef::Wall(trigger.wall, trigger.position))
}

fn cancel_wall(
    _: Trigger<CancelWall>,
    mut action: Single<&mut AddWallAction>,
    mut map: MapParam,
) -> Result {
    action.cancel(&mut map)
}

fn click_wall(
    trigger: Trigger<ClickWall>,
    mut action: Single<&mut AddWallAction>,
    mut map: MapParam,
) -> Result {
    action.click(&mut map, CornerDef::Wall(trigger.wall, trigger.position))
}

fn select_corner(
    trigger: Trigger<SelectCorner>,
    mut action: Single<&mut AddWallAction>,
    mut map: MapParam,
) -> Result {
    action.select_corner(&mut map, CornerDef::Corner(trigger.corner))
}

fn cancel_corner(
    _: Trigger<CancelCorner>,
    mut action: Single<&mut AddWallAction>,
    mut map: MapParam,
) -> Result {
    action.cancel(&mut map)
}

fn click_corner(
    trigger: Trigger<ClickCorner>,
    mut action: Single<&mut AddWallAction>,
    mut map: MapParam,
) -> Result {
    action.click(&mut map, CornerDef::Corner(trigger.corner))
}

impl AddWallAction {
    fn select_corner(&mut self, map: &mut MapParam, corner: CornerDef) -> Result {
        map.reset()?;

        match *self {
            AddWallAction::SelectStart => {
                map.insert_corner(corner)?;
            }
            AddWallAction::SelectEnd { start } => {
                map.insert_wall(start, corner)?;
            }
        }

        Ok(())
    }

    fn click(&mut self, map: &mut MapParam, corner: CornerDef) -> Result {
        map.reset()?;

        match *self {
            AddWallAction::SelectStart => {
                map.insert_corner(corner)?;
                *self = AddWallAction::SelectEnd { start: corner }
            }
            AddWallAction::SelectEnd { start } => {
                if let Some((_, end)) = map.insert_wall(start, corner)? {
                    map.commit()?;
                    *self = AddWallAction::SelectEnd {
                        start: CornerDef::Corner(end),
                    }
                }
            }
        }

        Ok(())
    }

    fn cancel(&mut self, map: &mut MapParam) -> Result {
        map.reset()
    }
}
