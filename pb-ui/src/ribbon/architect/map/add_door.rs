use bevy::prelude::*;
use pb_engine::map::{
    CornerDef, Map,
    door::{self, Door},
};
use pb_render::wall::VisibleMaps;

use crate::{
    action::Action,
    input::{
        cancel::Cancellable,
        picking::{
            physics::{
                PhysicsPickingState,
                wall::{CancelWall, ClickWall, SelectWall},
            },
            point::grid::Grid,
        },
    },
    ribbon::architect::map::MapParam,
};

pub fn add_door(
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
    Name::new(AddDoorAction::type_path()),
    PhysicsPickingState::Wall,
    Transform,
    Visibility
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
    action.select_wall(&mut map, trigger.wall, trigger.position)
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
    action.click_wall(&mut map, trigger.wall, trigger.position)
}

impl AddDoorAction {
    fn select_wall(&mut self, map: &mut MapParam, wall_id: Entity, position: Vec2) -> Result {
        map.reset()?;

        let wall = map.map_queries.wall_q.get(wall_id)?;
        if wall.length() < door::MIN_WIDTH {
            return Ok(());
        }

        let [start, end] = map.map_queries.corner_q.get_many(wall.corners())?;
        let start_position = start.position();
        let end_position = end.position();

        let hit_dir = position - start_position;
        let wall_dir = (end_position - start_position) / wall.length();

        let door_position = hit_dir
            .dot(wall_dir)
            .max(door::HALF_WIDTH)
            .min(wall.length() - door::HALF_WIDTH);

        let start_corner = if door_position > door::MAX_WIDTH / 2. {
            let door_start = door_position - door::HALF_WIDTH;
            CornerDef::Wall(wall_id, start_position + door_start * wall_dir)
        } else {
            CornerDef::Corner(wall.start())
        };

        let end_corner = if door_position < (wall.length() - door::MAX_WIDTH / 2.) {
            let door_end = door_position + door::HALF_WIDTH;
            CornerDef::Wall(wall_id, start_position + door_end * wall_dir)
        } else {
            CornerDef::Corner(wall.end())
        };

        match map.insert_wall_with(start_corner, end_corner, Door)? {
            Some((_, walls, _)) if walls.len() == 1 => Ok(()),
            _ => {
                map.reset()?;
                Ok(())
            }
        }
    }

    fn click_wall(&mut self, map: &mut MapParam, wall_id: Entity, position: Vec2) -> Result {
        self.select_wall(map, wall_id, position)?;
        map.commit()
    }

    fn cancel(&mut self, map: &mut MapParam) -> Result {
        map.reset()
    }
}
