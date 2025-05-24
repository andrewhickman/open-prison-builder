pub mod add_door;
pub mod add_wall;
pub mod remove_wall;

use bevy::{ecs::system::SystemParam, prelude::*};
use pb_engine::map::{CornerDef, Map, MapQueries};
use pb_render::wall::VisibleMap;

#[derive(SystemParam)]
struct MapParam<'w, 's> {
    map_queries: MapQueries<'w, 's>,
    visible_map: Res<'w, VisibleMap>,
    map_q: Query<'w, 's, &'static mut Map>,
}

impl MapParam<'_, '_> {
    fn id(&self) -> Result<Entity> {
        Ok(self.visible_map.id().ok_or("map should be visible")?)
    }

    fn source(&self) -> Result<Entity> {
        Ok(self
            .visible_map
            .source()
            .ok_or("visible map should have source")?)
    }

    fn reset(&mut self) -> Result {
        let [source, mut map] = self.map_q.get_many_mut([self.source()?, self.id()?])?;
        map.clone_from(&mut self.map_queries.commands, &source);
        Ok(())
    }

    fn insert_corner(&mut self, corner: CornerDef) -> Result<Entity> {
        self.map_q
            .get_mut(self.id()?)?
            .insert_corner(&mut self.map_queries, corner)
    }

    fn insert_wall(
        &mut self,
        start: CornerDef,
        end: CornerDef,
    ) -> Result<Option<(Entity, Entity)>> {
        self.map_q
            .get_mut(self.id()?)?
            .insert_wall(&mut self.map_queries, start, end)
    }

    fn remove_wall(&mut self, wall: Entity) -> Result {
        self.map_q
            .get_mut(self.id()?)?
            .remove_wall(&mut self.map_queries, wall)
    }

    fn commit(&mut self) -> Result {
        let [mut source, mut map] = self.map_q.get_many_mut([self.source()?, self.id()?])?;
        map.clone_into(&mut self.map_queries, &mut source);
        Ok(())
    }
}
