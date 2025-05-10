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
    PreviewStart {
        start: CornerDef,
    },
    SelectEnd {
        start: CornerDef,
    },
    PreviewEnd {
        start: CornerDef,
        end: CornerDef,
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
    action.select_corner(&mut map, CornerDef::Position(trigger.point))?;
    action.click(&mut map)?;
    Ok(())
}

fn select_wall(
    trigger: Trigger<SelectWall>,
    mut action: Single<(Entity, &mut WallAction)>,
    mut map: MapParam,
) -> Result {
    let (_, ref mut action) = *action;
    match trigger.kind {
        WallPickKind::Corner { corner, .. } => {
            action.select_corner(&mut map, CornerDef::Corner(corner))
        }
        WallPickKind::Wall {
            wall,
            position,
            start,
            start_position,
            end,
            end_position,
        } => action.select_wall(
            &mut map,
            wall,
            position,
            start,
            start_position,
            end,
            end_position,
        ),
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
        WallPickKind::Corner { corner, .. } => {
            action.select_corner(&mut map, CornerDef::Corner(corner))?;
            action.click(&mut map)
        }
        WallPickKind::Wall {
            wall,
            position,
            start,
            start_position,
            end,
            end_position,
        } => {
            action.select_wall(
                &mut map,
                wall,
                position,
                start,
                start_position,
                end,
                end_position,
            )
            // action.click(&mut commands, &engine_state)
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
                *self = WallAction::PreviewStart { start: corner }
            }
            WallAction::PreviewStart { ref mut start } => {
                map.insert_corner(corner)?;
                *start = corner;
            }
            WallAction::SelectEnd { start } => {
                map.insert_wall(start, corner)?;
                *self = WallAction::PreviewEnd { start, end: corner };
            }
            WallAction::PreviewEnd { start, ref mut end } => {
                map.insert_wall(start, corner)?;
                *end = corner;
            }
        }

        Ok(())
    }

    fn select_wall(
        &mut self,
        _param: &mut MapParam,
        _prev_wall: Entity,
        _pos: Vec2,
        _wall_start: Entity,
        _wall_start_pos: Vec2,
        _wall_end: Entity,
        _wall_end_pos: Vec2,
    ) -> Result {
        match *self {
            WallAction::SelectStart => {}
            WallAction::PreviewStart { .. } => {}
            WallAction::SelectEnd { .. } => {}
            WallAction::PreviewEnd { .. } => {}
        }

        Ok(())
    }

    fn cancel(&mut self, map: &mut MapParam) -> Result {
        map.reset()
    }

    fn click(&mut self, map: &mut MapParam) -> Result {
        match *self {
            WallAction::SelectStart => {}
            WallAction::PreviewStart { start } => *self = WallAction::SelectEnd { start },
            WallAction::SelectEnd { .. } => {}
            WallAction::PreviewEnd { end, .. } => {
                map.commit()?;
                *self = WallAction::SelectEnd { start: end }
            }
        }

        Ok(())
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

    fn insert_corner(&mut self, corner: CornerDef) -> Result {
        let mut map = self.map_q.get_mut(self.id())?;
        map.insert_corner(&mut self.map_queries, corner)?;
        Ok(())
    }

    fn insert_wall(&mut self, start: CornerDef, end: CornerDef) -> Result {
        let mut map = self.map_q.get_mut(self.id())?;
        map.insert_wall(&mut self.map_queries, start, end)?;
        Ok(())
    }

    fn commit(&mut self) -> Result {
        let [mut source, mut map] = self.map_q.get_many_mut([self.source(), self.id()])?;
        map.clone_into(&mut self.map_queries, &mut source);
        Ok(())
    }
}
