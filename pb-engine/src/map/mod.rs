pub mod corner;
pub mod door;
pub mod mesh;
pub mod room;
pub mod wall;

#[cfg(test)]
mod tests;

use std::{collections::HashSet, fmt};

use bevy::{
    ecs::{entity::EntityHashSet, system::SystemParam},
    platform::collections::HashMap,
    prelude::*,
};
use mesh::MapMesh;
use spade::{
    CdtEdge, ConstrainedDelaunayTriangulation, HasPosition, Point2, PositionInTriangulation,
    Triangulation,
    handles::{
        FaceHandle, FixedFaceHandle, FixedUndirectedEdgeHandle, FixedVertexHandle, OUTER_FACE,
        PossiblyOuterTag,
    },
};

use crate::{
    map::{corner::Corner, door::Door, room::Room, wall::Wall},
    save::MapModel,
};

pub const GRID_SIZE: f32 = 4.0;

#[derive(Component, TypePath)]
#[require(Transform, Visibility, MapMesh, Name::new(Map::type_path()))]
pub struct Map {
    id: Entity,
    children: EntityHashSet,
    size: u32,
    triangulation: ConstrainedDelaunayTriangulation<VertexData, (), UndirectedEdgeData, FaceData>,
}

#[derive(SystemParam)]
pub struct MapQueries<'w, 's> {
    pub commands: Commands<'w, 's>,
    pub corner_q: Query<'w, 's, &'static Corner>,
    pub wall_q: Query<'w, 's, &'static Wall>,
    pub room_q: Query<'w, 's, &'static Room>,
}

#[derive(Copy, Clone, Debug)]
pub enum CornerDef {
    Corner(Entity),
    Position(Vec2),
    Wall(Entity, Vec2),
}

/// Where an entity referenced by a map came from.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum MapEntity {
    /// This entity is a child of the original map this map was cloned from.
    Cloned(Entity),
    /// This entity exists in the original map, but has been replaced with a different entity in this map.
    Replaced(Entity, Entity),
    /// This entity was newly added in this map.
    Owned(Entity),
}

#[derive(Copy, Clone, Debug)]
struct VertexData {
    corner: Option<MapEntity>,
    position: Vec2,
    standalone: bool,
}

#[derive(Copy, Clone, Debug, Default)]
struct UndirectedEdgeData {
    wall: Option<MapEntity>,
}

#[derive(Copy, Clone, Debug, Default)]
struct FaceData {
    room: Option<MapEntity>,
}

pub fn map_inserted(
    trigger: Trigger<OnInsert, Map>,
    mut map_q: Query<&mut Map>,
    mut queries: MapQueries,
) -> Result {
    let mut map = map_q.get_mut(trigger.target())?;
    map.id = trigger.target();
    map.sync(&mut queries);
    Ok(())
}

impl Map {
    pub fn new() -> Self {
        Map::default()
    }

    pub fn from_model(model: &MapModel, entity_map: &mut impl EntityMapper) -> Result<Self> {
        let corner_indices: HashMap<Entity, usize> = model
            .corners
            .iter()
            .enumerate()
            .map(|(index, corner)| (corner.id, index))
            .collect();

        let mut triangulation = ConstrainedDelaunayTriangulation::<
            VertexData,
            (),
            UndirectedEdgeData,
            FaceData,
        >::bulk_load_cdt_stable(
            model
                .corners
                .iter()
                .map(|corner| {
                    VertexData::with_corner(
                        corner.position,
                        MapEntity::Owned(entity_map.get_mapped(corner.id)),
                    )
                })
                .collect(),
            model
                .walls
                .iter()
                .map(|wall| wall.corners.map(|id| corner_indices[&id]))
                .collect(),
        )?;

        if triangulation.vertices().len() != model.corners.len() {
            return Err("duplicate vertices".into());
        }

        for wall in &model.walls {
            let [from, to] = wall.corners.map(|id| {
                triangulation
                    .fixed_vertices()
                    .nth(corner_indices[&id])
                    .unwrap()
            });

            let edge = triangulation
                .get_edge_from_neighbors(from, to)
                .ok_or("edge not found")?;
            let face1 = edge.face().fix();
            let face2 = edge.rev().face().fix();
            triangulation
                .undirected_edge_data_mut(edge.as_undirected().fix())
                .data_mut()
                .wall = Some(MapEntity::Owned(entity_map.get_mapped(wall.id)));

            triangulation.face_data_mut(face1).room =
                Some(MapEntity::Owned(entity_map.get_mapped(wall.rooms[0])));
            triangulation.face_data_mut(face2).room =
                Some(MapEntity::Owned(entity_map.get_mapped(wall.rooms[1])));
        }

        let outer_room = entity_map.get_mapped(model.rooms[0].id);
        match &mut triangulation.face_data_mut(OUTER_FACE).room {
            Some(room) => assert_eq!(room.id(), outer_room),
            room @ None => *room = Some(MapEntity::Owned(outer_room)),
        }

        let mut map = Map {
            id: Entity::PLACEHOLDER,
            triangulation,
            children: EntityHashSet::default(),
            size: 0,
        };

        for corner in &model.corners {
            map.expand_size(corner.position)?;
        }

        Ok(map)
    }

    pub fn cloned(&self) -> Self {
        let mut cloned = Map::default();
        cloned.clone_from_inner(self);
        cloned
    }

    pub fn clone_from(&mut self, commands: &mut Commands, source: &Map) {
        self.clone_from_inner(source);

        for &child in &self.children {
            commands.entity(child).despawn();
        }

        self.children.clear();
    }

    fn clone_from_inner(&mut self, source: &Map) {
        self.triangulation.clone_from(&source.triangulation);
        for vertex in self.triangulation.fixed_vertices() {
            self.triangulation.vertex_data_mut(vertex).corner = source
                .triangulation
                .vertex(vertex)
                .data()
                .corner
                .map(MapEntity::cloned);
        }

        for edge in self.triangulation.fixed_undirected_edges() {
            self.triangulation
                .undirected_edge_data_mut(edge)
                .data_mut()
                .wall = source
                .triangulation
                .undirected_edge(edge)
                .data()
                .data()
                .wall
                .map(MapEntity::cloned);
        }

        for face in self.triangulation.fixed_inner_faces() {
            self.triangulation.face_data_mut(face).room = source
                .triangulation
                .face(face)
                .data()
                .room
                .map(MapEntity::cloned);
        }

        self.size = source.size;
    }

    pub fn clone_into(&mut self, queries: &mut MapQueries, source: &mut Map) {
        let mut new_children = EntityHashSet::default();

        self.triangulation.clone_into(&mut source.triangulation);
        for vertex in self.triangulation.fixed_vertices() {
            if let Some(corner) = self.triangulation.vertex(vertex).data().corner {
                self.triangulation.vertex_data_mut(vertex).corner = Some(corner.cloned());
                source.triangulation.vertex_data_mut(vertex).corner = Some(corner.to_owned());
                new_children.insert(corner.id());
            }
        }

        for edge in self.triangulation.fixed_undirected_edges() {
            if let Some(wall) = self.triangulation.undirected_edge(edge).data().data().wall {
                self.triangulation
                    .undirected_edge_data_mut(edge)
                    .data_mut()
                    .wall = Some(wall.cloned());
                source
                    .triangulation
                    .undirected_edge_data_mut(edge)
                    .data_mut()
                    .wall = Some(wall.to_owned());
                new_children.insert(wall.id());
            }
        }

        for face in self.triangulation.fixed_inner_faces() {
            let room = self.triangulation.face(face).data().room.unwrap();
            self.triangulation.face_data_mut(face).room = Some(room.cloned());
            source.triangulation.face_data_mut(face).room = Some(room.to_owned());
            new_children.insert(room.id());
        }

        for &removed_entity in source.children.difference(&new_children) {
            queries.commands.entity(removed_entity).despawn();
        }
        for &added_entity in new_children.difference(&source.children) {
            queries
                .commands
                .entity(added_entity)
                .insert(ChildOf(source.id()));
        }
        source.children = new_children;
        self.children.clear();

        source.size = self.size;
    }

    pub fn id(&self) -> Entity {
        self.id
    }

    pub fn corners(&self) -> impl Iterator<Item = MapEntity> + '_ {
        self.triangulation
            .vertices()
            .filter_map(|vertex| vertex.data().corner)
    }

    pub fn walls(&self) -> impl Iterator<Item = MapEntity> + '_ {
        self.triangulation
            .undirected_edges()
            .filter_map(|edge| edge.data().data().wall)
    }

    pub fn rooms(&self) -> impl Iterator<Item = MapEntity> + '_ {
        self.triangulation
            .all_faces()
            .filter_map(|face| face.data().room)
    }

    pub fn rooms_deduped(&self) -> impl Iterator<Item = MapEntity> + '_ {
        let mut unique = EntityHashSet::default();
        self.rooms().filter(move |&face| unique.insert(face.id()))
    }

    pub fn outer_room(&self) -> MapEntity {
        self.triangulation.face(OUTER_FACE).data().room.unwrap()
    }

    pub fn corner_walls(&self, corner: &Corner) -> impl Iterator<Item = (Entity, Entity)> + '_ {
        self.triangulation
            .vertex(corner.vertex())
            .out_edges()
            .filter(|edge| edge.is_constraint_edge())
            .map(|edge| {
                (
                    edge.as_undirected().data().data().wall(),
                    edge.to().data().corner(),
                )
            })
    }

    pub fn wall_rooms(&self, wall: &Wall) -> [Entity; 2] {
        let directed = self
            .triangulation
            .undirected_edge(wall.edge())
            .as_directed();
        [
            directed.face().data().room(),
            directed.rev().face().data().room(),
        ]
    }

    pub fn containing_room(&self, position: Vec2) -> Option<Entity> {
        match self
            .triangulation
            .locate(Point2::new(position.x, position.y))
        {
            PositionInTriangulation::OnVertex(vertex) => self
                .triangulation
                .vertex(vertex)
                .out_edge()
                .and_then(|edge| edge.face().data().room),
            PositionInTriangulation::OnEdge(edge) => {
                self.triangulation.directed_edge(edge).face().data().room
            }
            PositionInTriangulation::OnFace(face) => self.triangulation.face(face).data().room,
            PositionInTriangulation::OutsideOfConvexHull(_)
            | PositionInTriangulation::NoTriangulation => None,
        }
        .map(|entity| entity.id())
    }

    pub fn insert_corner(&mut self, queries: &mut MapQueries, corner: CornerDef) -> Result<Entity> {
        let vertex = self.get_or_insert_vertex(queries, corner)?;
        self.sync(queries);
        Ok(self.triangulation.vertex(vertex).data().corner())
    }

    pub fn remove_corner(&mut self, queries: &mut MapQueries, corner: Entity) -> Result {
        let vertex = queries.corner_q.get(corner)?.vertex();
        self.triangulation.remove(vertex);
        self.sync(queries);
        Ok(())
    }

    pub fn insert_wall(
        &mut self,
        queries: &mut MapQueries,
        start: CornerDef,
        end: CornerDef,
    ) -> Result<Option<(Entity, Entity)>> {
        let (start, end) = self.get_or_insert_vertices(queries, start, end)?;
        let edges = self
            .triangulation
            .add_constraint_and_split(start, end, VertexData::from);

        self.triangulation.vertex_data_mut(start).standalone = false;
        self.triangulation.vertex_data_mut(end).standalone = false;

        self.sync(queries);

        if edges.is_empty() {
            Ok(None)
        } else {
            Ok(Some((
                self.triangulation.vertex(start).data().corner(),
                self.triangulation.vertex(end).data().corner(),
            )))
        }
    }

    pub fn insert_wall_with(
        &mut self,
        queries: &mut MapQueries,
        start: CornerDef,
        end: CornerDef,
        bundle: impl Bundle + Clone,
    ) -> Result<Option<(Entity, Vec<Entity>, Entity)>> {
        let (start, end) = self.get_or_insert_vertices(queries, start, end)?;
        let edges = self
            .triangulation
            .add_constraint_and_split(start, end, VertexData::from);

        self.triangulation.vertex_data_mut(start).standalone = false;
        self.triangulation.vertex_data_mut(end).standalone = false;

        let walls: Vec<Entity> = edges
            .into_iter()
            .map(|edge| {
                let edge = edge.as_undirected();
                let wall = match self.triangulation.undirected_edge(edge).data().data().wall {
                    Some(wall) => queries.update(self.id, wall, bundle.clone()),
                    None => queries.spawn(self.id, bundle.clone()),
                };
                self.triangulation
                    .undirected_edge_data_mut(edge)
                    .data_mut()
                    .wall = Some(wall);
                wall.id()
            })
            .collect();

        self.sync(queries);

        if walls.is_empty() {
            Ok(None)
        } else {
            Ok(Some((
                self.triangulation.vertex(start).data().corner(),
                walls,
                self.triangulation.vertex(end).data().corner(),
            )))
        }
    }

    pub fn remove_wall(&mut self, queries: &mut MapQueries, wall: Entity) -> Result {
        let edge = queries.wall_q.get(wall)?.edge();
        self.triangulation.remove_constraint_edge(edge);
        self.sync(queries);
        Ok(())
    }

    fn get_or_insert_vertices(
        &mut self,
        queries: &mut MapQueries,
        target1: CornerDef,
        target2: CornerDef,
    ) -> Result<(FixedVertexHandle, FixedVertexHandle)> {
        match (target1, target2) {
            (CornerDef::Wall(wall1, position1), CornerDef::Wall(wall2, position2))
                if wall1 == wall2 =>
            {
                let wall = queries.wall_q.get(wall1)?;
                let edge = self.triangulation.undirected_edge(wall.edge());
                let [start, end] = edge.vertices().map(|v| v.fix());
                self.triangulation.remove_constraint_edge(edge.fix());

                let mid1 = self.triangulation.insert(VertexData::new(position1))?;
                let mid2 = self.triangulation.insert(VertexData::new(position2))?;

                self.triangulation
                    .add_constraint_and_split(start, mid1, VertexData::from);
                self.triangulation
                    .add_constraint_and_split(mid2, end, VertexData::from);

                Ok((mid1, mid2))
            }
            _ => Ok((
                self.get_or_insert_vertex(queries, target1)?,
                self.get_or_insert_vertex(queries, target2)?,
            )),
        }
    }

    fn get_or_insert_vertex(
        &mut self,
        queries: &mut MapQueries,
        target: CornerDef,
    ) -> Result<FixedVertexHandle> {
        match target {
            CornerDef::Corner(corner) => Ok(queries.corner_q.get(corner)?.vertex()),
            CornerDef::Position(position) => {
                self.expand_size(position)?;

                let vertex = self
                    .triangulation
                    .insert(VertexData::standalone(position))?;

                Ok(vertex)
            }
            CornerDef::Wall(wall, position) => {
                let wall = queries.wall_q.get(wall)?;
                let edge = self.triangulation.undirected_edge(wall.edge());
                let [start, end] = edge.vertices().map(|v| v.fix());
                self.triangulation.remove_constraint_edge(edge.fix());

                let mid = self.triangulation.insert(VertexData::new(position))?;

                self.triangulation
                    .add_constraint_and_split(start, mid, VertexData::from);
                self.triangulation
                    .add_constraint_and_split(mid, end, VertexData::from);

                Ok(mid)
            }
        }
    }

    fn expand_size(&mut self, point: Vec2) -> Result {
        let max_dim = point.x.abs().max(point.y.abs());

        while (self.size as f32 * GRID_SIZE) <= max_dim {
            let new_size = self.size + 1;
            let new_size_f = new_size as f32 * GRID_SIZE;

            for i in 0..new_size {
                let pos_f = i as f32 * GRID_SIZE;

                self.triangulation
                    .insert(VertexData::new(Vec2::new(new_size_f, new_size_f - pos_f)))?;
                self.triangulation
                    .insert(VertexData::new(Vec2::new(new_size_f, -pos_f)))?;
                self.triangulation
                    .insert(VertexData::new(Vec2::new(new_size_f - pos_f, -new_size_f)))?;
                self.triangulation
                    .insert(VertexData::new(Vec2::new(-pos_f, -new_size_f)))?;
                self.triangulation
                    .insert(VertexData::new(Vec2::new(-new_size_f, pos_f - new_size_f)))?;
                self.triangulation
                    .insert(VertexData::new(Vec2::new(-new_size_f, pos_f)))?;
                self.triangulation
                    .insert(VertexData::new(Vec2::new(pos_f - new_size_f, new_size_f)))?;
                self.triangulation
                    .insert(VertexData::new(Vec2::new(pos_f, new_size_f)))?;
            }

            self.size = new_size;
        }

        Ok(())
    }

    fn sync(&mut self, queries: &mut MapQueries) {
        let mut new_children = EntityHashSet::default();

        self.sync_vertices(queries, &mut new_children);
        self.sync_faces(queries, &mut new_children);
        self.sync_edges(queries, &mut new_children);

        for &removed_entity in self.children.difference(&new_children) {
            queries.commands.entity(removed_entity).despawn();
        }
        self.children = new_children;
    }

    fn sync_vertices(&mut self, queries: &mut MapQueries, new_children: &mut EntityHashSet) {
        for vertex in self.triangulation.fixed_vertices() {
            let vertex = self.triangulation.vertex(vertex);
            let vertex_data = vertex.data();
            let is_corner =
                vertex_data.standalone || vertex.out_edges().any(|e| e.is_constraint_edge());
            let vertex = vertex.fix();

            if is_corner {
                let corner =
                    self.update_corner(queries, vertex_data.corner, vertex, vertex_data.position);
                self.triangulation.vertex_data_mut(vertex).corner = Some(corner);
                if !corner.is_cloned() {
                    new_children.insert(corner.id());
                }
            } else if vertex_data.corner.is_some() {
                self.triangulation.vertex_data_mut(vertex).corner = None;
            }
        }
    }

    fn sync_faces(&mut self, queries: &mut MapQueries, new_children: &mut EntityHashSet) {
        let mut visited_faces = HashSet::new();
        let mut visited_rooms = EntityHashSet::default();

        for face in self.triangulation.fixed_all_faces() {
            if !visited_faces.insert(face) {
                continue;
            }

            let mut open = vec![face];
            let mut faces = vec![face];
            let mut room = None;

            while let Some(face) = open.pop() {
                self.for_each_adjacent_face(face, |adjacent_face| {
                    if !visited_faces.insert(adjacent_face.fix()) {
                        return;
                    }

                    open.push(adjacent_face.fix());
                    faces.push(adjacent_face.fix());

                    if room.is_none() {
                        if let Some(adjacent_face_room) = adjacent_face.data().room {
                            if visited_rooms.insert(adjacent_face_room.id()) {
                                room = Some(adjacent_face_room);
                            }
                        }
                    }
                });
            }

            faces.sort_unstable();

            let room = self.update_room(queries, room);
            for face in faces {
                self.triangulation.face_data_mut(face).room = Some(room);
            }
            if !room.is_cloned() {
                new_children.insert(room.id());
            }
        }
    }

    fn sync_edges(&mut self, queries: &mut MapQueries, new_children: &mut EntityHashSet) {
        for edge in self.triangulation.fixed_undirected_edges() {
            let edge = self.triangulation.undirected_edge(edge);
            let edge_data = *edge.data().data();
            let is_wall = edge.is_constraint_edge();

            if is_wall {
                let corners = edge
                    .as_directed()
                    .vertices()
                    .map(|vertex| vertex.data().corner());
                let positions = edge.vertices().map(|vertex| vertex.data().position);
                let edge = edge.fix();

                let wall = self.update_wall(queries, edge_data.wall, edge, corners, positions);
                self.triangulation
                    .undirected_edge_data_mut(edge)
                    .data_mut()
                    .wall = Some(wall);
                if !wall.is_cloned() {
                    new_children.insert(wall.id());
                }
            } else if edge_data.wall.is_some() {
                self.triangulation
                    .undirected_edge_data_mut(edge.fix())
                    .data_mut()
                    .wall = None;
            }
        }
    }

    fn for_each_adjacent_face(
        &self,
        face: FixedFaceHandle<PossiblyOuterTag>,
        mut f: impl FnMut(
            FaceHandle<PossiblyOuterTag, VertexData, (), CdtEdge<UndirectedEdgeData>, FaceData>,
        ),
    ) {
        if let Some(inner_face) = face.as_inner() {
            for adjacent_face in self
                .triangulation
                .face(inner_face)
                .adjacent_edges()
                .into_iter()
                .filter(|edge| !edge.is_constraint_edge())
                .map(|edge| edge.rev().face())
            {
                f(adjacent_face);
            }
        } else {
            for adjacent_face in self
                .triangulation
                .convex_hull()
                .filter(|edge| !edge.is_constraint_edge())
                .map(|edge| edge.rev().face())
            {
                f(adjacent_face);
            }
        }
    }

    fn update_corner(
        &self,
        queries: &mut MapQueries,
        corner: Option<MapEntity>,
        vertex: FixedVertexHandle,
        position: Vec2,
    ) -> MapEntity {
        if let Some(corner) = corner {
            match queries.corner(corner.id()) {
                Some(corner_data) if corner_data.vertex() == vertex => corner,
                _ => queries.update(self.id, corner, Corner::bundle(vertex, position)),
            }
        } else {
            queries.spawn(self.id, Corner::bundle(vertex, position))
        }
    }

    fn update_wall(
        &self,
        queries: &mut MapQueries,
        wall: Option<MapEntity>,
        edge: FixedUndirectedEdgeHandle,
        corners: [Entity; 2],
        positions: [Vec2; 2],
    ) -> MapEntity {
        if let Some(wall) = wall {
            match queries.wall(wall.id()) {
                Some(wall_data) if wall_data.edge() == edge && wall_data.corners() == corners => {
                    wall
                }
                _ => queries.update(self.id, wall, Wall::bundle(edge, corners, positions)),
            }
        } else {
            queries.spawn(self.id, Wall::bundle(edge, corners, positions))
        }
    }

    fn update_room(&self, queries: &mut MapQueries, room: Option<MapEntity>) -> MapEntity {
        if let Some(room) = room {
            match queries.room(room.id()) {
                Some(_) => room,
                _ => queries.update(self.id, room, Room::bundle()),
            }
        } else {
            queries.spawn(self.id, Room::bundle())
        }
    }
}

impl Default for Map {
    fn default() -> Self {
        Self {
            id: Entity::PLACEHOLDER,
            children: EntityHashSet::default(),
            triangulation: Default::default(),
            size: 0,
        }
    }
}

impl fmt::Debug for Map {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Layout")
            .field(
                "vertices",
                &self.triangulation.vertices().collect::<Vec<_>>(),
            )
            .field(
                "edges",
                &self.triangulation.undirected_edges().collect::<Vec<_>>(),
            )
            .field(
                "faces",
                &self.triangulation.inner_faces().collect::<Vec<_>>(),
            )
            .finish()
    }
}

impl MapEntity {
    pub fn id(self) -> Entity {
        match self {
            MapEntity::Cloned(entity) => entity,
            MapEntity::Replaced(_, entity) => entity,
            MapEntity::Owned(entity) => entity,
        }
    }

    pub fn source(self) -> Option<Entity> {
        match self {
            MapEntity::Cloned(entity) => Some(entity),
            MapEntity::Replaced(entity, _) => Some(entity),
            MapEntity::Owned(_) => None,
        }
    }

    pub fn cloned(self) -> Self {
        MapEntity::Cloned(self.id())
    }

    pub fn to_owned(self) -> Self {
        MapEntity::Owned(self.id())
    }

    pub fn is_cloned(self) -> bool {
        matches!(self, MapEntity::Cloned(_))
    }
}

impl MapQueries<'_, '_> {
    fn corner(&self, corner: Entity) -> Option<&Corner> {
        self.corner_q.get(corner).ok()
    }

    fn wall(&self, wall: Entity) -> Option<&Wall> {
        self.wall_q.get(wall).ok()
    }

    fn room(&self, room: Entity) -> Option<&Room> {
        self.room_q.get(room).ok()
    }

    fn spawn(&mut self, map: Entity, bundle: impl Bundle) -> MapEntity {
        MapEntity::Owned(self.commands.spawn((bundle, ChildOf(map))).id())
    }

    fn update(&mut self, map: Entity, entity: MapEntity, bundle: impl Bundle) -> MapEntity {
        match entity {
            MapEntity::Cloned(source) => {
                let id = self
                    .commands
                    .entity(source)
                    .clone_and_spawn_with(|options| {
                        options
                            .deny_all()
                            .allow::<Corner>()
                            .allow::<Wall>()
                            .allow::<Room>()
                            .allow::<Door>()
                            .linked_cloning(false);
                    })
                    .insert((bundle, ChildOf(map)))
                    .id();
                MapEntity::Replaced(source, id)
            }
            MapEntity::Replaced(_, id) | MapEntity::Owned(id) => {
                self.commands.entity(id).insert(bundle);
                entity
            }
        }
    }
}

impl VertexData {
    fn new(position: Vec2) -> Self {
        VertexData {
            corner: None,
            position,
            standalone: false,
        }
    }

    fn with_corner(position: Vec2, corner: MapEntity) -> Self {
        VertexData {
            corner: Some(corner),
            position,
            standalone: false,
        }
    }

    fn standalone(position: Vec2) -> Self {
        VertexData {
            corner: None,
            position,
            standalone: true,
        }
    }

    fn corner(&self) -> Entity {
        self.corner.expect("expected corner to be populated").id()
    }
}

impl HasPosition for VertexData {
    type Scalar = f32;

    fn position(&self) -> Point2<f32> {
        Point2::new(self.position.x, self.position.y)
    }
}

impl From<Point2<f32>> for VertexData {
    fn from(position: Point2<f32>) -> Self {
        VertexData {
            corner: None,
            position: Vec2::new(position.x, position.y),
            standalone: false,
        }
    }
}

impl UndirectedEdgeData {
    fn wall(&self) -> Entity {
        self.wall.expect("expected wall to be populated").id()
    }
}

impl FaceData {
    fn room(&self) -> Entity {
        self.room.expect("expected room to be populated").id()
    }
}
