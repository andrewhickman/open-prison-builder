use bevy::{ecs::system::SystemParam, prelude::*};
use pb_engine::map::{CornerDef, Map, MapQueries};
use pb_render::map::VisibleMap;

use crate::{
    action::Action,
    input::{
        cancel::Cancellable,
        picking::{
            physics::{
                PhysicsPickingState,
                wall::{CancelWall, ClickWall, SelectWall, WallPickKind},
            },
            point::{CancelPoint, ClickPoint, SelectPoint, grid::Grid},
        },
    },
};

pub fn wall(
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
            WallAction::default(),
            children![
                Grid::default(),
                Observer::new(select_point),
                Observer::new(cancel_point),
                Observer::new(click_point),
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
#[require(Action, Cancellable, PhysicsPickingState = PhysicsPickingState::Wall, Transform, Visibility, Name = Name::new(WallAction::type_path()))]
pub enum WallAction {
    #[default]
    SelectStart,
    SelectEnd {
        start: CornerDef,
    },
}

#[derive(SystemParam)]
struct MapParam<'w, 's> {
    map_queries: MapQueries<'w, 's>,
    visible_map: Res<'w, VisibleMap>,
    map_q: Query<'w, 's, &'static mut Map>,
}

fn select_point(
    trigger: Trigger<SelectPoint>,
    mut action: Single<&mut WallAction>,
    mut map: MapParam,
) -> Result {
    action.select_corner(&mut map, CornerDef::Position(trigger.point))
}

fn cancel_point(
    _: Trigger<CancelPoint>,
    mut action: Single<&mut WallAction>,
    mut map: MapParam,
) -> Result {
    action.cancel(&mut map)
}

fn click_point(
    trigger: Trigger<ClickPoint>,
    mut action: Single<&mut WallAction>,
    mut map: MapParam,
) -> Result {
    action.click(&mut map, CornerDef::Position(trigger.point))
}

fn select_wall(
    trigger: Trigger<SelectWall>,
    mut action: Single<&mut WallAction>,
    mut map: MapParam,
) -> Result {
    match trigger.kind {
        WallPickKind::Corner { corner, .. } => {
            action.select_corner(&mut map, CornerDef::Corner(corner))
        }
        WallPickKind::Wall { wall, position, .. } => {
            action.select_corner(&mut map, CornerDef::Wall(wall, position))
        }
    }
}

fn cancel_wall(
    _: Trigger<CancelWall>,
    mut action: Single<&mut WallAction>,
    mut map: MapParam,
) -> Result {
    action.cancel(&mut map)
}

fn click_wall(
    trigger: Trigger<ClickWall>,
    mut action: Single<&mut WallAction>,
    mut map: MapParam,
) -> Result {
    match trigger.kind {
        WallPickKind::Corner { corner, .. } => action.click(&mut map, CornerDef::Corner(corner)),
        WallPickKind::Wall { wall, position, .. } => {
            action.click(&mut map, CornerDef::Wall(wall, position))
        }
    }
}

pub fn cancel(_: Trigger<OnRemove, WallAction>) {}

impl WallAction {
    fn select_corner(&mut self, map: &mut MapParam, corner: CornerDef) -> Result {
        map.reset()?;

        match *self {
            WallAction::SelectStart => {
                map.insert_corner(corner)?;
            }
            WallAction::SelectEnd { start } => {
                map.insert_wall(start, corner)?;
            }
        }

        Ok(())
    }

    fn click(&mut self, map: &mut MapParam, corner: CornerDef) -> Result {
        map.reset()?;

        match *self {
            WallAction::SelectStart => {
                map.insert_corner(corner)?;
                *self = WallAction::SelectEnd { start: corner }
            }
            WallAction::SelectEnd { start } => {
                if let Some((_, end)) = map.insert_wall(start, corner)? {
                    map.commit()?;
                    *self = WallAction::SelectEnd {
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

impl MapParam<'_, '_> {
    fn id(&self) -> Entity {
        self.visible_map.id().expect("map should be visible")
    }

    fn source(&self) -> Entity {
        self.visible_map.source().expect("map should be visible")
    }

    fn reset(&mut self) -> Result {
        let [source, mut map] = self.map_q.get_many_mut([self.source(), self.id()])?;
        map.clone_from(&mut self.map_queries.commands, &source);
        Ok(())
    }

    fn insert_corner(&mut self, corner: CornerDef) -> Result<Entity> {
        self.map_q
            .get_mut(self.id())?
            .insert_corner(&mut self.map_queries, corner)
    }

    fn insert_wall(
        &mut self,
        start: CornerDef,
        end: CornerDef,
    ) -> Result<Option<(Entity, Entity)>> {
        self.map_q
            .get_mut(self.id())?
            .insert_wall(&mut self.map_queries, start, end)
    }

    fn commit(&mut self) -> Result {
        let [mut source, mut map] = self.map_q.get_many_mut([self.source(), self.id()])?;
        map.clone_into(&mut self.map_queries, &mut source);
        Ok(())
    }
}
