#![allow(clippy::type_complexity, clippy::too_many_arguments)]

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
use spade::{
    ConstrainedDelaunayTriangulation, HasPosition, HierarchyHintGenerator, Point2, Triangulation,
    handles::{
        FixedFaceHandle, FixedUndirectedEdgeHandle, FixedVertexHandle, InnerTag, OUTER_FACE,
    },
};

use crate::save::MapModel;

pub const GRID_SIZE: f32 = 16.0;

#[derive(Component)]
#[require(Transform, Visibility)]
pub struct Map {
    id: Entity,
    source: Option<Entity>,
    children: EntityHashSet,
    size: u32,
    triangulation: ConstrainedDelaunayTriangulation<
        VertexData,
        (),
        UndirectedEdgeData,
        FaceData,
        HierarchyHintGenerator<f32>,
    >,
}

#[derive(Clone, Debug, Component)]
#[require(Transform, Visibility)]
pub struct Corner {
    vertex: FixedVertexHandle,
    position: Vec2,
}

#[derive(Clone, Debug, Component)]
#[require(Transform, Visibility)]
pub struct Wall {
    edge: FixedUndirectedEdgeHandle,
    length: f32,
    position: Vec2,
    rotation: Rot2,
    corners: [Entity; 2],
    rooms: [Entity; 2],
}

#[derive(Clone, Debug, Component)]
#[require(Transform, Visibility)]
pub struct Room {
    faces: Vec<FixedFaceHandle<InnerTag>>,
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
            HierarchyHintGenerator<f32>,
        >::bulk_load_cdt_stable(
            model
                .corners
                .iter()
                .map(|corner| {
                    VertexData::corner(
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
            source: None,
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
        debug_assert!(self.source().is_none());

        let mut cloned = Map::default();
        cloned.clone_from_inner(self);
        cloned
    }

    pub fn clone_from(&mut self, commands: &mut Commands, source: &Map) {
        debug_assert!(source.source().is_none());

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

        self.source = Some(source.id());
        self.size = source.size;
    }

    pub fn clone_into(&mut self, queries: &mut MapQueries, source: &mut Map) {
        debug_assert_eq!(self.source(), Some(source.id()));

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

    pub fn source(&self) -> Option<Entity> {
        self.source
    }

    pub fn is_cloned(&self) -> bool {
        self.source.is_some()
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
            .inner_faces()
            .filter_map(|face| face.data().room)
    }

    pub fn rooms_deduped(&self) -> impl Iterator<Item = MapEntity> + '_ {
        let mut unique = EntityHashSet::default();
        self.rooms().filter(move |&face| unique.insert(face.id()))
    }

    pub fn corner_walls(&self, corner: &Corner) -> impl Iterator<Item = (Entity, Entity)> + '_ {
        self.triangulation
            .vertex(corner.vertex)
            .out_edges()
            .filter(|edge| edge.is_constraint_edge())
            .map(|edge| {
                (
                    edge.as_undirected().data().data().wall.unwrap().id(),
                    edge.to().data().corner.unwrap().id(),
                )
            })
    }

    pub fn insert_corner(&mut self, queries: &mut MapQueries, corner: CornerDef) -> Result<Entity> {
        let vertex = self.get_or_insert_vertex(queries, corner)?;
        self.sync(queries);
        Ok(self
            .triangulation
            .vertex(vertex)
            .data()
            .corner
            .unwrap()
            .id())
    }

    pub fn remove_corner(&mut self, queries: &mut MapQueries, corner: Entity) -> Result {
        let vertex = queries.corner_q.get(corner)?.vertex;
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
        let start = self.get_or_insert_vertex(queries, start)?;
        let end = self.get_or_insert_vertex(queries, end)?;
        self.triangulation
            .add_constraint_and_split(start, end, VertexData::from);

        self.triangulation.vertex_data_mut(start).standalone = false;
        self.triangulation.vertex_data_mut(end).standalone = false;

        self.sync(queries);

        if start == end {
            Ok(None)
        } else {
            Ok(Some((
                self.triangulation.vertex(start).data().corner.unwrap().id(),
                self.triangulation.vertex(end).data().corner.unwrap().id(),
            )))
        }
    }

    pub fn remove_wall(&mut self, queries: &mut MapQueries, wall: Entity) -> Result {
        let edge = queries.wall_q.get(wall)?.edge;
        self.triangulation.remove_constraint_edge(edge);
        self.sync(queries);
        Ok(())
    }

    fn get_or_insert_vertex(
        &mut self,
        queries: &mut MapQueries,
        target: CornerDef,
    ) -> Result<FixedVertexHandle> {
        match target {
            CornerDef::Corner(corner) => Ok(queries.corner_q.get(corner)?.vertex),
            CornerDef::Position(position) => {
                self.expand_size(position)?;

                let vertex = self
                    .triangulation
                    .insert(VertexData::standalone(position))?;

                Ok(vertex)
            }
            CornerDef::Wall(wall, position) => {
                let wall = queries.wall_q.get(wall)?;
                let edge = self.triangulation.undirected_edge(wall.edge);
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

        for face in self.triangulation.fixed_inner_faces() {
            if !visited_faces.insert(face) {
                continue;
            }

            let mut open = vec![face];
            let mut faces = vec![face];
            let mut room = None;

            while let Some(face) = open.pop() {
                for adjacent_face in self
                    .triangulation
                    .face(face)
                    .adjacent_edges()
                    .into_iter()
                    .filter(|edge| !edge.is_constraint_edge())
                    .flat_map(|edge| edge.rev().face().as_inner())
                {
                    if !visited_faces.insert(adjacent_face.fix()) {
                        continue;
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
                }
            }

            faces.sort_unstable();

            let room = self.update_room(queries, room, &faces);
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
                let directed1 = edge.as_directed();
                let directed2 = directed1.rev();
                let corners = directed1
                    .vertices()
                    .map(|vertex| vertex.data().corner.unwrap().id());
                let positions = edge.vertices().map(|vertex| vertex.data().position);
                let room1 = directed1.face().data().room.unwrap().id();
                let room2 = directed2.face().data().room.unwrap().id();
                let edge = edge.fix();

                let wall = self.update_wall(
                    queries,
                    edge_data.wall,
                    edge,
                    corners,
                    positions,
                    [room1, room2],
                );
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

    fn update_corner(
        &self,
        queries: &mut MapQueries,
        corner: Option<MapEntity>,
        vertex: FixedVertexHandle,
        position: Vec2,
    ) -> MapEntity {
        if let Some(corner) = corner {
            match queries.corner(corner.id()) {
                Some(corner_data) if corner_data.vertex == vertex => corner,
                _ => {
                    if let MapEntity::Cloned(source) = corner {
                        let corner = queries.spawn_corner(self.id, vertex, position);
                        MapEntity::Replaced(source, corner)
                    } else {
                        queries.update_corner(corner.id(), vertex, position);
                        corner
                    }
                }
            }
        } else {
            let corner = queries.spawn_corner(self.id, vertex, position);
            MapEntity::Owned(corner)
        }
    }

    fn update_wall(
        &self,
        queries: &mut MapQueries,
        wall: Option<MapEntity>,
        edge: FixedUndirectedEdgeHandle,
        corners: [Entity; 2],
        positions: [Vec2; 2],
        rooms: [Entity; 2],
    ) -> MapEntity {
        if let Some(wall) = wall {
            match queries.wall(wall.id()) {
                Some(wall_data)
                    if wall_data.edge == edge
                        && wall_data.corners == corners
                        && wall_data.rooms == rooms =>
                {
                    wall
                }
                _ => {
                    if let MapEntity::Cloned(source) = wall {
                        let wall = queries.spawn_wall(self.id, edge, corners, positions, rooms);
                        MapEntity::Replaced(source, wall)
                    } else {
                        queries.update_wall(wall.id(), edge, corners, positions, rooms);
                        wall
                    }
                }
            }
        } else {
            let wall = queries.spawn_wall(self.id, edge, corners, positions, rooms);
            MapEntity::Owned(wall)
        }
    }

    fn update_room(
        &self,
        queries: &mut MapQueries,
        room: Option<MapEntity>,
        faces: &[FixedFaceHandle<InnerTag>],
    ) -> MapEntity {
        if let Some(room) = room {
            match queries.room(room.id()) {
                Some(room_data) if room_data.faces == faces => room,
                _ => {
                    if let MapEntity::Cloned(source) = room {
                        let room: Entity = queries.spawn_room(self.id, faces.to_vec());
                        MapEntity::Replaced(source, room)
                    } else {
                        queries.update_room(room.id(), faces.to_vec());
                        room
                    }
                }
            }
        } else {
            let room = queries.spawn_room(self.id, faces.to_vec());
            MapEntity::Owned(room)
        }
    }
}

impl Default for Map {
    fn default() -> Self {
        Self {
            id: Entity::PLACEHOLDER,
            source: None,
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

impl Corner {
    pub fn position(&self) -> Vec2 {
        self.position
    }

    fn bundle(vertex: FixedVertexHandle, position: Vec2) -> impl Bundle {
        (
            Corner { vertex, position },
            Transform::from_translation(position.extend(0.)),
        )
    }
}

impl Wall {
    pub fn length(&self) -> f32 {
        self.length
    }

    pub fn corners(&self) -> [Entity; 2] {
        self.corners
    }

    pub fn rooms(&self) -> [Entity; 2] {
        self.rooms
    }

    pub fn position(&self) -> Vec2 {
        self.position
    }

    pub fn rotation(&self) -> Rot2 {
        self.rotation
    }

    pub fn isometry(&self) -> Isometry2d {
        Isometry2d {
            translation: self.position,
            rotation: self.rotation,
        }
    }

    fn bundle(
        edge: FixedUndirectedEdgeHandle,
        corners: [Entity; 2],
        [position1, position2]: [Vec2; 2],
        rooms: [Entity; 2],
    ) -> impl Bundle {
        let length = position1.distance(position2);
        let position = position1.midpoint(position2);
        let rotation = (position2 - position1).to_angle();

        (
            Wall {
                edge,
                length,
                position,
                rotation: Rot2::radians(rotation),
                corners,
                rooms,
            },
            Transform {
                scale: Vec3::ONE,
                translation: position.extend(0.),
                rotation: Quat::from_rotation_z(rotation),
            },
        )
    }
}

impl Room {
    fn bundle(faces: Vec<FixedFaceHandle<InnerTag>>) -> impl Bundle {
        Room { faces }
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

    fn spawn_corner(&mut self, map: Entity, vertex: FixedVertexHandle, position: Vec2) -> Entity {
        self.commands
            .spawn((Corner::bundle(vertex, position), ChildOf(map)))
            .id()
    }

    fn update_corner(&mut self, corner: Entity, vertex: FixedVertexHandle, position: Vec2) {
        self.commands
            .entity(corner)
            .insert(Corner::bundle(vertex, position));
    }

    fn spawn_wall(
        &mut self,
        map: Entity,
        edge: FixedUndirectedEdgeHandle,
        corners: [Entity; 2],
        positions: [Vec2; 2],
        rooms: [Entity; 2],
    ) -> Entity {
        self.commands
            .spawn((Wall::bundle(edge, corners, positions, rooms), ChildOf(map)))
            .id()
    }

    fn update_wall(
        &mut self,
        wall: Entity,
        edge: FixedUndirectedEdgeHandle,
        corners: [Entity; 2],
        positions: [Vec2; 2],
        rooms: [Entity; 2],
    ) {
        self.commands
            .entity(wall)
            .insert(Wall::bundle(edge, corners, positions, rooms));
    }

    fn spawn_room(&mut self, map: Entity, faces: Vec<FixedFaceHandle<InnerTag>>) -> Entity {
        self.commands
            .spawn((Room::bundle(faces), ChildOf(map)))
            .id()
    }

    fn update_room(&mut self, room: Entity, faces: Vec<FixedFaceHandle<InnerTag>>) {
        self.commands.entity(room).insert(Room::bundle(faces));
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

    fn corner(position: Vec2, corner: MapEntity) -> Self {
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
