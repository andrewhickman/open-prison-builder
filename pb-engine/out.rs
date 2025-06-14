#![feature(prelude_import)]
#![allow(clippy::type_complexity, clippy::too_many_arguments)]
#[prelude_import]
use std::prelude::rust_2024::*;
#[macro_use]
extern crate std;
pub mod dev {
    use avian2d::prelude::*;
    use bevy::{
        color::palettes::tailwind::{GREEN_300, INDIGO_800},
        prelude::*,
    };
    use crate::{map::mesh::MapMesh, pawn::ai::{Task, path::PathTask}};
    pub struct DevSettings {
        pub draw_paths: bool,
        pub draw_meshes: bool,
    }
    #[automatically_derived]
    impl ::core::default::Default for DevSettings {
        #[inline]
        fn default() -> DevSettings {
            DevSettings {
                draw_paths: ::core::default::Default::default(),
                draw_meshes: ::core::default::Default::default(),
            }
        }
    }
    impl bevy::ecs::resource::Resource for DevSettings
    where
        Self: Send + Sync + 'static,
    {}
    pub fn draw_paths_condition(settings: Res<DevSettings>) -> bool {
        settings.draw_paths
    }
    pub fn draw_paths(
        task_q: Query<(&Task, &PathTask)>,
        pos_q: Query<&Position>,
        mut gizmos: Gizmos,
    ) {
        for (task, path) in &task_q {
            if let Some(steps) = path.steps() {
                if let Ok(start) = pos_q.get(task.actor()) {
                    if !steps.is_empty() {
                        gizmos.line_2d(start.0, steps[0], INDIGO_800);
                        for i in 0..(steps.len() - 1) {
                            gizmos.line_2d(steps[i], steps[i + 1], INDIGO_800);
                        }
                    }
                }
            }
        }
    }
    pub fn draw_meshes_condition(settings: Res<DevSettings>) -> bool {
        settings.draw_meshes
    }
    pub fn draw_meshes(map_q: Query<&MapMesh>, mut gizmos: Gizmos) {
        for map in &map_q {
            for mesh in map.meshes() {
                for layer in &mesh.layers {
                    for polygon in &layer.polygons {
                        gizmos
                            .linestrip(
                                polygon
                                    .vertices
                                    .iter()
                                    .cycle()
                                    .take(polygon.vertices.len() + 1)
                                    .map(|&index| {
                                        layer.vertices[index as usize].coords.extend(0.)
                                    }),
                                GREEN_300,
                            );
                    }
                }
            }
        }
    }
}
pub mod layer {
    use avian2d::prelude::PhysicsLayer;
    pub enum Layer {
        #[default]
        Default,
        Perimeter,
        Wall,
        Pawn,
    }
    impl PhysicsLayer for Layer {
        fn all_bits() -> u32 {
            15u32
        }
        fn to_bits(&self) -> u32 {
            match self {
                Layer::Default => 1u32,
                Layer::Perimeter => 2u32,
                Layer::Wall => 4u32,
                Layer::Pawn => 8u32,
            }
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Layer {
        #[inline]
        fn clone(&self) -> Layer {
            *self
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for Layer {}
    #[automatically_derived]
    impl ::core::fmt::Debug for Layer {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(
                f,
                match self {
                    Layer::Default => "Default",
                    Layer::Perimeter => "Perimeter",
                    Layer::Wall => "Wall",
                    Layer::Pawn => "Pawn",
                },
            )
        }
    }
    #[automatically_derived]
    impl ::core::default::Default for Layer {
        #[inline]
        fn default() -> Layer {
            Self::Default
        }
    }
}
pub mod map {
    pub mod corner {
        use avian2d::prelude::*;
        use bevy::prelude::*;
        use pb_util::event::ComponentEvent;
        use spade::handles::FixedVertexHandle;
        use crate::{layer::Layer, map::wall::Wall, root::ChildOfRoot};
        #[require(Transform, Visibility)]
        #[component(immutable)]
        pub struct Corner {
            vertex: FixedVertexHandle,
            position: Vec2,
        }
        #[automatically_derived]
        impl ::core::clone::Clone for Corner {
            #[inline]
            fn clone(&self) -> Corner {
                Corner {
                    vertex: ::core::clone::Clone::clone(&self.vertex),
                    position: ::core::clone::Clone::clone(&self.position),
                }
            }
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Corner {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "Corner",
                    "vertex",
                    &self.vertex,
                    "position",
                    &&self.position,
                )
            }
        }
        #[doc = "**Required Components**: [`Transform`], [`Visibility`]. \n\n A component's Required Components are inserted whenever it is inserted. Note that this will also insert the required components _of_ the required components, recursively, in depth-first order."]
        impl bevy::ecs::component::Component for Corner
        where
            Self: Send + Sync + 'static,
        {
            const STORAGE_TYPE: bevy::ecs::component::StorageType = bevy::ecs::component::StorageType::Table;
            type Mutability = bevy::ecs::component::Immutable;
            fn register_required_components(
                requiree: bevy::ecs::component::ComponentId,
                components: &mut bevy::ecs::component::ComponentsRegistrator,
                required_components: &mut bevy::ecs::component::RequiredComponents,
                inheritance_depth: u16,
                recursion_check_stack: &mut bevy::ecs::__macro_exports::Vec<
                    bevy::ecs::component::ComponentId,
                >,
            ) {
                bevy::ecs::component::enforce_no_required_components_recursion(
                    components,
                    recursion_check_stack,
                );
                let self_id = components.register_component::<Self>();
                recursion_check_stack.push(self_id);
                components
                    .register_required_components_manual::<
                        Self,
                        Transform,
                    >(
                        required_components,
                        <Transform as Default>::default,
                        inheritance_depth,
                        recursion_check_stack,
                    );
                components
                    .register_required_components_manual::<
                        Self,
                        Visibility,
                    >(
                        required_components,
                        <Visibility as Default>::default,
                        inheritance_depth,
                        recursion_check_stack,
                    );
                <Transform as bevy::ecs::component::Component>::register_required_components(
                    requiree,
                    components,
                    required_components,
                    inheritance_depth + 1,
                    recursion_check_stack,
                );
                <Visibility as bevy::ecs::component::Component>::register_required_components(
                    requiree,
                    components,
                    required_components,
                    inheritance_depth + 1,
                    recursion_check_stack,
                );
                recursion_check_stack.pop();
            }
            fn clone_behavior() -> bevy::ecs::component::ComponentCloneBehavior {
                use bevy::ecs::component::{
                    DefaultCloneBehaviorBase, DefaultCloneBehaviorViaClone,
                };
                (&&&bevy::ecs::component::DefaultCloneBehaviorSpecialization::<
                    Self,
                >::default())
                    .default_clone_behavior()
            }
        }
        pub fn add_colliders(
            mut commands: Commands,
            mut corner_e: EventReader<ComponentEvent<OnInsert, Corner>>,
            root_q: Query<&ChildOfRoot>,
        ) -> Result {
            for event in corner_e.read() {
                if root_q.contains(event.target) {
                    commands
                        .entity(event.target)
                        .insert((
                            RigidBody::Static,
                            Collider::circle(Wall::RADIUS),
                            CollisionLayers::new(Layer::Wall, LayerMask::ALL),
                        ));
                }
            }
            Ok(())
        }
        impl Corner {
            pub fn position(&self) -> Vec2 {
                self.position
            }
            pub(crate) fn vertex(&self) -> FixedVertexHandle {
                self.vertex
            }
            pub(crate) fn bundle(
                vertex: FixedVertexHandle,
                position: Vec2,
            ) -> impl Bundle {
                (
                    Name::new(
                        ::alloc::__export::must_use({
                            let res = ::alloc::fmt::format(
                                format_args!("corner ({0}, {1})", position.x, position.y),
                            );
                            res
                        }),
                    ),
                    Corner { vertex, position },
                    Transform::from_translation(position.extend(0.)),
                )
            }
        }
    }
    pub mod door {
        use bevy::{
            ecs::{
                component::HookContext, entity::EntityHashMap, query::QueryEntityError,
                world::DeferredWorld,
            },
            prelude::*,
        };
        use pb_util::event::ComponentEvent;
        use crate::{
            map::{Map, wall::Wall},
            root::ChildOfRoot,
        };
        pub const INNER_WIDTH: f32 = WIDTH - (2. * Wall::RADIUS);
        pub const MIN_WIDTH: f32 = 0.9;
        pub const WIDTH: f32 = 1.0;
        pub const DEPTH: f32 = 0.2;
        pub const MAX_WIDTH: f32 = 1.1;
        pub const HALF_INNER_WIDTH: f32 = INNER_WIDTH / 2.;
        pub const HALF_WIDTH: f32 = WIDTH / 2.;
        pub const HALF_DEPTH: f32 = DEPTH / 2.;
        #[component(immutable)]
        pub struct Door;
        #[automatically_derived]
        impl ::core::clone::Clone for Door {
            #[inline]
            fn clone(&self) -> Door {
                Door
            }
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Door {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::write_str(f, "Door")
            }
        }
        impl bevy::ecs::component::Component for Door
        where
            Self: Send + Sync + 'static,
        {
            const STORAGE_TYPE: bevy::ecs::component::StorageType = bevy::ecs::component::StorageType::Table;
            type Mutability = bevy::ecs::component::Immutable;
            fn register_required_components(
                requiree: bevy::ecs::component::ComponentId,
                components: &mut bevy::ecs::component::ComponentsRegistrator,
                required_components: &mut bevy::ecs::component::RequiredComponents,
                inheritance_depth: u16,
                recursion_check_stack: &mut bevy::ecs::__macro_exports::Vec<
                    bevy::ecs::component::ComponentId,
                >,
            ) {
                bevy::ecs::component::enforce_no_required_components_recursion(
                    components,
                    recursion_check_stack,
                );
                let self_id = components.register_component::<Self>();
                recursion_check_stack.push(self_id);
                recursion_check_stack.pop();
            }
            fn clone_behavior() -> bevy::ecs::component::ComponentCloneBehavior {
                use bevy::ecs::component::{
                    DefaultCloneBehaviorBase, DefaultCloneBehaviorViaClone,
                };
                (&&&bevy::ecs::component::DefaultCloneBehaviorSpecialization::<
                    Self,
                >::default())
                    .default_clone_behavior()
            }
        }
        #[component(
            immutable,
            on_insert = DoorLinks::on_insert,
            on_remove = DoorLinks::on_remove
        )]
        pub struct DoorLinks {
            pub left: Entity,
            pub right: Entity,
        }
        #[automatically_derived]
        impl ::core::clone::Clone for DoorLinks {
            #[inline]
            fn clone(&self) -> DoorLinks {
                DoorLinks {
                    left: ::core::clone::Clone::clone(&self.left),
                    right: ::core::clone::Clone::clone(&self.right),
                }
            }
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for DoorLinks {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "DoorLinks",
                    "left",
                    &self.left,
                    "right",
                    &&self.right,
                )
            }
        }
        impl bevy::ecs::component::Component for DoorLinks
        where
            Self: Send + Sync + 'static,
        {
            const STORAGE_TYPE: bevy::ecs::component::StorageType = bevy::ecs::component::StorageType::Table;
            type Mutability = bevy::ecs::component::Immutable;
            fn register_required_components(
                requiree: bevy::ecs::component::ComponentId,
                components: &mut bevy::ecs::component::ComponentsRegistrator,
                required_components: &mut bevy::ecs::component::RequiredComponents,
                inheritance_depth: u16,
                recursion_check_stack: &mut bevy::ecs::__macro_exports::Vec<
                    bevy::ecs::component::ComponentId,
                >,
            ) {
                bevy::ecs::component::enforce_no_required_components_recursion(
                    components,
                    recursion_check_stack,
                );
                let self_id = components.register_component::<Self>();
                recursion_check_stack.push(self_id);
                recursion_check_stack.pop();
            }
            fn on_insert() -> ::core::option::Option<
                bevy::ecs::component::ComponentHook,
            > {
                ::core::option::Option::Some(DoorLinks::on_insert)
            }
            fn on_remove() -> ::core::option::Option<
                bevy::ecs::component::ComponentHook,
            > {
                ::core::option::Option::Some(DoorLinks::on_remove)
            }
            fn clone_behavior() -> bevy::ecs::component::ComponentCloneBehavior {
                use bevy::ecs::component::{
                    DefaultCloneBehaviorBase, DefaultCloneBehaviorViaClone,
                };
                (&&&bevy::ecs::component::DefaultCloneBehaviorSpecialization::<
                    Self,
                >::default())
                    .default_clone_behavior()
            }
        }
        pub struct RoomLinks {
            doors: EntityHashMap<RoomLink>,
        }
        #[automatically_derived]
        impl ::core::default::Default for RoomLinks {
            #[inline]
            fn default() -> RoomLinks {
                RoomLinks {
                    doors: ::core::default::Default::default(),
                }
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for RoomLinks {
            #[inline]
            fn clone(&self) -> RoomLinks {
                RoomLinks {
                    doors: ::core::clone::Clone::clone(&self.doors),
                }
            }
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for RoomLinks {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field1_finish(
                    f,
                    "RoomLinks",
                    "doors",
                    &&self.doors,
                )
            }
        }
        impl bevy::ecs::component::Component for RoomLinks
        where
            Self: Send + Sync + 'static,
        {
            const STORAGE_TYPE: bevy::ecs::component::StorageType = bevy::ecs::component::StorageType::Table;
            type Mutability = bevy::ecs::component::Mutable;
            fn register_required_components(
                requiree: bevy::ecs::component::ComponentId,
                components: &mut bevy::ecs::component::ComponentsRegistrator,
                required_components: &mut bevy::ecs::component::RequiredComponents,
                inheritance_depth: u16,
                recursion_check_stack: &mut bevy::ecs::__macro_exports::Vec<
                    bevy::ecs::component::ComponentId,
                >,
            ) {
                bevy::ecs::component::enforce_no_required_components_recursion(
                    components,
                    recursion_check_stack,
                );
                let self_id = components.register_component::<Self>();
                recursion_check_stack.push(self_id);
                recursion_check_stack.pop();
            }
            fn clone_behavior() -> bevy::ecs::component::ComponentCloneBehavior {
                use bevy::ecs::component::{
                    DefaultCloneBehaviorBase, DefaultCloneBehaviorViaClone,
                };
                (&&&bevy::ecs::component::DefaultCloneBehaviorSpecialization::<
                    Self,
                >::default())
                    .default_clone_behavior()
            }
        }
        pub struct RoomLink {
            position: Vec2,
            room: Entity,
        }
        #[automatically_derived]
        impl ::core::clone::Clone for RoomLink {
            #[inline]
            fn clone(&self) -> RoomLink {
                RoomLink {
                    position: ::core::clone::Clone::clone(&self.position),
                    room: ::core::clone::Clone::clone(&self.room),
                }
            }
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for RoomLink {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "RoomLink",
                    "position",
                    &self.position,
                    "room",
                    &&self.room,
                )
            }
        }
        pub fn validate(
            mut commands: Commands,
            mut door_e: EventReader<ComponentEvent<OnInsert, Door>>,
            wall_q: Query<&Wall>,
        ) {
            for door in door_e.read() {
                match wall_q.get(door.target) {
                    Ok(wall) if wall.length() >= MIN_WIDTH => {}
                    Err(QueryEntityError::EntityDoesNotExist(..)) => {}
                    Err(QueryEntityError::AliasedMutability(..)) => {
                        ::core::panicking::panic(
                            "internal error: entered unreachable code",
                        )
                    }
                    Ok(_) | Err(QueryEntityError::QueryDoesNotMatch(..)) => {
                        commands.entity(door.target).try_remove::<Door>();
                    }
                }
            }
        }
        pub fn wall_replaced(trigger: Trigger<OnReplace, Wall>, mut commands: Commands) {
            commands.entity(trigger.target()).try_remove::<DoorLinks>();
        }
        pub fn remove_links(
            map_q: Query<&Map, (Changed<Map>, With<ChildOfRoot>)>,
            door_q: Query<Entity, With<DoorLinks>>,
            mut commands: Commands,
        ) {
            map_q
                .iter()
                .for_each(|map| {
                    for wall in map.walls() {
                        if door_q.contains(wall.id()) {
                            commands.entity(wall.id()).remove::<DoorLinks>();
                        }
                    }
                });
        }
        pub fn add_links(
            door_q: Query<
                (Entity, &Wall, &ChildOf),
                (With<Door>, With<ChildOfRoot>, Without<DoorLinks>),
            >,
            map_q: Query<&Map>,
            mut commands: Commands,
        ) -> Result {
            for (id, wall, parent) in door_q {
                let map = map_q.get(parent.parent())?;
                let [left, right] = map.wall_rooms(wall);
                commands.entity(id).insert(DoorLinks { left, right });
            }
            Ok(())
        }
        impl RoomLinks {
            pub fn doors(&self) -> impl Iterator<Item = (Entity, Entity, Vec2)> {
                self.doors.iter().map(|(&door, link)| (door, link.room, link.position))
            }
        }
        impl DoorLinks {
            fn on_insert(mut world: DeferredWorld, context: HookContext) {
                let door = world.entity(context.entity);
                let wall = door.get::<Wall>().unwrap().clone();
                let links = door.get::<DoorLinks>().unwrap().clone();
                world
                    .entity_mut(links.left)
                    .get_mut::<RoomLinks>()
                    .unwrap()
                    .doors
                    .insert(
                        context.entity,
                        RoomLink {
                            position: wall.position(),
                            room: links.right,
                        },
                    );
                world
                    .entity_mut(links.right)
                    .get_mut::<RoomLinks>()
                    .unwrap()
                    .doors
                    .insert(
                        context.entity,
                        RoomLink {
                            position: wall.position(),
                            room: links.left,
                        },
                    );
            }
            fn on_remove(mut world: DeferredWorld, context: HookContext) {
                let links = world
                    .entity(context.entity)
                    .get::<DoorLinks>()
                    .unwrap()
                    .clone();
                if let Ok(mut room) = world.get_entity_mut(links.left) {
                    room.get_mut::<RoomLinks>().unwrap().doors.remove(&context.entity);
                }
                if let Ok(mut room) = world.get_entity_mut(links.right) {
                    room.get_mut::<RoomLinks>().unwrap().doors.remove(&context.entity);
                }
            }
        }
    }
    pub mod mesh {
        use std::f32::consts::{FRAC_PI_2, FRAC_PI_4, SQRT_2, TAU};
        use bevy::{ecs::entity::EntityHashMap, math::FloatOrd, prelude::*};
        use polyanya::{
            Coords, Mesh, Path, Triangulation,
            geo::{Area, BooleanOps, Closest, ClosestPoint, Point, Polygon, unary_union},
        };
        use smallvec::SmallVec;
        use spade::Triangulation as _;
        use crate::{
            map::{Corner, Map, door::Door, wall::Wall},
            pawn::Pawn, root::ChildOfRoot,
        };
        pub struct MapMesh {
            islands: Vec<MapMeshIsland>,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for MapMesh {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field1_finish(
                    f,
                    "MapMesh",
                    "islands",
                    &&self.islands,
                )
            }
        }
        #[automatically_derived]
        impl ::core::default::Default for MapMesh {
            #[inline]
            fn default() -> MapMesh {
                MapMesh {
                    islands: ::core::default::Default::default(),
                }
            }
        }
        impl bevy::ecs::component::Component for MapMesh
        where
            Self: Send + Sync + 'static,
        {
            const STORAGE_TYPE: bevy::ecs::component::StorageType = bevy::ecs::component::StorageType::Table;
            type Mutability = bevy::ecs::component::Mutable;
            fn register_required_components(
                requiree: bevy::ecs::component::ComponentId,
                components: &mut bevy::ecs::component::ComponentsRegistrator,
                required_components: &mut bevy::ecs::component::RequiredComponents,
                inheritance_depth: u16,
                recursion_check_stack: &mut bevy::ecs::__macro_exports::Vec<
                    bevy::ecs::component::ComponentId,
                >,
            ) {
                bevy::ecs::component::enforce_no_required_components_recursion(
                    components,
                    recursion_check_stack,
                );
                let self_id = components.register_component::<Self>();
                recursion_check_stack.push(self_id);
                recursion_check_stack.pop();
            }
            fn clone_behavior() -> bevy::ecs::component::ComponentCloneBehavior {
                use bevy::ecs::component::{
                    DefaultCloneBehaviorBase, DefaultCloneBehaviorViaClone,
                };
                (&&&bevy::ecs::component::DefaultCloneBehaviorSpecialization::<
                    Self,
                >::default())
                    .default_clone_behavior()
            }
        }
        struct MapMeshIsland {
            mesh: Mesh,
            polygon: Polygon<f32>,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for MapMeshIsland {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "MapMeshIsland",
                    "mesh",
                    &self.mesh,
                    "polygon",
                    &&self.polygon,
                )
            }
        }
        const RADIUS: f32 = Wall::RADIUS + Pawn::RADIUS;
        struct CornerGeometry {
            center: Vec2,
            points: SmallVec<[CornerGeometryPoint; 4]>,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for CornerGeometry {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "CornerGeometry",
                    "center",
                    &self.center,
                    "points",
                    &&self.points,
                )
            }
        }
        struct CornerGeometryPoint {
            kind: CornerGeometryPointKind,
            point: Vec2,
        }
        #[automatically_derived]
        impl ::core::marker::Copy for CornerGeometryPoint {}
        #[automatically_derived]
        impl ::core::clone::Clone for CornerGeometryPoint {
            #[inline]
            fn clone(&self) -> CornerGeometryPoint {
                let _: ::core::clone::AssertParamIsClone<CornerGeometryPointKind>;
                let _: ::core::clone::AssertParamIsClone<Vec2>;
                *self
            }
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for CornerGeometryPoint {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "CornerGeometryPoint",
                    "kind",
                    &self.kind,
                    "point",
                    &&self.point,
                )
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for CornerGeometryPoint {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for CornerGeometryPoint {
            #[inline]
            fn eq(&self, other: &CornerGeometryPoint) -> bool {
                self.kind == other.kind && self.point == other.point
            }
        }
        enum CornerGeometryPointKind {
            Wall(Entity),
            Corner,
        }
        #[automatically_derived]
        impl ::core::marker::Copy for CornerGeometryPointKind {}
        #[automatically_derived]
        impl ::core::clone::Clone for CornerGeometryPointKind {
            #[inline]
            fn clone(&self) -> CornerGeometryPointKind {
                let _: ::core::clone::AssertParamIsClone<Entity>;
                *self
            }
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for CornerGeometryPointKind {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match self {
                    CornerGeometryPointKind::Wall(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "Wall",
                            &__self_0,
                        )
                    }
                    CornerGeometryPointKind::Corner => {
                        ::core::fmt::Formatter::write_str(f, "Corner")
                    }
                }
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for CornerGeometryPointKind {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for CornerGeometryPointKind {
            #[inline]
            fn eq(&self, other: &CornerGeometryPointKind) -> bool {
                let __self_discr = ::core::intrinsics::discriminant_value(self);
                let __arg1_discr = ::core::intrinsics::discriminant_value(other);
                __self_discr == __arg1_discr
                    && match (self, other) {
                        (
                            CornerGeometryPointKind::Wall(__self_0),
                            CornerGeometryPointKind::Wall(__arg1_0),
                        ) => __self_0 == __arg1_0,
                        _ => true,
                    }
            }
        }
        pub fn update_mesh(
            mut map_q: Query<(&Map, &mut MapMesh), (Changed<Map>, With<ChildOfRoot>)>,
            corner_q: Query<&Corner>,
            wall_q: Query<&Wall>,
            door_q: Query<&Door>,
        ) -> Result {
            for (map, mut mesh) in &mut map_q {
                if map.triangulation.all_vertices_on_line() {
                    mesh.islands.clear();
                    continue;
                }
                let mut corners = EntityHashMap::new();
                for entity in map.corners() {
                    let corner = corner_q.get(entity.id())?;
                    let geometry = CornerGeometry::new(
                        corner,
                        map
                            .corner_walls(corner)
                            .map(|(wall, end_corner)| Ok((
                                wall,
                                corner_q.get(end_corner)?,
                            ))),
                    )?;
                    corners.insert(entity.id(), geometry);
                }
                let mut interiors = Vec::with_capacity(
                    map.triangulation.num_constraints() * 3,
                );
                for (_, corner) in &corners {
                    interiors
                        .push(
                            Polygon::new(
                                corner
                                    .points
                                    .iter()
                                    .map(|point| point.point.to_array())
                                    .collect(),
                                ::alloc::vec::Vec::new(),
                            ),
                        );
                }
                for entity in map.walls() {
                    let wall = wall_q.get(entity.id())?;
                    let start_points = corners[&wall.start()]
                        .wall_intersections(entity.id())?;
                    let end_points = corners[&wall.end()]
                        .wall_intersections(entity.id())?;
                    if door_q.contains(entity.id()) {
                        let wall_half_len = wall.length() / 2.;
                        let door_start_points = [
                            wall.isometry()
                                * Vec2::new(-wall_half_len + RADIUS, -RADIUS),
                            wall.isometry() * Vec2::new(-wall_half_len + RADIUS, RADIUS),
                        ];
                        let door_end_points = [
                            wall.isometry() * Vec2::new(wall_half_len - RADIUS, RADIUS),
                            wall.isometry() * Vec2::new(wall_half_len - RADIUS, -RADIUS),
                        ];
                        interiors
                            .push(
                                Polygon::new(
                                    start_points
                                        .into_iter()
                                        .chain(door_start_points)
                                        .map(|point| point.to_array())
                                        .collect(),
                                    ::alloc::vec::Vec::new(),
                                ),
                            );
                        interiors
                            .push(
                                Polygon::new(
                                    end_points
                                        .into_iter()
                                        .chain(door_end_points)
                                        .map(|point| point.to_array())
                                        .collect(),
                                    ::alloc::vec::Vec::new(),
                                ),
                            );
                    } else {
                        interiors
                            .push(
                                Polygon::new(
                                    start_points
                                        .into_iter()
                                        .chain(end_points)
                                        .map(|point| point.to_array())
                                        .collect(),
                                    ::alloc::vec::Vec::new(),
                                ),
                            );
                    }
                }
                let exterior = Polygon::new(
                    map
                        .triangulation
                        .convex_hull()
                        .map(|edge| edge.from().data().position.to_array())
                        .collect(),
                    ::alloc::vec::Vec::new(),
                );
                let interior = unary_union(&interiors);
                mesh.islands.clear();
                mesh.islands
                    .extend(
                        exterior
                            .difference(&interior)
                            .into_iter()
                            .filter(|polygon| polygon.unsigned_area() > Pawn::AREA)
                            .map(|polygon| {
                                let layer = Triangulation::from_geo_polygon(polygon.clone())
                                    .as_layer();
                                let mut mesh = Mesh {
                                    layers: <[_]>::into_vec(::alloc::boxed::box_new([layer])),
                                    search_delta: RADIUS / 2.,
                                    search_steps: 2,
                                };
                                mesh.bake();
                                MapMeshIsland { mesh, polygon }
                            }),
                    );
            }
            Ok(())
        }
        impl MapMesh {
            pub fn path(&self, from: Vec2, to: Vec2) -> Option<Path> {
                if self.islands.len() == 1 {
                    return self.islands[0].path(from, to);
                }
                let (index, from) = self
                    .islands
                    .iter()
                    .enumerate()
                    .flat_map(|(index, island)| Some((
                        index,
                        island.closest_point(from)?,
                    )))
                    .min_by_key(|(_, closest)| FloatOrd(
                        closest.position().distance_squared(from),
                    ))?;
                self.islands[index].path_from(from, to)
            }
            pub fn meshes(&self) -> impl Iterator<Item = &'_ Mesh> {
                self.islands.iter().map(|island| &island.mesh)
            }
        }
        impl MapMeshIsland {
            fn path(&self, from: Vec2, to: Vec2) -> Option<Path> {
                let from = self.closest_point(from)?;
                self.path_from(from, to)
            }
            fn path_from(&self, from: Coords, to: Vec2) -> Option<Path> {
                let to = self.closest_point(to)?;
                self.mesh.path(from, to)
            }
            fn closest_point(&self, point: Vec2) -> Option<Coords> {
                if let Some(coords) = self.mesh.get_closest_point(point) {
                    return Some(coords);
                }
                match self.polygon.closest_point(&Point::new(point.x, point.y)) {
                    Closest::Intersection(closest) | Closest::SinglePoint(closest) => {
                        if let Some(coords)
                            = self
                                .mesh
                                .get_closest_point(Vec2::new(closest.x(), closest.y()))
                        {
                            Some(coords)
                        } else if let Some(coords)
                            = self
                                .mesh
                                .get_closest_point_towards(
                                    point,
                                    Vec2::new(closest.x(), closest.y()),
                                )
                        {
                            Some(coords)
                        } else {
                            {
                                use ::tracing::__macro_support::Callsite as _;
                                static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                    static META: ::tracing::Metadata<'static> = {
                                        ::tracing_core::metadata::Metadata::new(
                                            "event pb-engine\\src\\map\\mesh.rs:214",
                                            "pb_engine::map::mesh",
                                            ::tracing::Level::ERROR,
                                            ::tracing_core::__macro_support::Option::Some(
                                                "pb-engine\\src\\map\\mesh.rs",
                                            ),
                                            ::tracing_core::__macro_support::Option::Some(214u32),
                                            ::tracing_core::__macro_support::Option::Some(
                                                "pb_engine::map::mesh",
                                            ),
                                            ::tracing_core::field::FieldSet::new(
                                                &["message"],
                                                ::tracing_core::callsite::Identifier(&__CALLSITE),
                                            ),
                                            ::tracing::metadata::Kind::EVENT,
                                        )
                                    };
                                    ::tracing::callsite::DefaultCallsite::new(&META)
                                };
                                let enabled = ::tracing::Level::ERROR
                                    <= ::tracing::level_filters::STATIC_MAX_LEVEL
                                    && ::tracing::Level::ERROR
                                        <= ::tracing::level_filters::LevelFilter::current()
                                    && {
                                        let interest = __CALLSITE.interest();
                                        !interest.is_never()
                                            && ::tracing::__macro_support::__is_enabled(
                                                __CALLSITE.metadata(),
                                                interest,
                                            )
                                    };
                                if enabled {
                                    (|value_set: ::tracing::field::ValueSet| {
                                        let meta = __CALLSITE.metadata();
                                        ::tracing::Event::dispatch(meta, &value_set);
                                    })({
                                        #[allow(unused_imports)]
                                        use ::tracing::field::{debug, display, Value};
                                        let mut iter = __CALLSITE.metadata().fields().iter();
                                        __CALLSITE
                                            .metadata()
                                            .fields()
                                            .value_set(
                                                &[
                                                    (
                                                        &::tracing::__macro_support::Iterator::next(&mut iter)
                                                            .expect("FieldSet corrupted (this is a bug)"),
                                                        ::tracing::__macro_support::Option::Some(
                                                            &format_args!(
                                                                "closest point {0:?} for target point {1:?} was not found in the mesh",
                                                                closest,
                                                                point,
                                                            ) as &dyn Value,
                                                        ),
                                                    ),
                                                ],
                                            )
                                    });
                                } else {
                                }
                            };
                            None
                        }
                    }
                    Closest::Indeterminate => {
                        {
                            use ::tracing::__macro_support::Callsite as _;
                            static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                static META: ::tracing::Metadata<'static> = {
                                    ::tracing_core::metadata::Metadata::new(
                                        "event pb-engine\\src\\map\\mesh.rs:221",
                                        "pb_engine::map::mesh",
                                        ::tracing::Level::ERROR,
                                        ::tracing_core::__macro_support::Option::Some(
                                            "pb-engine\\src\\map\\mesh.rs",
                                        ),
                                        ::tracing_core::__macro_support::Option::Some(221u32),
                                        ::tracing_core::__macro_support::Option::Some(
                                            "pb_engine::map::mesh",
                                        ),
                                        ::tracing_core::field::FieldSet::new(
                                            &["message"],
                                            ::tracing_core::callsite::Identifier(&__CALLSITE),
                                        ),
                                        ::tracing::metadata::Kind::EVENT,
                                    )
                                };
                                ::tracing::callsite::DefaultCallsite::new(&META)
                            };
                            let enabled = ::tracing::Level::ERROR
                                <= ::tracing::level_filters::STATIC_MAX_LEVEL
                                && ::tracing::Level::ERROR
                                    <= ::tracing::level_filters::LevelFilter::current()
                                && {
                                    let interest = __CALLSITE.interest();
                                    !interest.is_never()
                                        && ::tracing::__macro_support::__is_enabled(
                                            __CALLSITE.metadata(),
                                            interest,
                                        )
                                };
                            if enabled {
                                (|value_set: ::tracing::field::ValueSet| {
                                    let meta = __CALLSITE.metadata();
                                    ::tracing::Event::dispatch(meta, &value_set);
                                })({
                                    #[allow(unused_imports)]
                                    use ::tracing::field::{debug, display, Value};
                                    let mut iter = __CALLSITE.metadata().fields().iter();
                                    __CALLSITE
                                        .metadata()
                                        .fields()
                                        .value_set(
                                            &[
                                                (
                                                    &::tracing::__macro_support::Iterator::next(&mut iter)
                                                        .expect("FieldSet corrupted (this is a bug)"),
                                                    ::tracing::__macro_support::Option::Some(
                                                        &format_args!(
                                                            "indeterminate closest point to {0:?} on polygon",
                                                            point,
                                                        ) as &dyn Value,
                                                    ),
                                                ),
                                            ],
                                        )
                                });
                            } else {
                            }
                        };
                        None
                    }
                }
            }
        }
        impl CornerGeometry {
            fn new<'a>(
                start: &Corner,
                walls: impl Iterator<Item = Result<(Entity, &'a Corner)>>,
            ) -> Result<Self> {
                let pos = start.position();
                let mut angles: SmallVec<[(Entity, f32); 4]> = walls
                    .map(|res| {
                        res.map(|(id, end)| (id, (end.position() - pos).to_angle()))
                    })
                    .collect::<Result<_>>()?;
                angles.sort_by_key(|&(_, angle)| FloatOrd(angle));
                let mut points: SmallVec<[CornerGeometryPoint; 4]> = default();
                for (index, &(wall, a2)) in angles.iter().enumerate() {
                    if index == 0 {
                        let a1 = wrapping_idx(&angles, index, -1).1;
                        points
                            .extend(
                                corner_intersections(pos, a1, a2)
                                    .map(CornerGeometryPoint::corner),
                            );
                    }
                    points
                        .push(
                            CornerGeometryPoint::wall(
                                pos + Vec2::from_angle(a2) * RADIUS,
                                wall,
                            ),
                        );
                    if index != (angles.len() - 1) {
                        let a3 = wrapping_idx(&angles, index, 1).1;
                        points
                            .extend(
                                corner_intersections(pos, a2, a3)
                                    .map(CornerGeometryPoint::corner),
                            );
                    }
                }
                Ok(CornerGeometry {
                    points,
                    center: pos,
                })
            }
            fn wall_intersections(&self, wall: Entity) -> Result<[Vec2; 3]> {
                let index = self
                    .points
                    .iter()
                    .position(|p| p.kind == CornerGeometryPointKind::Wall(wall))
                    .ok_or("wall not found")?;
                Ok([
                    wrapping_idx(&self.points, index, 1).point,
                    self.center,
                    wrapping_idx(&self.points, index, -1).point,
                ])
            }
        }
        impl CornerGeometryPoint {
            fn wall(point: Vec2, wall: Entity) -> Self {
                CornerGeometryPoint {
                    point,
                    kind: CornerGeometryPointKind::Wall(wall),
                }
            }
            fn corner(point: Vec2) -> Self {
                CornerGeometryPoint {
                    point,
                    kind: CornerGeometryPointKind::Corner,
                }
            }
        }
        fn corner_intersections(
            pos: Vec2,
            a1: f32,
            a2: f32,
        ) -> impl Iterator<Item = Vec2> {
            let da = angle_delta(a1, a2);
            let mut result = SmallVec::<[Vec2; 2]>::new();
            if da > 3. * FRAC_PI_2 {
                result
                    .extend_from_slice(
                        &[
                            pos + right_angle_intersection(a1 + 3. * FRAC_PI_4),
                            pos + right_angle_intersection(a2 - 3. * FRAC_PI_4),
                        ],
                    );
            } else {
                let mid = a1 + da / 2.;
                result.push(pos + angle_intersection(mid, da / 2.));
            }
            result.into_iter()
        }
        fn right_angle_intersection(a: f32) -> Vec2 {
            Vec2::from_angle(a) * RADIUS * SQRT_2
        }
        fn angle_intersection(mid: f32, da: f32) -> Vec2 {
            Vec2::from_angle(mid) * RADIUS / f32::sin(da)
        }
        fn angle_delta(a1: f32, a2: f32) -> f32 {
            if a1 == a2 { TAU } else { (a2 - a1).rem_euclid(TAU) }
        }
        fn wrapping_idx<T>(slice: &[T], index: usize, offset: isize) -> &T {
            &slice[(index as isize + offset).rem_euclid(slice.len() as isize) as usize]
        }
    }
    pub mod perimeter {
        use avian2d::prelude::*;
        use bevy::prelude::*;
        use pb_util::event::ComponentEvent;
        use crate::{layer::Layer, root::ChildOfRoot};
        #[require(Transform, Visibility)]
        #[component(immutable)]
        pub struct Perimeter {
            start: Vec2,
            end: Vec2,
        }
        #[automatically_derived]
        impl ::core::clone::Clone for Perimeter {
            #[inline]
            fn clone(&self) -> Perimeter {
                Perimeter {
                    start: ::core::clone::Clone::clone(&self.start),
                    end: ::core::clone::Clone::clone(&self.end),
                }
            }
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Perimeter {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "Perimeter",
                    "start",
                    &self.start,
                    "end",
                    &&self.end,
                )
            }
        }
        #[doc = "**Required Components**: [`Transform`], [`Visibility`]. \n\n A component's Required Components are inserted whenever it is inserted. Note that this will also insert the required components _of_ the required components, recursively, in depth-first order."]
        impl bevy::ecs::component::Component for Perimeter
        where
            Self: Send + Sync + 'static,
        {
            const STORAGE_TYPE: bevy::ecs::component::StorageType = bevy::ecs::component::StorageType::Table;
            type Mutability = bevy::ecs::component::Immutable;
            fn register_required_components(
                requiree: bevy::ecs::component::ComponentId,
                components: &mut bevy::ecs::component::ComponentsRegistrator,
                required_components: &mut bevy::ecs::component::RequiredComponents,
                inheritance_depth: u16,
                recursion_check_stack: &mut bevy::ecs::__macro_exports::Vec<
                    bevy::ecs::component::ComponentId,
                >,
            ) {
                bevy::ecs::component::enforce_no_required_components_recursion(
                    components,
                    recursion_check_stack,
                );
                let self_id = components.register_component::<Self>();
                recursion_check_stack.push(self_id);
                components
                    .register_required_components_manual::<
                        Self,
                        Transform,
                    >(
                        required_components,
                        <Transform as Default>::default,
                        inheritance_depth,
                        recursion_check_stack,
                    );
                components
                    .register_required_components_manual::<
                        Self,
                        Visibility,
                    >(
                        required_components,
                        <Visibility as Default>::default,
                        inheritance_depth,
                        recursion_check_stack,
                    );
                <Transform as bevy::ecs::component::Component>::register_required_components(
                    requiree,
                    components,
                    required_components,
                    inheritance_depth + 1,
                    recursion_check_stack,
                );
                <Visibility as bevy::ecs::component::Component>::register_required_components(
                    requiree,
                    components,
                    required_components,
                    inheritance_depth + 1,
                    recursion_check_stack,
                );
                recursion_check_stack.pop();
            }
            fn clone_behavior() -> bevy::ecs::component::ComponentCloneBehavior {
                use bevy::ecs::component::{
                    DefaultCloneBehaviorBase, DefaultCloneBehaviorViaClone,
                };
                (&&&bevy::ecs::component::DefaultCloneBehaviorSpecialization::<
                    Self,
                >::default())
                    .default_clone_behavior()
            }
        }
        pub fn add_colliders(
            mut commands: Commands,
            mut perimeter_e: EventReader<ComponentEvent<OnInsert, Perimeter>>,
            root_q: Query<&ChildOfRoot>,
        ) -> Result {
            for event in perimeter_e.read() {
                if root_q.contains(event.target) {
                    commands
                        .entity(event.target)
                        .insert((
                            RigidBody::Static,
                            Collider::half_space(Vec2::Y),
                            CollisionLayers::new(Layer::Perimeter, LayerMask::ALL),
                        ));
                }
            }
            Ok(())
        }
        impl Perimeter {
            pub fn start(&self) -> Vec2 {
                self.start
            }
            pub fn end(&self) -> Vec2 {
                self.end
            }
            pub(crate) fn bundle(start: Vec2, end: Vec2) -> impl Bundle {
                let position = start.midpoint(end);
                let rotation = (start - end).to_angle();
                (
                    Name::new(
                        ::alloc::__export::must_use({
                            let res = ::alloc::fmt::format(
                                format_args!(
                                    "perimeter wall ({0}, {1}) to ({2}, {3})",
                                    start.x,
                                    start.y,
                                    end.x,
                                    end.y,
                                ),
                            );
                            res
                        }),
                    ),
                    Perimeter { start, end },
                    Transform {
                        scale: Vec3::ONE,
                        translation: position.extend(0.),
                        rotation: Quat::from_rotation_z(rotation),
                    },
                )
            }
        }
    }
    pub mod room {
        use bevy::{
            ecs::{entity::EntityHashSet, relationship::Relationship},
            prelude::*,
        };
        use spade::handles::{
            FixedFaceHandle, FixedVertexHandle, OUTER_FACE, PossiblyOuterTag,
        };
        use crate::{
            map::{Map, door::RoomLinks},
            pawn::Pawn, root::ChildOfRoot,
        };
        #[require(Transform, Visibility, RoomLinks)]
        #[component(immutable)]
        pub struct Room {
            faces: Vec<FixedFaceHandle<PossiblyOuterTag>>,
        }
        #[automatically_derived]
        impl ::core::clone::Clone for Room {
            #[inline]
            fn clone(&self) -> Room {
                Room {
                    faces: ::core::clone::Clone::clone(&self.faces),
                }
            }
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Room {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field1_finish(
                    f,
                    "Room",
                    "faces",
                    &&self.faces,
                )
            }
        }
        #[doc = "**Required Components**: [`Transform`], [`Visibility`], [`RoomLinks`]. \n\n A component's Required Components are inserted whenever it is inserted. Note that this will also insert the required components _of_ the required components, recursively, in depth-first order."]
        impl bevy::ecs::component::Component for Room
        where
            Self: Send + Sync + 'static,
        {
            const STORAGE_TYPE: bevy::ecs::component::StorageType = bevy::ecs::component::StorageType::Table;
            type Mutability = bevy::ecs::component::Immutable;
            fn register_required_components(
                requiree: bevy::ecs::component::ComponentId,
                components: &mut bevy::ecs::component::ComponentsRegistrator,
                required_components: &mut bevy::ecs::component::RequiredComponents,
                inheritance_depth: u16,
                recursion_check_stack: &mut bevy::ecs::__macro_exports::Vec<
                    bevy::ecs::component::ComponentId,
                >,
            ) {
                bevy::ecs::component::enforce_no_required_components_recursion(
                    components,
                    recursion_check_stack,
                );
                let self_id = components.register_component::<Self>();
                recursion_check_stack.push(self_id);
                components
                    .register_required_components_manual::<
                        Self,
                        Transform,
                    >(
                        required_components,
                        <Transform as Default>::default,
                        inheritance_depth,
                        recursion_check_stack,
                    );
                components
                    .register_required_components_manual::<
                        Self,
                        Visibility,
                    >(
                        required_components,
                        <Visibility as Default>::default,
                        inheritance_depth,
                        recursion_check_stack,
                    );
                components
                    .register_required_components_manual::<
                        Self,
                        RoomLinks,
                    >(
                        required_components,
                        <RoomLinks as Default>::default,
                        inheritance_depth,
                        recursion_check_stack,
                    );
                <Transform as bevy::ecs::component::Component>::register_required_components(
                    requiree,
                    components,
                    required_components,
                    inheritance_depth + 1,
                    recursion_check_stack,
                );
                <Visibility as bevy::ecs::component::Component>::register_required_components(
                    requiree,
                    components,
                    required_components,
                    inheritance_depth + 1,
                    recursion_check_stack,
                );
                <RoomLinks as bevy::ecs::component::Component>::register_required_components(
                    requiree,
                    components,
                    required_components,
                    inheritance_depth + 1,
                    recursion_check_stack,
                );
                recursion_check_stack.pop();
            }
            fn clone_behavior() -> bevy::ecs::component::ComponentCloneBehavior {
                use bevy::ecs::component::{
                    DefaultCloneBehaviorBase, DefaultCloneBehaviorViaClone,
                };
                (&&&bevy::ecs::component::DefaultCloneBehaviorSpecialization::<
                    Self,
                >::default())
                    .default_clone_behavior()
            }
        }
        #[relationship(relationship_target = RoomContents)]
        pub struct ContainingRoom {
            #[relationship]
            room: Entity,
            hint: Option<FixedVertexHandle>,
        }
        impl bevy::ecs::component::Component for ContainingRoom
        where
            Self: Send + Sync + 'static,
        {
            const STORAGE_TYPE: bevy::ecs::component::StorageType = bevy::ecs::component::StorageType::Table;
            type Mutability = bevy::ecs::component::Immutable;
            fn register_required_components(
                requiree: bevy::ecs::component::ComponentId,
                components: &mut bevy::ecs::component::ComponentsRegistrator,
                required_components: &mut bevy::ecs::component::RequiredComponents,
                inheritance_depth: u16,
                recursion_check_stack: &mut bevy::ecs::__macro_exports::Vec<
                    bevy::ecs::component::ComponentId,
                >,
            ) {
                bevy::ecs::component::enforce_no_required_components_recursion(
                    components,
                    recursion_check_stack,
                );
                let self_id = components.register_component::<Self>();
                recursion_check_stack.push(self_id);
                recursion_check_stack.pop();
            }
            fn on_insert() -> ::core::option::Option<
                bevy::ecs::component::ComponentHook,
            > {
                ::core::option::Option::Some(
                    <Self as bevy::ecs::relationship::Relationship>::on_insert,
                )
            }
            fn on_replace() -> ::core::option::Option<
                bevy::ecs::component::ComponentHook,
            > {
                ::core::option::Option::Some(
                    <Self as bevy::ecs::relationship::Relationship>::on_replace,
                )
            }
            fn clone_behavior() -> bevy::ecs::component::ComponentCloneBehavior {
                use bevy::ecs::component::{
                    DefaultCloneBehaviorBase, DefaultCloneBehaviorViaClone,
                };
                (&&&bevy::ecs::component::DefaultCloneBehaviorSpecialization::<
                    Self,
                >::default())
                    .default_clone_behavior()
            }
            fn map_entities<M: bevy::ecs::entity::EntityMapper>(
                this: &mut Self,
                mapper: &mut M,
            ) {
                use bevy::ecs::entity::MapEntities;
                this.room.map_entities(mapper);
            }
        }
        impl bevy::ecs::relationship::Relationship for ContainingRoom {
            type RelationshipTarget = RoomContents;
            #[inline(always)]
            fn get(&self) -> bevy::ecs::entity::Entity {
                self.room
            }
            #[inline]
            fn from(entity: bevy::ecs::entity::Entity) -> Self {
                Self {
                    hint: core::default::Default::default(),
                    room: entity,
                }
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for ContainingRoom {
            #[inline]
            fn clone(&self) -> ContainingRoom {
                ContainingRoom {
                    room: ::core::clone::Clone::clone(&self.room),
                    hint: ::core::clone::Clone::clone(&self.hint),
                }
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for ContainingRoom {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for ContainingRoom {
            #[inline]
            fn eq(&self, other: &ContainingRoom) -> bool {
                self.room == other.room && self.hint == other.hint
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Eq for ContainingRoom {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {
                let _: ::core::cmp::AssertParamIsEq<Entity>;
                let _: ::core::cmp::AssertParamIsEq<Option<FixedVertexHandle>>;
            }
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for ContainingRoom {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "ContainingRoom",
                    "room",
                    &self.room,
                    "hint",
                    &&self.hint,
                )
            }
        }
        #[relationship_target(relationship = ContainingRoom)]
        pub struct RoomContents(EntityHashSet);
        impl bevy::ecs::component::Component for RoomContents
        where
            Self: Send + Sync + 'static,
        {
            const STORAGE_TYPE: bevy::ecs::component::StorageType = bevy::ecs::component::StorageType::Table;
            type Mutability = bevy::ecs::component::Mutable;
            fn register_required_components(
                requiree: bevy::ecs::component::ComponentId,
                components: &mut bevy::ecs::component::ComponentsRegistrator,
                required_components: &mut bevy::ecs::component::RequiredComponents,
                inheritance_depth: u16,
                recursion_check_stack: &mut bevy::ecs::__macro_exports::Vec<
                    bevy::ecs::component::ComponentId,
                >,
            ) {
                bevy::ecs::component::enforce_no_required_components_recursion(
                    components,
                    recursion_check_stack,
                );
                let self_id = components.register_component::<Self>();
                recursion_check_stack.push(self_id);
                recursion_check_stack.pop();
            }
            fn on_replace() -> ::core::option::Option<
                bevy::ecs::component::ComponentHook,
            > {
                ::core::option::Option::Some(
                    <Self as bevy::ecs::relationship::RelationshipTarget>::on_replace,
                )
            }
            fn clone_behavior() -> bevy::ecs::component::ComponentCloneBehavior {
                bevy::ecs::component::ComponentCloneBehavior::Custom(
                    bevy::ecs::relationship::clone_relationship_target::<Self>,
                )
            }
            fn map_entities<M: bevy::ecs::entity::EntityMapper>(
                this: &mut Self,
                mapper: &mut M,
            ) {
                use bevy::ecs::entity::MapEntities;
                this.0.map_entities(mapper);
            }
        }
        impl bevy::ecs::relationship::RelationshipTarget for RoomContents {
            const LINKED_SPAWN: bool = false;
            type Relationship = ContainingRoom;
            type Collection = EntityHashSet;
            #[inline]
            fn collection(&self) -> &Self::Collection {
                &self.0
            }
            #[inline]
            fn collection_mut_risky(&mut self) -> &mut Self::Collection {
                &mut self.0
            }
            #[inline]
            fn from_collection_risky(collection: Self::Collection) -> Self {
                Self { 0: collection }
            }
        }
        #[automatically_derived]
        impl ::core::default::Default for RoomContents {
            #[inline]
            fn default() -> RoomContents {
                RoomContents(::core::default::Default::default())
            }
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for RoomContents {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_tuple_field1_finish(
                    f,
                    "RoomContents",
                    &&self.0,
                )
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for RoomContents {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for RoomContents {
            #[inline]
            fn eq(&self, other: &RoomContents) -> bool {
                self.0 == other.0
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Eq for RoomContents {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {
                let _: ::core::cmp::AssertParamIsEq<EntityHashSet>;
            }
        }
        pub fn room_replaced(trigger: Trigger<OnReplace, Room>, mut commands: Commands) {
            commands.entity(trigger.target()).try_remove::<RoomContents>();
        }
        pub fn update_containing_room(
            commands: ParallelCommands,
            map_q: Query<&Map, With<ChildOfRoot>>,
            item_q: Query<
                (Entity, &Transform, Option<&ContainingRoom>),
                (
                    With<Pawn>,
                    With<ChildOfRoot>,
                    Or<(Without<ContainingRoom>, Changed<Transform>)>,
                ),
            >,
        ) {
            item_q
                .par_iter()
                .for_each(|(id, transform, containing_room)| {
                    let hint = containing_room.and_then(|prev_room| prev_room.hint);
                    for map in &map_q {
                        if let Some((room, hint))
                            = map.containing_room(transform.translation.xy(), hint)
                        {
                            if containing_room
                                .is_none_or(|prev_room| prev_room.get() != room)
                            {
                                {
                                    use ::tracing::__macro_support::Callsite as _;
                                    static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                        static META: ::tracing::Metadata<'static> = {
                                            ::tracing_core::metadata::Metadata::new(
                                                "event pb-engine\\src\\map\\room.rs:57",
                                                "pb_engine::map::room",
                                                ::tracing::Level::INFO,
                                                ::tracing_core::__macro_support::Option::Some(
                                                    "pb-engine\\src\\map\\room.rs",
                                                ),
                                                ::tracing_core::__macro_support::Option::Some(57u32),
                                                ::tracing_core::__macro_support::Option::Some(
                                                    "pb_engine::map::room",
                                                ),
                                                ::tracing_core::field::FieldSet::new(
                                                    &["message"],
                                                    ::tracing_core::callsite::Identifier(&__CALLSITE),
                                                ),
                                                ::tracing::metadata::Kind::EVENT,
                                            )
                                        };
                                        ::tracing::callsite::DefaultCallsite::new(&META)
                                    };
                                    let enabled = ::tracing::Level::INFO
                                        <= ::tracing::level_filters::STATIC_MAX_LEVEL
                                        && ::tracing::Level::INFO
                                            <= ::tracing::level_filters::LevelFilter::current()
                                        && {
                                            let interest = __CALLSITE.interest();
                                            !interest.is_never()
                                                && ::tracing::__macro_support::__is_enabled(
                                                    __CALLSITE.metadata(),
                                                    interest,
                                                )
                                        };
                                    if enabled {
                                        (|value_set: ::tracing::field::ValueSet| {
                                            let meta = __CALLSITE.metadata();
                                            ::tracing::Event::dispatch(meta, &value_set);
                                        })({
                                            #[allow(unused_imports)]
                                            use ::tracing::field::{debug, display, Value};
                                            let mut iter = __CALLSITE.metadata().fields().iter();
                                            __CALLSITE
                                                .metadata()
                                                .fields()
                                                .value_set(
                                                    &[
                                                        (
                                                            &::tracing::__macro_support::Iterator::next(&mut iter)
                                                                .expect("FieldSet corrupted (this is a bug)"),
                                                            ::tracing::__macro_support::Option::Some(
                                                                &format_args!(
                                                                    "updated containing room {0} for {1}",
                                                                    room,
                                                                    id,
                                                                ) as &dyn Value,
                                                            ),
                                                        ),
                                                    ],
                                                )
                                        });
                                    } else {
                                    }
                                };
                                commands
                                    .command_scope(|mut commands| {
                                        commands
                                            .entity(id)
                                            .insert(ContainingRoom {
                                                room,
                                                hint: Some(hint),
                                            });
                                    });
                            }
                            return;
                        }
                    }
                    {
                        use ::tracing::__macro_support::Callsite as _;
                        static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                            static META: ::tracing::Metadata<'static> = {
                                ::tracing_core::metadata::Metadata::new(
                                    "event pb-engine\\src\\map\\room.rs:69",
                                    "pb_engine::map::room",
                                    ::tracing::Level::WARN,
                                    ::tracing_core::__macro_support::Option::Some(
                                        "pb-engine\\src\\map\\room.rs",
                                    ),
                                    ::tracing_core::__macro_support::Option::Some(69u32),
                                    ::tracing_core::__macro_support::Option::Some(
                                        "pb_engine::map::room",
                                    ),
                                    ::tracing_core::field::FieldSet::new(
                                        &["message"],
                                        ::tracing_core::callsite::Identifier(&__CALLSITE),
                                    ),
                                    ::tracing::metadata::Kind::EVENT,
                                )
                            };
                            ::tracing::callsite::DefaultCallsite::new(&META)
                        };
                        let enabled = ::tracing::Level::WARN
                            <= ::tracing::level_filters::STATIC_MAX_LEVEL
                            && ::tracing::Level::WARN
                                <= ::tracing::level_filters::LevelFilter::current()
                            && {
                                let interest = __CALLSITE.interest();
                                !interest.is_never()
                                    && ::tracing::__macro_support::__is_enabled(
                                        __CALLSITE.metadata(),
                                        interest,
                                    )
                            };
                        if enabled {
                            (|value_set: ::tracing::field::ValueSet| {
                                let meta = __CALLSITE.metadata();
                                ::tracing::Event::dispatch(meta, &value_set);
                            })({
                                #[allow(unused_imports)]
                                use ::tracing::field::{debug, display, Value};
                                let mut iter = __CALLSITE.metadata().fields().iter();
                                __CALLSITE
                                    .metadata()
                                    .fields()
                                    .value_set(
                                        &[
                                            (
                                                &::tracing::__macro_support::Iterator::next(&mut iter)
                                                    .expect("FieldSet corrupted (this is a bug)"),
                                                ::tracing::__macro_support::Option::Some(
                                                    &format_args!("no containing room found for {0}", id)
                                                        as &dyn Value,
                                                ),
                                            ),
                                        ],
                                    )
                            });
                        } else {
                        }
                    };
                    if containing_room.is_some() {
                        commands
                            .command_scope(|mut commands| {
                                commands.entity(id).remove::<ContainingRoom>();
                            });
                    }
                });
        }
        impl Room {
            pub fn is_outer(&self) -> bool {
                self.faces[0] == OUTER_FACE
            }
            pub(crate) fn faces(&self) -> &[FixedFaceHandle<PossiblyOuterTag>] {
                &self.faces
            }
            pub(crate) fn bundle(
                faces: Vec<FixedFaceHandle<PossiblyOuterTag>>,
            ) -> impl Bundle {
                if true {
                    if !!faces.is_empty() {
                        ::core::panicking::panic("assertion failed: !faces.is_empty()")
                    }
                }
                (Name::new("room"), Room { faces })
            }
        }
    }
    pub mod wall {
        use avian2d::prelude::*;
        use bevy::prelude::*;
        use pb_util::event::ComponentEvent;
        use spade::handles::FixedUndirectedEdgeHandle;
        use crate::{layer::Layer, root::ChildOfRoot};
        use super::door::Door;
        #[require(Transform, Visibility)]
        #[component(immutable)]
        pub struct Wall {
            edge: FixedUndirectedEdgeHandle,
            length: f32,
            position: Vec2,
            rotation: Rot2,
            corners: [Entity; 2],
        }
        #[automatically_derived]
        impl ::core::clone::Clone for Wall {
            #[inline]
            fn clone(&self) -> Wall {
                Wall {
                    edge: ::core::clone::Clone::clone(&self.edge),
                    length: ::core::clone::Clone::clone(&self.length),
                    position: ::core::clone::Clone::clone(&self.position),
                    rotation: ::core::clone::Clone::clone(&self.rotation),
                    corners: ::core::clone::Clone::clone(&self.corners),
                }
            }
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Wall {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field5_finish(
                    f,
                    "Wall",
                    "edge",
                    &self.edge,
                    "length",
                    &self.length,
                    "position",
                    &self.position,
                    "rotation",
                    &self.rotation,
                    "corners",
                    &&self.corners,
                )
            }
        }
        #[doc = "**Required Components**: [`Transform`], [`Visibility`]. \n\n A component's Required Components are inserted whenever it is inserted. Note that this will also insert the required components _of_ the required components, recursively, in depth-first order."]
        impl bevy::ecs::component::Component for Wall
        where
            Self: Send + Sync + 'static,
        {
            const STORAGE_TYPE: bevy::ecs::component::StorageType = bevy::ecs::component::StorageType::Table;
            type Mutability = bevy::ecs::component::Immutable;
            fn register_required_components(
                requiree: bevy::ecs::component::ComponentId,
                components: &mut bevy::ecs::component::ComponentsRegistrator,
                required_components: &mut bevy::ecs::component::RequiredComponents,
                inheritance_depth: u16,
                recursion_check_stack: &mut bevy::ecs::__macro_exports::Vec<
                    bevy::ecs::component::ComponentId,
                >,
            ) {
                bevy::ecs::component::enforce_no_required_components_recursion(
                    components,
                    recursion_check_stack,
                );
                let self_id = components.register_component::<Self>();
                recursion_check_stack.push(self_id);
                components
                    .register_required_components_manual::<
                        Self,
                        Transform,
                    >(
                        required_components,
                        <Transform as Default>::default,
                        inheritance_depth,
                        recursion_check_stack,
                    );
                components
                    .register_required_components_manual::<
                        Self,
                        Visibility,
                    >(
                        required_components,
                        <Visibility as Default>::default,
                        inheritance_depth,
                        recursion_check_stack,
                    );
                <Transform as bevy::ecs::component::Component>::register_required_components(
                    requiree,
                    components,
                    required_components,
                    inheritance_depth + 1,
                    recursion_check_stack,
                );
                <Visibility as bevy::ecs::component::Component>::register_required_components(
                    requiree,
                    components,
                    required_components,
                    inheritance_depth + 1,
                    recursion_check_stack,
                );
                recursion_check_stack.pop();
            }
            fn clone_behavior() -> bevy::ecs::component::ComponentCloneBehavior {
                use bevy::ecs::component::{
                    DefaultCloneBehaviorBase, DefaultCloneBehaviorViaClone,
                };
                (&&&bevy::ecs::component::DefaultCloneBehaviorSpecialization::<
                    Self,
                >::default())
                    .default_clone_behavior()
            }
        }
        pub fn add_colliders(
            mut commands: Commands,
            mut wall_e: EventReader<ComponentEvent<OnInsert, Wall>>,
            wall_q: Query<(&Wall, Has<Door>)>,
            root_q: Query<&ChildOfRoot>,
        ) -> Result {
            for event in wall_e.read() {
                if root_q.contains(event.target) {
                    let (wall, is_door) = wall_q.get(event.target)?;
                    if !is_door {
                        commands
                            .entity(event.target)
                            .insert((
                                RigidBody::Static,
                                Collider::rectangle(wall.length(), Wall::RADIUS * 2.),
                                CollisionLayers::new(Layer::Wall, LayerMask::ALL),
                            ));
                    }
                }
            }
            Ok(())
        }
        impl Wall {
            pub const RADIUS: f32 = 0.125;
            pub fn length(&self) -> f32 {
                self.length
            }
            pub fn corners(&self) -> [Entity; 2] {
                self.corners
            }
            pub fn start(&self) -> Entity {
                self.corners[0]
            }
            pub fn end(&self) -> Entity {
                self.corners[1]
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
            pub(crate) fn edge(&self) -> FixedUndirectedEdgeHandle {
                self.edge
            }
            pub(crate) fn bundle(
                edge: FixedUndirectedEdgeHandle,
                corners: [Entity; 2],
                [position1,
                position2,
                ]: [Vec2; 2],
            ) -> impl Bundle {
                let length = position1.distance(position2);
                let position = position1.midpoint(position2);
                let rotation = (position2 - position1).to_angle();
                (
                    Name::new(
                        ::alloc::__export::must_use({
                            let res = ::alloc::fmt::format(
                                format_args!(
                                    "wall ({0}, {1}) to ({2}, {3})",
                                    position1.x,
                                    position1.y,
                                    position2.x,
                                    position2.y,
                                ),
                            );
                            res
                        }),
                    ),
                    Wall {
                        edge,
                        length,
                        position,
                        rotation: Rot2::radians(rotation),
                        corners,
                    },
                    Transform {
                        scale: Vec3::ONE,
                        translation: position.extend(0.),
                        rotation: Quat::from_rotation_z(rotation),
                    },
                )
            }
        }
    }
    use std::{collections::HashSet, fmt};
    use bevy::{
        ecs::{entity::EntityHashSet, system::SystemParam},
        platform::collections::HashMap, prelude::*,
    };
    use mesh::MapMesh;
    use spade::{
        CdtEdge, ConstrainedDelaunayTriangulation, HasPosition, Point2,
        PositionInTriangulation, Triangulation,
        handles::{
            FaceHandle, FixedFaceHandle, FixedUndirectedEdgeHandle, FixedVertexHandle,
            OUTER_FACE, PossiblyOuterTag,
        },
    };
    use crate::{
        map::{corner::Corner, door::Door, perimeter::Perimeter, room::Room, wall::Wall},
        save::MapModel,
    };
    pub const GRID_SIZE: f32 = 4.0;
    #[require(Transform, Visibility, MapMesh, Name::new(Map::type_path()))]
    pub struct Map {
        id: Entity,
        children: EntityHashSet,
        size: u32,
        triangulation: ConstrainedDelaunayTriangulation<
            VertexData,
            (),
            UndirectedEdgeData,
            FaceData,
        >,
    }
    #[doc = "**Required Components**: [`Transform`], [`Visibility`], [`MapMesh`], [`Name`]. \n\n A component's Required Components are inserted whenever it is inserted. Note that this will also insert the required components _of_ the required components, recursively, in depth-first order."]
    impl bevy::ecs::component::Component for Map
    where
        Self: Send + Sync + 'static,
    {
        const STORAGE_TYPE: bevy::ecs::component::StorageType = bevy::ecs::component::StorageType::Table;
        type Mutability = bevy::ecs::component::Mutable;
        fn register_required_components(
            requiree: bevy::ecs::component::ComponentId,
            components: &mut bevy::ecs::component::ComponentsRegistrator,
            required_components: &mut bevy::ecs::component::RequiredComponents,
            inheritance_depth: u16,
            recursion_check_stack: &mut bevy::ecs::__macro_exports::Vec<
                bevy::ecs::component::ComponentId,
            >,
        ) {
            bevy::ecs::component::enforce_no_required_components_recursion(
                components,
                recursion_check_stack,
            );
            let self_id = components.register_component::<Self>();
            recursion_check_stack.push(self_id);
            components
                .register_required_components_manual::<
                    Self,
                    Transform,
                >(
                    required_components,
                    <Transform as Default>::default,
                    inheritance_depth,
                    recursion_check_stack,
                );
            components
                .register_required_components_manual::<
                    Self,
                    Visibility,
                >(
                    required_components,
                    <Visibility as Default>::default,
                    inheritance_depth,
                    recursion_check_stack,
                );
            components
                .register_required_components_manual::<
                    Self,
                    MapMesh,
                >(
                    required_components,
                    <MapMesh as Default>::default,
                    inheritance_depth,
                    recursion_check_stack,
                );
            components
                .register_required_components_manual::<
                    Self,
                    Name,
                >(
                    required_components,
                    || {
                        let x: Name = (|| Name::new(Map::type_path()))().into();
                        x
                    },
                    inheritance_depth,
                    recursion_check_stack,
                );
            <Transform as bevy::ecs::component::Component>::register_required_components(
                requiree,
                components,
                required_components,
                inheritance_depth + 1,
                recursion_check_stack,
            );
            <Visibility as bevy::ecs::component::Component>::register_required_components(
                requiree,
                components,
                required_components,
                inheritance_depth + 1,
                recursion_check_stack,
            );
            <MapMesh as bevy::ecs::component::Component>::register_required_components(
                requiree,
                components,
                required_components,
                inheritance_depth + 1,
                recursion_check_stack,
            );
            <Name as bevy::ecs::component::Component>::register_required_components(
                requiree,
                components,
                required_components,
                inheritance_depth + 1,
                recursion_check_stack,
            );
            recursion_check_stack.pop();
        }
        fn clone_behavior() -> bevy::ecs::component::ComponentCloneBehavior {
            use bevy::ecs::component::{
                DefaultCloneBehaviorBase, DefaultCloneBehaviorViaClone,
            };
            (&&&bevy::ecs::component::DefaultCloneBehaviorSpecialization::<
                Self,
            >::default())
                .default_clone_behavior()
        }
    }
    const _: () = {
        const _: () = {
            extern crate alloc;
            use alloc::string::ToString;
            impl bevy::reflect::TypePath for Map
            where
                Map: ::core::any::Any + ::core::marker::Send + ::core::marker::Sync,
            {
                fn type_path() -> &'static str {
                    "pb_engine::map::Map"
                }
                fn short_type_path() -> &'static str {
                    "Map"
                }
                fn type_ident() -> Option<&'static str> {
                    ::core::option::Option::Some("Map")
                }
                fn crate_name() -> Option<&'static str> {
                    ::core::option::Option::Some(
                        "pb_engine::map".split(':').next().unwrap(),
                    )
                }
                fn module_path() -> Option<&'static str> {
                    ::core::option::Option::Some("pb_engine::map")
                }
            }
        };
    };
    pub struct MapQueries<'w, 's> {
        pub commands: Commands<'w, 's>,
        pub corner_q: Query<'w, 's, &'static Corner>,
        pub wall_q: Query<'w, 's, &'static Wall>,
        pub perimeter_q: Query<'w, 's, &'static Perimeter>,
        pub room_q: Query<'w, 's, &'static Room>,
    }
    const _: () = {
        type __StructFieldsAlias<'w, 's> = (
            Commands<'w, 's>,
            Query<'w, 's, &'static Corner>,
            Query<'w, 's, &'static Wall>,
            Query<'w, 's, &'static Perimeter>,
            Query<'w, 's, &'static Room>,
        );
        #[doc(hidden)]
        pub struct FetchState {
            state: <__StructFieldsAlias<
                'static,
                'static,
            > as bevy::ecs::system::SystemParam>::State,
        }
        unsafe impl bevy::ecs::system::SystemParam for MapQueries<'_, '_> {
            type State = FetchState;
            type Item<'w, 's> = MapQueries<'w, 's>;
            fn init_state(
                world: &mut bevy::ecs::world::World,
                system_meta: &mut bevy::ecs::system::SystemMeta,
            ) -> Self::State {
                FetchState {
                    state: <__StructFieldsAlias<
                        '_,
                        '_,
                    > as bevy::ecs::system::SystemParam>::init_state(world, system_meta),
                }
            }
            unsafe fn new_archetype(
                state: &mut Self::State,
                archetype: &bevy::ecs::archetype::Archetype,
                system_meta: &mut bevy::ecs::system::SystemMeta,
            ) {
                unsafe {
                    <__StructFieldsAlias<
                        '_,
                        '_,
                    > as bevy::ecs::system::SystemParam>::new_archetype(
                        &mut state.state,
                        archetype,
                        system_meta,
                    )
                }
            }
            fn apply(
                state: &mut Self::State,
                system_meta: &bevy::ecs::system::SystemMeta,
                world: &mut bevy::ecs::world::World,
            ) {
                <__StructFieldsAlias<
                    '_,
                    '_,
                > as bevy::ecs::system::SystemParam>::apply(
                    &mut state.state,
                    system_meta,
                    world,
                );
            }
            fn queue(
                state: &mut Self::State,
                system_meta: &bevy::ecs::system::SystemMeta,
                world: bevy::ecs::world::DeferredWorld,
            ) {
                <__StructFieldsAlias<
                    '_,
                    '_,
                > as bevy::ecs::system::SystemParam>::queue(
                    &mut state.state,
                    system_meta,
                    world,
                );
            }
            #[inline]
            unsafe fn validate_param<'w, 's>(
                state: &'s Self::State,
                _system_meta: &bevy::ecs::system::SystemMeta,
                _world: bevy::ecs::world::unsafe_world_cell::UnsafeWorldCell<'w>,
            ) -> Result<(), bevy::ecs::system::SystemParamValidationError> {
                let FetchState { state: (f0, f1, f2, f3, f4) } = state;
                <Commands<
                    'w,
                    's,
                > as bevy::ecs::system::SystemParam>::validate_param(
                        f0,
                        _system_meta,
                        _world,
                    )
                    .map_err(|err| bevy::ecs::system::SystemParamValidationError::new::<
                        Self,
                    >(err.skipped, err.message, "::commands"))?;
                <Query<
                    'w,
                    's,
                    &'static Corner,
                > as bevy::ecs::system::SystemParam>::validate_param(
                        f1,
                        _system_meta,
                        _world,
                    )
                    .map_err(|err| bevy::ecs::system::SystemParamValidationError::new::<
                        Self,
                    >(err.skipped, err.message, "::corner_q"))?;
                <Query<
                    'w,
                    's,
                    &'static Wall,
                > as bevy::ecs::system::SystemParam>::validate_param(
                        f2,
                        _system_meta,
                        _world,
                    )
                    .map_err(|err| bevy::ecs::system::SystemParamValidationError::new::<
                        Self,
                    >(err.skipped, err.message, "::wall_q"))?;
                <Query<
                    'w,
                    's,
                    &'static Perimeter,
                > as bevy::ecs::system::SystemParam>::validate_param(
                        f3,
                        _system_meta,
                        _world,
                    )
                    .map_err(|err| bevy::ecs::system::SystemParamValidationError::new::<
                        Self,
                    >(err.skipped, err.message, "::perimeter_q"))?;
                <Query<
                    'w,
                    's,
                    &'static Room,
                > as bevy::ecs::system::SystemParam>::validate_param(
                        f4,
                        _system_meta,
                        _world,
                    )
                    .map_err(|err| bevy::ecs::system::SystemParamValidationError::new::<
                        Self,
                    >(err.skipped, err.message, "::room_q"))?;
                Result::Ok(())
            }
            #[inline]
            unsafe fn get_param<'w, 's>(
                state: &'s mut Self::State,
                system_meta: &bevy::ecs::system::SystemMeta,
                world: bevy::ecs::world::unsafe_world_cell::UnsafeWorldCell<'w>,
                change_tick: bevy::ecs::component::Tick,
            ) -> Self::Item<'w, 's> {
                let (f0, f1, f2, f3, f4) = <(
                    Commands<'w, 's>,
                    Query<'w, 's, &'static Corner>,
                    Query<'w, 's, &'static Wall>,
                    Query<'w, 's, &'static Perimeter>,
                    Query<'w, 's, &'static Room>,
                ) as bevy::ecs::system::SystemParam>::get_param(
                    &mut state.state,
                    system_meta,
                    world,
                    change_tick,
                );
                MapQueries {
                    commands: f0,
                    corner_q: f1,
                    wall_q: f2,
                    perimeter_q: f3,
                    room_q: f4,
                }
            }
        }
        unsafe impl<'w, 's> bevy::ecs::system::ReadOnlySystemParam for MapQueries<'w, 's>
        where
            Commands<'w, 's>: bevy::ecs::system::ReadOnlySystemParam,
            Query<'w, 's, &'static Corner>: bevy::ecs::system::ReadOnlySystemParam,
            Query<'w, 's, &'static Wall>: bevy::ecs::system::ReadOnlySystemParam,
            Query<'w, 's, &'static Perimeter>: bevy::ecs::system::ReadOnlySystemParam,
            Query<'w, 's, &'static Room>: bevy::ecs::system::ReadOnlySystemParam,
        {}
    };
    pub enum CornerDef {
        Corner(Entity),
        Position(Vec2),
        Wall(Entity, Vec2),
    }
    #[automatically_derived]
    impl ::core::marker::Copy for CornerDef {}
    #[automatically_derived]
    impl ::core::clone::Clone for CornerDef {
        #[inline]
        fn clone(&self) -> CornerDef {
            let _: ::core::clone::AssertParamIsClone<Entity>;
            let _: ::core::clone::AssertParamIsClone<Vec2>;
            *self
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for CornerDef {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                CornerDef::Corner(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Corner",
                        &__self_0,
                    )
                }
                CornerDef::Position(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Position",
                        &__self_0,
                    )
                }
                CornerDef::Wall(__self_0, __self_1) => {
                    ::core::fmt::Formatter::debug_tuple_field2_finish(
                        f,
                        "Wall",
                        __self_0,
                        &__self_1,
                    )
                }
            }
        }
    }
    /// Where an entity referenced by a map came from.
    pub enum MapEntity {
        /// This entity is a child of the original map this map was cloned from.
        Cloned(Entity),
        /// This entity exists in the original map, but has been replaced with a different entity in this map.
        Replaced(Entity, Entity),
        /// This entity was newly added in this map.
        Owned(Entity),
    }
    #[automatically_derived]
    impl ::core::marker::Copy for MapEntity {}
    #[automatically_derived]
    impl ::core::clone::Clone for MapEntity {
        #[inline]
        fn clone(&self) -> MapEntity {
            let _: ::core::clone::AssertParamIsClone<Entity>;
            *self
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for MapEntity {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                MapEntity::Cloned(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Cloned",
                        &__self_0,
                    )
                }
                MapEntity::Replaced(__self_0, __self_1) => {
                    ::core::fmt::Formatter::debug_tuple_field2_finish(
                        f,
                        "Replaced",
                        __self_0,
                        &__self_1,
                    )
                }
                MapEntity::Owned(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Owned",
                        &__self_0,
                    )
                }
            }
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for MapEntity {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for MapEntity {
        #[inline]
        fn eq(&self, other: &MapEntity) -> bool {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            let __arg1_discr = ::core::intrinsics::discriminant_value(other);
            __self_discr == __arg1_discr
                && match (self, other) {
                    (MapEntity::Cloned(__self_0), MapEntity::Cloned(__arg1_0)) => {
                        __self_0 == __arg1_0
                    }
                    (
                        MapEntity::Replaced(__self_0, __self_1),
                        MapEntity::Replaced(__arg1_0, __arg1_1),
                    ) => __self_0 == __arg1_0 && __self_1 == __arg1_1,
                    (MapEntity::Owned(__self_0), MapEntity::Owned(__arg1_0)) => {
                        __self_0 == __arg1_0
                    }
                    _ => unsafe { ::core::intrinsics::unreachable() }
                }
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Eq for MapEntity {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<Entity>;
        }
    }
    struct VertexData {
        corner: Option<MapEntity>,
        position: Vec2,
        standalone: bool,
    }
    #[automatically_derived]
    impl ::core::marker::Copy for VertexData {}
    #[automatically_derived]
    impl ::core::clone::Clone for VertexData {
        #[inline]
        fn clone(&self) -> VertexData {
            let _: ::core::clone::AssertParamIsClone<Option<MapEntity>>;
            let _: ::core::clone::AssertParamIsClone<Vec2>;
            let _: ::core::clone::AssertParamIsClone<bool>;
            *self
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for VertexData {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field3_finish(
                f,
                "VertexData",
                "corner",
                &self.corner,
                "position",
                &self.position,
                "standalone",
                &&self.standalone,
            )
        }
    }
    struct UndirectedEdgeData {
        wall: Option<MapEntity>,
    }
    #[automatically_derived]
    impl ::core::marker::Copy for UndirectedEdgeData {}
    #[automatically_derived]
    impl ::core::clone::Clone for UndirectedEdgeData {
        #[inline]
        fn clone(&self) -> UndirectedEdgeData {
            let _: ::core::clone::AssertParamIsClone<Option<MapEntity>>;
            *self
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for UndirectedEdgeData {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field1_finish(
                f,
                "UndirectedEdgeData",
                "wall",
                &&self.wall,
            )
        }
    }
    #[automatically_derived]
    impl ::core::default::Default for UndirectedEdgeData {
        #[inline]
        fn default() -> UndirectedEdgeData {
            UndirectedEdgeData {
                wall: ::core::default::Default::default(),
            }
        }
    }
    struct FaceData {
        room: Option<MapEntity>,
    }
    #[automatically_derived]
    impl ::core::marker::Copy for FaceData {}
    #[automatically_derived]
    impl ::core::clone::Clone for FaceData {
        #[inline]
        fn clone(&self) -> FaceData {
            let _: ::core::clone::AssertParamIsClone<Option<MapEntity>>;
            *self
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for FaceData {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field1_finish(
                f,
                "FaceData",
                "room",
                &&self.room,
            )
        }
    }
    #[automatically_derived]
    impl ::core::default::Default for FaceData {
        #[inline]
        fn default() -> FaceData {
            FaceData {
                room: ::core::default::Default::default(),
            }
        }
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
        pub fn from_model(
            model: &MapModel,
            entity_map: &mut impl EntityMapper,
        ) -> Result<Self> {
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
                let [from, to] = wall
                    .corners
                    .map(|id| {
                        triangulation.fixed_vertices().nth(corner_indices[&id]).unwrap()
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
                triangulation
                    .face_data_mut(face1)
                    .room = Some(MapEntity::Owned(entity_map.get_mapped(wall.rooms[0])));
                triangulation
                    .face_data_mut(face2)
                    .room = Some(MapEntity::Owned(entity_map.get_mapped(wall.rooms[1])));
            }
            let outer_room = entity_map.get_mapped(model.rooms[0].id);
            match &mut triangulation.face_data_mut(OUTER_FACE).room {
                Some(room) => {
                    match (&room.id(), &outer_room) {
                        (left_val, right_val) => {
                            if !(*left_val == *right_val) {
                                let kind = ::core::panicking::AssertKind::Eq;
                                ::core::panicking::assert_failed(
                                    kind,
                                    &*left_val,
                                    &*right_val,
                                    ::core::option::Option::None,
                                );
                            }
                        }
                    }
                }
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
                self
                    .triangulation
                    .vertex_data_mut(vertex)
                    .corner = source
                    .triangulation
                    .vertex(vertex)
                    .data()
                    .corner
                    .map(MapEntity::cloned);
            }
            for edge in self.triangulation.fixed_undirected_edges() {
                self
                    .triangulation
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
                self
                    .triangulation
                    .face_data_mut(face)
                    .room = source
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
                    self
                        .triangulation
                        .vertex_data_mut(vertex)
                        .corner = Some(corner.cloned());
                    source
                        .triangulation
                        .vertex_data_mut(vertex)
                        .corner = Some(corner.to_owned());
                    new_children.insert(corner.id());
                }
            }
            for edge in self.triangulation.fixed_undirected_edges() {
                if let Some(wall)
                    = self.triangulation.undirected_edge(edge).data().data().wall
                {
                    self
                        .triangulation
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
                queries.commands.entity(added_entity).insert(ChildOf(source.id()));
            }
            source.children = new_children;
            self.children.clear();
            source.size = self.size;
        }
        pub fn id(&self) -> Entity {
            self.id
        }
        pub fn corners(&self) -> impl Iterator<Item = MapEntity> + '_ {
            self.triangulation.vertices().filter_map(|vertex| vertex.data().corner)
        }
        pub fn walls(&self) -> impl Iterator<Item = MapEntity> + '_ {
            self.triangulation
                .undirected_edges()
                .filter(|edge| edge.is_constraint_edge())
                .map(|edge| edge.data().data().wall.unwrap())
        }
        pub fn rooms(&self) -> impl Iterator<Item = MapEntity> + '_ {
            self.triangulation.all_faces().map(|face| face.data().room.unwrap())
        }
        pub fn rooms_deduped(&self) -> impl Iterator<Item = MapEntity> + '_ {
            let mut unique = EntityHashSet::default();
            self.rooms().filter(move |&face| unique.insert(face.id()))
        }
        pub fn perimeter(&self) -> impl Iterator<Item = MapEntity> + '_ {
            self.triangulation
                .convex_hull()
                .map(|edge| edge.as_undirected().data().data().wall.unwrap())
        }
        pub fn perimeter_room(&self) -> MapEntity {
            self.triangulation.face(OUTER_FACE).data().room.unwrap()
        }
        pub fn corner_walls(
            &self,
            corner: &Corner,
        ) -> impl Iterator<Item = (Entity, Entity)> + '_ {
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
            let directed = self.triangulation.undirected_edge(wall.edge()).as_directed();
            [directed.face().data().room(), directed.rev().face().data().room()]
        }
        pub fn containing_room(
            &self,
            position: Vec2,
            hint: Option<FixedVertexHandle>,
        ) -> Option<(Entity, FixedVertexHandle)> {
            let point = Point2::new(position.x, position.y);
            let location = match hint {
                Some(hint) => self.triangulation.locate_with_hint(point, hint),
                None => self.triangulation.locate(point),
            };
            match location {
                PositionInTriangulation::OnVertex(vertex) => {
                    self.triangulation
                        .vertex(vertex)
                        .out_edge()
                        .map(|edge| (edge.face().data().room(), vertex))
                }
                PositionInTriangulation::OnEdge(edge) => {
                    let edge = self.triangulation.directed_edge(edge);
                    Some((edge.face().data().room(), edge.from().fix()))
                }
                PositionInTriangulation::OnFace(face) => {
                    let face = self.triangulation.face(face);
                    Some((face.data().room(), face.adjacent_edge().from().fix()))
                }
                PositionInTriangulation::OutsideOfConvexHull(_)
                | PositionInTriangulation::NoTriangulation => None,
            }
        }
        pub fn insert_corner(
            &mut self,
            queries: &mut MapQueries,
            corner: CornerDef,
        ) -> Result<Entity> {
            let vertex = self.get_or_insert_vertex(queries, corner)?;
            self.sync(queries);
            Ok(self.triangulation.vertex(vertex).data().corner())
        }
        pub fn remove_corner(
            &mut self,
            queries: &mut MapQueries,
            corner: Entity,
        ) -> Result {
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
                Ok(
                    Some((
                        self.triangulation.vertex(start).data().corner(),
                        self.triangulation.vertex(end).data().corner(),
                    )),
                )
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
                    let wall = match self
                        .triangulation
                        .undirected_edge(edge)
                        .data()
                        .data()
                        .wall
                    {
                        Some(wall) => queries.update(self.id, wall, bundle.clone()),
                        None => queries.spawn(self.id, bundle.clone()),
                    };
                    self
                        .triangulation
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
                Ok(
                    Some((
                        self.triangulation.vertex(start).data().corner(),
                        walls,
                        self.triangulation.vertex(end).data().corner(),
                    )),
                )
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
                (
                    CornerDef::Wall(wall1, position1),
                    CornerDef::Wall(wall2, position2),
                ) if wall1 == wall2 => {
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
                _ => {
                    Ok((
                        self.get_or_insert_vertex(queries, target1)?,
                        self.get_or_insert_vertex(queries, target2)?,
                    ))
                }
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
            let new_size = (point.x.abs().max(point.y.abs()) / GRID_SIZE).ceil() + 1.;
            if new_size > self.size as f32 {
                self.triangulation
                    .insert(
                        VertexData::new(
                            Vec2::new(-new_size * GRID_SIZE, -new_size * GRID_SIZE),
                        ),
                    )?;
                self.triangulation
                    .insert(
                        VertexData::new(
                            Vec2::new(-new_size * GRID_SIZE, new_size * GRID_SIZE),
                        ),
                    )?;
                self.triangulation
                    .insert(
                        VertexData::new(
                            Vec2::new(new_size * GRID_SIZE, new_size * GRID_SIZE),
                        ),
                    )?;
                self.triangulation
                    .insert(
                        VertexData::new(
                            Vec2::new(new_size * GRID_SIZE, -new_size * GRID_SIZE),
                        ),
                    )?;
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
        fn sync_vertices(
            &mut self,
            queries: &mut MapQueries,
            new_children: &mut EntityHashSet,
        ) {
            for vertex in self.triangulation.fixed_vertices() {
                let vertex = self.triangulation.vertex(vertex);
                let vertex_data = vertex.data();
                let is_corner = vertex.out_edges().any(|e| e.is_constraint_edge());
                let vertex = vertex.fix();
                if vertex_data.standalone || is_corner {
                    let corner = self
                        .update_corner(
                            queries,
                            vertex_data.corner,
                            vertex,
                            vertex_data.position,
                        );
                    self.triangulation.vertex_data_mut(vertex).corner = Some(corner);
                    if !corner.is_cloned() {
                        new_children.insert(corner.id());
                    }
                } else if vertex_data.corner.is_some() {
                    self.triangulation.vertex_data_mut(vertex).corner = None;
                }
            }
        }
        fn sync_faces(
            &mut self,
            queries: &mut MapQueries,
            new_children: &mut EntityHashSet,
        ) {
            let mut visited_faces = HashSet::new();
            let mut visited_rooms = EntityHashSet::default();
            for face in self.triangulation.fixed_all_faces() {
                if !visited_faces.insert(face) {
                    continue;
                }
                let mut open = <[_]>::into_vec(::alloc::boxed::box_new([face]));
                let mut faces = <[_]>::into_vec(::alloc::boxed::box_new([face]));
                let mut room = None;
                while let Some(face) = open.pop() {
                    self.for_each_adjacent_face(
                        face,
                        |adjacent_face| {
                            if !visited_faces.insert(adjacent_face.fix()) {
                                return;
                            }
                            open.push(adjacent_face.fix());
                            faces.push(adjacent_face.fix());
                            if room.is_none() {
                                if let Some(adjacent_face_room) = adjacent_face.data().room
                                {
                                    if visited_rooms.insert(adjacent_face_room.id()) {
                                        room = Some(adjacent_face_room);
                                    }
                                }
                            }
                        },
                    );
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
        fn sync_edges(
            &mut self,
            queries: &mut MapQueries,
            new_children: &mut EntityHashSet,
        ) {
            for edge in self.triangulation.fixed_undirected_edges() {
                let edge = self.triangulation.undirected_edge(edge);
                let edge_data = *edge.data().data();
                let is_wall = edge.is_constraint_edge();
                let is_perimeter = edge.is_part_of_convex_hull();
                if true {
                    if !!(is_wall && is_perimeter) {
                        ::core::panicking::panic(
                            "assertion failed: !(is_wall && is_perimeter)",
                        )
                    }
                }
                if is_wall || is_perimeter {
                    let entity = if is_wall {
                        let corners = edge
                            .vertices()
                            .map(|vertex| vertex.data().corner());
                        let positions = edge
                            .vertices()
                            .map(|vertex| vertex.data().position);
                        self.update_wall(
                            queries,
                            edge_data.wall,
                            edge.fix(),
                            corners,
                            positions,
                        )
                    } else {
                        let directed_edge = if edge.as_directed().is_outer_edge() {
                            edge.as_directed()
                        } else {
                            edge.as_directed().rev()
                        };
                        let positions = directed_edge
                            .vertices()
                            .map(|vertex| vertex.data().position);
                        self.update_perimeter(queries, edge_data.wall, positions)
                    };
                    self
                        .triangulation
                        .undirected_edge_data_mut(edge.fix())
                        .data_mut()
                        .wall = Some(entity);
                    if !entity.is_cloned() {
                        new_children.insert(entity.id());
                    }
                } else if edge_data.wall.is_some() {
                    self
                        .triangulation
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
                FaceHandle<
                    PossiblyOuterTag,
                    VertexData,
                    (),
                    CdtEdge<UndirectedEdgeData>,
                    FaceData,
                >,
            ),
        ) {
            if let Some(inner_face) = face.as_inner() {
                for edge in self
                    .triangulation
                    .face(inner_face)
                    .adjacent_edges()
                    .into_iter()
                    .filter(|edge| !edge.is_constraint_edge())
                {
                    f(edge.rev().face());
                }
            } else {
                for edge in self.triangulation.convex_hull() {
                    if true {
                        if !!edge.is_constraint_edge() {
                            ::core::panicking::panic(
                                "assertion failed: !edge.is_constraint_edge()",
                            )
                        }
                    }
                    if true {
                        match (&edge.face().fix(), &OUTER_FACE) {
                            (left_val, right_val) => {
                                if !(*left_val == *right_val) {
                                    let kind = ::core::panicking::AssertKind::Eq;
                                    ::core::panicking::assert_failed(
                                        kind,
                                        &*left_val,
                                        &*right_val,
                                        ::core::option::Option::None,
                                    );
                                }
                            }
                        };
                    }
                    f(edge.rev().face());
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
                    _ => {
                        queries.update(self.id, corner, Corner::bundle(vertex, position))
                    }
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
                    Some(
                        wall_data,
                    ) if wall_data.edge() == edge && wall_data.corners() == corners => {
                        wall
                    }
                    _ => {
                        queries
                            .update(
                                self.id,
                                wall,
                                Wall::bundle(edge, corners, positions),
                            )
                    }
                }
            } else {
                queries.spawn(self.id, Wall::bundle(edge, corners, positions))
            }
        }
        fn update_perimeter(
            &self,
            queries: &mut MapQueries,
            perimeter: Option<MapEntity>,
            [start,
            end,
            ]: [Vec2; 2],
        ) -> MapEntity {
            if let Some(perimeter) = perimeter {
                match queries.perimeter(perimeter.id()) {
                    Some(
                        perimeter_data,
                    ) if perimeter_data.start() == start
                        && perimeter_data.end() == end => perimeter,
                    _ => {
                        queries.update(self.id, perimeter, Perimeter::bundle(start, end))
                    }
                }
            } else {
                queries.spawn(self.id, Perimeter::bundle(start, end))
            }
        }
        fn update_room(
            &self,
            queries: &mut MapQueries,
            room: Option<MapEntity>,
            faces: &[FixedFaceHandle<PossiblyOuterTag>],
        ) -> MapEntity {
            if let Some(room) = room {
                match queries.room(room.id()) {
                    Some(room_data) if room_data.faces() == faces => room,
                    _ => queries.update(self.id, room, Room::bundle(faces.to_owned())),
                }
            } else {
                queries.spawn(self.id, Room::bundle(faces.to_owned()))
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
                .field("vertices", &self.triangulation.vertices().collect::<Vec<_>>())
                .field(
                    "edges",
                    &self.triangulation.undirected_edges().collect::<Vec<_>>(),
                )
                .field("faces", &self.triangulation.inner_faces().collect::<Vec<_>>())
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
            match self {
                MapEntity::Cloned(_) => true,
                _ => false,
            }
        }
    }
    impl MapQueries<'_, '_> {
        fn corner(&self, corner: Entity) -> Option<&Corner> {
            self.corner_q.get(corner).ok()
        }
        fn wall(&self, wall: Entity) -> Option<&Wall> {
            self.wall_q.get(wall).ok()
        }
        fn perimeter(&self, perimeter: Entity) -> Option<&Perimeter> {
            self.perimeter_q.get(perimeter).ok()
        }
        fn room(&self, room: Entity) -> Option<&Room> {
            self.room_q.get(room).ok()
        }
        fn spawn(&mut self, map: Entity, bundle: impl Bundle) -> MapEntity {
            MapEntity::Owned(self.commands.spawn((bundle, ChildOf(map))).id())
        }
        fn update(
            &mut self,
            map: Entity,
            entity: MapEntity,
            bundle: impl Bundle,
        ) -> MapEntity {
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
                                .allow::<Perimeter>()
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
}
pub mod pawn {
    pub mod ai {
        pub mod path {
            #[rustfmt::skip]
            mod model {
                #[allow(
                    unused_parens,
                    non_snake_case,
                    non_upper_case_globals,
                    clippy::let_and_return,
                    clippy::just_underscores_and_digits
                )]
                pub fn main_graph(
                    onnx__Gemm_0: [[f32; 12usize]; 1usize],
                ) -> [[f32; 6usize]; 1usize] {
                    static encoder_encoder_net_mlp_0_weight: [[f32; 12usize]; 32usize] = [
                        [
                            -1.7830737f32,
                            1.0620426f32,
                            0.46835732f32,
                            0.4472988f32,
                            -0.055962216f32,
                            0.009533292f32,
                            0.15474692f32,
                            -0.10288635f32,
                            -0.0590213f32,
                            -0.23326942f32,
                            -0.46798536f32,
                            -0.021653771f32,
                        ],
                        [
                            -0.6561509f32,
                            -0.19667605f32,
                            0.0069813775f32,
                            -0.2680614f32,
                            -0.0057294653f32,
                            0.041987598f32,
                            -11.043386f32,
                            1.568494f32,
                            -0.01673174f32,
                            0.11414716f32,
                            -0.36248133f32,
                            0.07034442f32,
                        ],
                        [
                            0.06907217f32,
                            -0.6178155f32,
                            -0.01796539f32,
                            0.740727f32,
                            -0.13898282f32,
                            -0.028832773f32,
                            -0.23583223f32,
                            0.3517642f32,
                            0.014502123f32,
                            0.06696725f32,
                            0.53787786f32,
                            0.25115454f32,
                        ],
                        [
                            -0.3877245f32,
                            0.13095415f32,
                            -0.033790562f32,
                            3.9026654f32,
                            -0.017622175f32,
                            -0.03750544f32,
                            -0.05806592f32,
                            -0.09930707f32,
                            0.005901511f32,
                            -0.27875483f32,
                            -0.45107374f32,
                            0.04249686f32,
                        ],
                        [
                            -1.3235215f32,
                            -1.1139559f32,
                            0.24286354f32,
                            -0.41054788f32,
                            0.04951321f32,
                            -0.052874103f32,
                            -0.19379818f32,
                            -0.23656403f32,
                            0.0034305584f32,
                            0.24795014f32,
                            0.73038f32,
                            -0.27054572f32,
                        ],
                        [
                            0.54510695f32,
                            -0.02065428f32,
                            -0.27665403f32,
                            -0.3608755f32,
                            -0.061608326f32,
                            0.01719081f32,
                            0.2854575f32,
                            -0.15059392f32,
                            0.028914146f32,
                            0.07936835f32,
                            -0.4836425f32,
                            0.26084363f32,
                        ],
                        [
                            -0.25942385f32,
                            0.28614512f32,
                            0.13333973f32,
                            1.9159366f32,
                            0.00055056845f32,
                            -0.020464309f32,
                            -0.02812836f32,
                            0.015514686f32,
                            0.060819704f32,
                            -0.15940505f32,
                            0.048257746f32,
                            0.033563197f32,
                        ],
                        [
                            -0.45553973f32,
                            -0.6106283f32,
                            0.057019044f32,
                            -0.26635975f32,
                            -0.055399083f32,
                            -0.0050588506f32,
                            -5.8761587f32,
                            0.93046665f32,
                            -0.030425258f32,
                            0.27047765f32,
                            -0.5648073f32,
                            0.021850169f32,
                        ],
                        [
                            -1.5541197f32,
                            -1.3231282f32,
                            -0.11892913f32,
                            0.04391614f32,
                            0.0628639f32,
                            -0.042393792f32,
                            0.107686706f32,
                            0.17059882f32,
                            -0.03712392f32,
                            0.022619188f32,
                            -0.18301803f32,
                            -0.23940845f32,
                        ],
                        [
                            -1.0074124f32,
                            -0.13514876f32,
                            -0.05401415f32,
                            -0.31751359f32,
                            -0.0014863389f32,
                            -0.000748565f32,
                            12.842709f32,
                            2.0392668f32,
                            -0.009883671f32,
                            0.28404182f32,
                            0.3766692f32,
                            0.13375959f32,
                        ],
                        [
                            -1.61482f32,
                            0.32466498f32,
                            -0.4504908f32,
                            -2.4203637f32,
                            0.058913935f32,
                            0.019769877f32,
                            0.049162947f32,
                            0.09276278f32,
                            0.12153214f32,
                            0.22489542f32,
                            -0.29383394f32,
                            0.18376148f32,
                        ],
                        [
                            -0.081112094f32,
                            -1.6035995f32,
                            -0.2653604f32,
                            -0.009175603f32,
                            -0.84744227f32,
                            0.09059779f32,
                            -1.6709007f32,
                            0.9745356f32,
                            0.11335288f32,
                            -0.09769751f32,
                            -0.3239939f32,
                            0.21037805f32,
                        ],
                        [
                            0.4270093f32,
                            0.6794052f32,
                            0.43926555f32,
                            -0.49994603f32,
                            0.02610511f32,
                            -0.23441747f32,
                            -0.55837435f32,
                            0.32943732f32,
                            0.38197035f32,
                            0.118626654f32,
                            -0.24865988f32,
                            -0.14853829f32,
                        ],
                        [
                            -1.1878424f32,
                            1.0105493f32,
                            1.6844093f32,
                            -0.7968279f32,
                            0.11387417f32,
                            0.0064f32,
                            -0.12186292f32,
                            -0.056135144f32,
                            -0.10777493f32,
                            0.189625f32,
                            -0.90805644f32,
                            -0.14045887f32,
                        ],
                        [
                            -2.3388932f32,
                            0.7931738f32,
                            -0.05429318f32,
                            2.6810184f32,
                            0.01982084f32,
                            0.038772702f32,
                            -0.09884556f32,
                            0.1455413f32,
                            0.0046237037f32,
                            0.18402463f32,
                            0.11563893f32,
                            0.1516867f32,
                        ],
                        [
                            0.26946566f32,
                            0.39944094f32,
                            -0.20511909f32,
                            13.606885f32,
                            0.07258808f32,
                            -0.076351196f32,
                            0.01508885f32,
                            -0.10224715f32,
                            -0.049798667f32,
                            -0.014530659f32,
                            -0.48775068f32,
                            0.18596736f32,
                        ],
                        [
                            -1.0694176f32,
                            0.36058912f32,
                            0.15022822f32,
                            -0.13749534f32,
                            0.0341957f32,
                            0.06870546f32,
                            3.775545f32,
                            1.0574088f32,
                            0.028759103f32,
                            0.067103565f32,
                            1.1274424f32,
                            -0.035993427f32,
                        ],
                        [
                            0.21796454f32,
                            0.14524502f32,
                            -0.26686576f32,
                            -2.3916025f32,
                            -0.8992993f32,
                            0.0043817987f32,
                            -0.21707435f32,
                            -0.014937236f32,
                            0.08562737f32,
                            0.2260046f32,
                            -0.1520107f32,
                            -0.21804133f32,
                        ],
                        [
                            -0.82263726f32,
                            -0.17654479f32,
                            -0.19096166f32,
                            -0.51506484f32,
                            0.42130718f32,
                            0.032103833f32,
                            0.3947054f32,
                            1.3333582f32,
                            -0.35933733f32,
                            0.12962997f32,
                            0.11365344f32,
                            -0.18573897f32,
                        ],
                        [
                            -2.2305684f32,
                            -1.3407298f32,
                            -0.014506384f32,
                            2.4078326f32,
                            0.012946426f32,
                            0.03581407f32,
                            -0.055589918f32,
                            0.13860452f32,
                            -0.00710958f32,
                            0.08940244f32,
                            0.10295925f32,
                            -0.049817935f32,
                        ],
                        [
                            1.0845559f32,
                            -0.8764364f32,
                            0.036069814f32,
                            0.5367387f32,
                            -0.023459034f32,
                            -0.041396663f32,
                            -0.11083593f32,
                            0.31923673f32,
                            -0.08265032f32,
                            0.24915904f32,
                            0.06511834f32,
                            0.11224747f32,
                        ],
                        [
                            0.5018998f32,
                            -0.9121758f32,
                            0.02872843f32,
                            -0.38847408f32,
                            0.03590668f32,
                            0.026378363f32,
                            0.037710816f32,
                            0.025905484f32,
                            0.0059614466f32,
                            0.2646622f32,
                            -0.47427076f32,
                            0.26833808f32,
                        ],
                        [
                            0.39956132f32,
                            -1.4434916f32,
                            0.18521446f32,
                            0.25079638f32,
                            -0.04508759f32,
                            0.03054398f32,
                            -0.85675514f32,
                            -1.1792568f32,
                            0.03382956f32,
                            -0.035189956f32,
                            -0.70695025f32,
                            -0.23351361f32,
                        ],
                        [
                            -0.8062249f32,
                            -0.5975941f32,
                            0.44331998f32,
                            -0.20085901f32,
                            -0.03723472f32,
                            -0.0600824f32,
                            -0.117467105f32,
                            -0.27328458f32,
                            -0.2694236f32,
                            -0.16806482f32,
                            -0.27850112f32,
                            -0.26486054f32,
                        ],
                        [
                            -0.597616f32,
                            -0.6508158f32,
                            -0.071189046f32,
                            0.26330692f32,
                            0.6556676f32,
                            0.31679842f32,
                            -0.009765012f32,
                            -0.32730597f32,
                            -0.16173492f32,
                            -0.20576915f32,
                            0.14554352f32,
                            0.25754797f32,
                        ],
                        [
                            -0.5362067f32,
                            0.54382396f32,
                            0.50905263f32,
                            -0.008016783f32,
                            0.72519815f32,
                            0.16819912f32,
                            0.30826485f32,
                            -0.43698823f32,
                            -0.15802276f32,
                            -0.11531119f32,
                            -0.34838223f32,
                            0.11550686f32,
                        ],
                        [
                            0.75223666f32,
                            -0.66187906f32,
                            -0.25082928f32,
                            -0.6966222f32,
                            3.421033f32,
                            0.19933325f32,
                            0.2806113f32,
                            0.4082301f32,
                            0.048076525f32,
                            0.14255199f32,
                            0.31877658f32,
                            0.17251018f32,
                        ],
                        [
                            0.29775715f32,
                            4.291141f32,
                            -0.0026032105f32,
                            -0.13196018f32,
                            0.27065405f32,
                            -0.035427473f32,
                            38.114174f32,
                            -0.4461587f32,
                            0.037024323f32,
                            -0.070497975f32,
                            0.6888892f32,
                            0.114885986f32,
                        ],
                        [
                            -0.4117187f32,
                            0.4700751f32,
                            0.20162895f32,
                            0.10203586f32,
                            -0.0051458157f32,
                            -0.009583648f32,
                            0.35749257f32,
                            -0.5313385f32,
                            0.14196257f32,
                            -0.08288598f32,
                            0.8204301f32,
                            -0.082706794f32,
                        ],
                        [
                            -0.37161654f32,
                            0.14778018f32,
                            0.23605435f32,
                            -0.80240834f32,
                            -0.16996118f32,
                            -0.017822552f32,
                            -0.049104095f32,
                            -0.06668685f32,
                            0.013341079f32,
                            0.18581557f32,
                            0.43041152f32,
                            -0.17531365f32,
                        ],
                        [
                            -0.00078604213f32,
                            -0.64405084f32,
                            1.0239296f32,
                            -0.3639129f32,
                            -0.041249458f32,
                            0.0014604611f32,
                            -0.16940619f32,
                            -0.0811325f32,
                            0.06279104f32,
                            -0.20520097f32,
                            0.3074734f32,
                            0.2621748f32,
                        ],
                        [
                            0.72439516f32,
                            -0.90053713f32,
                            -0.08127591f32,
                            -0.117490575f32,
                            -0.59011877f32,
                            -0.20870373f32,
                            -0.5538308f32,
                            -0.4596488f32,
                            0.21568067f32,
                            -0.18207964f32,
                            0.6735773f32,
                            0.27663797f32,
                        ],
                    ];
                    static encoder_encoder_net_mlp_0_bias: [f32; 32usize] = [
                        -0.5633885f32,
                        -0.33285868f32,
                        0.5263825f32,
                        -0.49054396f32,
                        0.45741013f32,
                        -0.38500777f32,
                        -0.009107336f32,
                        -0.62598187f32,
                        -0.36117363f32,
                        0.80034244f32,
                        -0.5922175f32,
                        -0.39691722f32,
                        -0.24627127f32,
                        -0.5041194f32,
                        0.16155405f32,
                        -0.25312936f32,
                        0.93164843f32,
                        -0.07527141f32,
                        0.022914104f32,
                        -0.055915467f32,
                        -0.05212832f32,
                        -0.022025064f32,
                        -0.5106956f32,
                        0.07747461f32,
                        0.2187288f32,
                        -0.2961508f32,
                        0.29822737f32,
                        0.74419534f32,
                        0.40434545f32,
                        0.21284582f32,
                        0.26365823f32,
                        0.822359f32,
                    ];
                    static pi_net_mlp_0_weight: [[f32; 32usize]; 6usize] = [
                        [
                            0.44035783f32,
                            0.13797958f32,
                            0.10070256f32,
                            0.11363656f32,
                            0.20603529f32,
                            -0.021383736f32,
                            0.28882122f32,
                            0.027116397f32,
                            -0.28102627f32,
                            0.13168539f32,
                            0.11129408f32,
                            -0.25782195f32,
                            -0.07650225f32,
                            -0.04819576f32,
                            0.19075754f32,
                            -0.007978922f32,
                            0.07012113f32,
                            -0.14722773f32,
                            -0.02909447f32,
                            0.0710866f32,
                            0.3682174f32,
                            0.17099302f32,
                            0.2541219f32,
                            -0.18075404f32,
                            0.082702614f32,
                            -0.19107875f32,
                            -0.17129405f32,
                            0.005772439f32,
                            0.218881f32,
                            -0.2054946f32,
                            -0.011320338f32,
                            0.005791234f32,
                        ],
                        [
                            -0.0620536f32,
                            -1.2002411f32,
                            -0.27047926f32,
                            -1.273131f32,
                            0.12264867f32,
                            -0.12037205f32,
                            -0.35684478f32,
                            -0.7649376f32,
                            -1.1139609f32,
                            0.71314967f32,
                            -0.11530454f32,
                            -0.716874f32,
                            -0.23660326f32,
                            0.018117303f32,
                            -0.12784526f32,
                            -0.54640204f32,
                            0.62855136f32,
                            -1.4667124f32,
                            2.0241725f32,
                            -0.47890884f32,
                            -1.1703619f32,
                            -1.0004495f32,
                            -0.42730466f32,
                            -0.5876573f32,
                            1.6032531f32,
                            2.054403f32,
                            1.3050922f32,
                            1.3216236f32,
                            0.59705365f32,
                            -0.13532208f32,
                            -1.0323482f32,
                            -2.1208339f32,
                        ],
                        [
                            -0.38505238f32,
                            -0.32829544f32,
                            0.9215876f32,
                            1.094383f32,
                            -0.80109274f32,
                            -0.20624042f32,
                            0.95474774f32,
                            -0.1556889f32,
                            0.35832217f32,
                            -0.006825462f32,
                            -1.2781938f32,
                            -0.39864212f32,
                            0.15986903f32,
                            -0.12900588f32,
                            0.54093033f32,
                            3.8342314f32,
                            0.18945491f32,
                            1.1062632f32,
                            0.16312942f32,
                            0.63709384f32,
                            0.5182108f32,
                            0.3787653f32,
                            -0.14629619f32,
                            -0.32701388f32,
                            0.0741231f32,
                            -0.036845323f32,
                            0.31419086f32,
                            0.38947272f32,
                            0.2027213f32,
                            -0.19366488f32,
                            0.11814575f32,
                            0.16878828f32,
                        ],
                        [
                            0.21349566f32,
                            -0.50004756f32,
                            -0.34628314f32,
                            -0.38987327f32,
                            -1.0612898f32,
                            0.10175939f32,
                            -0.17595257f32,
                            0.099444754f32,
                            0.2791318f32,
                            0.36399874f32,
                            -0.6118101f32,
                            0.61379945f32,
                            0.2827856f32,
                            0.4356086f32,
                            -0.44865373f32,
                            -0.15378056f32,
                            -0.56329197f32,
                            -0.1639879f32,
                            -0.19857107f32,
                            0.6596732f32,
                            -0.142367f32,
                            0.18572691f32,
                            0.97838736f32,
                            -0.021098336f32,
                            0.1355323f32,
                            -0.095052116f32,
                            -0.14055763f32,
                            -0.032506622f32,
                            -0.05362499f32,
                            -0.055194065f32,
                            -0.6918991f32,
                            -0.6418383f32,
                        ],
                        [
                            0.23266655f32,
                            -0.5409028f32,
                            0.12534642f32,
                            -0.553755f32,
                            -0.4674389f32,
                            0.06893007f32,
                            0.13903414f32,
                            -0.13131416f32,
                            0.011330491f32,
                            0.2074187f32,
                            -0.7162522f32,
                            0.50399685f32,
                            -0.11772661f32,
                            0.23615201f32,
                            -0.20852517f32,
                            -0.2377459f32,
                            -0.16305469f32,
                            -0.26067856f32,
                            0.122285955f32,
                            0.012692794f32,
                            -0.2807731f32,
                            -0.21363464f32,
                            0.3038527f32,
                            0.015367611f32,
                            0.6674715f32,
                            -0.045779094f32,
                            0.7280477f32,
                            0.11161794f32,
                            0.19748263f32,
                            -0.0303373f32,
                            -0.60232496f32,
                            -0.20295331f32,
                        ],
                        [
                            1.0196524f32,
                            -0.12094987f32,
                            -0.023638446f32,
                            -1.1190002f32,
                            0.6084028f32,
                            -0.651812f32,
                            0.44529566f32,
                            -0.3391059f32,
                            -0.3039196f32,
                            -0.73723316f32,
                            -1.4314309f32,
                            0.19131738f32,
                            1.0219479f32,
                            2.7308893f32,
                            -0.24403955f32,
                            -0.2181709f32,
                            0.21534935f32,
                            -0.24974932f32,
                            -0.35101926f32,
                            0.36415946f32,
                            -0.1632504f32,
                            -0.013121497f32,
                            0.249832f32,
                            0.102022395f32,
                            0.8633092f32,
                            0.3548762f32,
                            0.19080342f32,
                            0.5729004f32,
                            0.2953701f32,
                            0.44719887f32,
                            0.7693497f32,
                            -0.67040217f32,
                        ],
                    ];
                    static pi_net_mlp_0_bias: [f32; 6usize] = [
                        -0.10699861f32,
                        0.3262202f32,
                        0.068587355f32,
                        -0.19249648f32,
                        0.027465288f32,
                        0.4647454f32,
                    ];
                    const onnx__Max_34: [f32; 1usize] = [-20f32];
                    const onnx__Min_35: [f32; 1usize] = [20f32];
                    let (_encoder_encoder_net_mlp_mlp_0_Gemm_output_0) = {
                        let mut c = [
                            [
                                encoder_encoder_net_mlp_0_bias[0usize],
                                encoder_encoder_net_mlp_0_bias[1usize],
                                encoder_encoder_net_mlp_0_bias[2usize],
                                encoder_encoder_net_mlp_0_bias[3usize],
                                encoder_encoder_net_mlp_0_bias[4usize],
                                encoder_encoder_net_mlp_0_bias[5usize],
                                encoder_encoder_net_mlp_0_bias[6usize],
                                encoder_encoder_net_mlp_0_bias[7usize],
                                encoder_encoder_net_mlp_0_bias[8usize],
                                encoder_encoder_net_mlp_0_bias[9usize],
                                encoder_encoder_net_mlp_0_bias[10usize],
                                encoder_encoder_net_mlp_0_bias[11usize],
                                encoder_encoder_net_mlp_0_bias[12usize],
                                encoder_encoder_net_mlp_0_bias[13usize],
                                encoder_encoder_net_mlp_0_bias[14usize],
                                encoder_encoder_net_mlp_0_bias[15usize],
                                encoder_encoder_net_mlp_0_bias[16usize],
                                encoder_encoder_net_mlp_0_bias[17usize],
                                encoder_encoder_net_mlp_0_bias[18usize],
                                encoder_encoder_net_mlp_0_bias[19usize],
                                encoder_encoder_net_mlp_0_bias[20usize],
                                encoder_encoder_net_mlp_0_bias[21usize],
                                encoder_encoder_net_mlp_0_bias[22usize],
                                encoder_encoder_net_mlp_0_bias[23usize],
                                encoder_encoder_net_mlp_0_bias[24usize],
                                encoder_encoder_net_mlp_0_bias[25usize],
                                encoder_encoder_net_mlp_0_bias[26usize],
                                encoder_encoder_net_mlp_0_bias[27usize],
                                encoder_encoder_net_mlp_0_bias[28usize],
                                encoder_encoder_net_mlp_0_bias[29usize],
                                encoder_encoder_net_mlp_0_bias[30usize],
                                encoder_encoder_net_mlp_0_bias[31usize],
                            ],
                        ];
                        unsafe {
                            ::matrixmultiply::sgemm(
                                1usize,
                                12usize,
                                32usize,
                                1f32,
                                onnx__Gemm_0.as_flattened().as_ptr(),
                                12isize,
                                1isize,
                                encoder_encoder_net_mlp_0_weight.as_flattened().as_ptr(),
                                1isize,
                                12isize,
                                1f32,
                                c.as_flattened_mut().as_mut_ptr(),
                                1isize,
                                1isize,
                            );
                        }
                        c
                    };
                    let (_encoder_encoder_net_mlp_mlp_1_Tanh_output_0) = [
                        [
                            _encoder_encoder_net_mlp_mlp_0_Gemm_output_0[0usize][0usize]
                                .tanh(),
                            _encoder_encoder_net_mlp_mlp_0_Gemm_output_0[0usize][1usize]
                                .tanh(),
                            _encoder_encoder_net_mlp_mlp_0_Gemm_output_0[0usize][2usize]
                                .tanh(),
                            _encoder_encoder_net_mlp_mlp_0_Gemm_output_0[0usize][3usize]
                                .tanh(),
                            _encoder_encoder_net_mlp_mlp_0_Gemm_output_0[0usize][4usize]
                                .tanh(),
                            _encoder_encoder_net_mlp_mlp_0_Gemm_output_0[0usize][5usize]
                                .tanh(),
                            _encoder_encoder_net_mlp_mlp_0_Gemm_output_0[0usize][6usize]
                                .tanh(),
                            _encoder_encoder_net_mlp_mlp_0_Gemm_output_0[0usize][7usize]
                                .tanh(),
                            _encoder_encoder_net_mlp_mlp_0_Gemm_output_0[0usize][8usize]
                                .tanh(),
                            _encoder_encoder_net_mlp_mlp_0_Gemm_output_0[0usize][9usize]
                                .tanh(),
                            _encoder_encoder_net_mlp_mlp_0_Gemm_output_0[0usize][10usize]
                                .tanh(),
                            _encoder_encoder_net_mlp_mlp_0_Gemm_output_0[0usize][11usize]
                                .tanh(),
                            _encoder_encoder_net_mlp_mlp_0_Gemm_output_0[0usize][12usize]
                                .tanh(),
                            _encoder_encoder_net_mlp_mlp_0_Gemm_output_0[0usize][13usize]
                                .tanh(),
                            _encoder_encoder_net_mlp_mlp_0_Gemm_output_0[0usize][14usize]
                                .tanh(),
                            _encoder_encoder_net_mlp_mlp_0_Gemm_output_0[0usize][15usize]
                                .tanh(),
                            _encoder_encoder_net_mlp_mlp_0_Gemm_output_0[0usize][16usize]
                                .tanh(),
                            _encoder_encoder_net_mlp_mlp_0_Gemm_output_0[0usize][17usize]
                                .tanh(),
                            _encoder_encoder_net_mlp_mlp_0_Gemm_output_0[0usize][18usize]
                                .tanh(),
                            _encoder_encoder_net_mlp_mlp_0_Gemm_output_0[0usize][19usize]
                                .tanh(),
                            _encoder_encoder_net_mlp_mlp_0_Gemm_output_0[0usize][20usize]
                                .tanh(),
                            _encoder_encoder_net_mlp_mlp_0_Gemm_output_0[0usize][21usize]
                                .tanh(),
                            _encoder_encoder_net_mlp_mlp_0_Gemm_output_0[0usize][22usize]
                                .tanh(),
                            _encoder_encoder_net_mlp_mlp_0_Gemm_output_0[0usize][23usize]
                                .tanh(),
                            _encoder_encoder_net_mlp_mlp_0_Gemm_output_0[0usize][24usize]
                                .tanh(),
                            _encoder_encoder_net_mlp_mlp_0_Gemm_output_0[0usize][25usize]
                                .tanh(),
                            _encoder_encoder_net_mlp_mlp_0_Gemm_output_0[0usize][26usize]
                                .tanh(),
                            _encoder_encoder_net_mlp_mlp_0_Gemm_output_0[0usize][27usize]
                                .tanh(),
                            _encoder_encoder_net_mlp_mlp_0_Gemm_output_0[0usize][28usize]
                                .tanh(),
                            _encoder_encoder_net_mlp_mlp_0_Gemm_output_0[0usize][29usize]
                                .tanh(),
                            _encoder_encoder_net_mlp_mlp_0_Gemm_output_0[0usize][30usize]
                                .tanh(),
                            _encoder_encoder_net_mlp_mlp_0_Gemm_output_0[0usize][31usize]
                                .tanh(),
                        ],
                    ];
                    let (_pi_net_mlp_mlp_0_Gemm_output_0) = {
                        let mut c = [
                            [
                                pi_net_mlp_0_bias[0usize],
                                pi_net_mlp_0_bias[1usize],
                                pi_net_mlp_0_bias[2usize],
                                pi_net_mlp_0_bias[3usize],
                                pi_net_mlp_0_bias[4usize],
                                pi_net_mlp_0_bias[5usize],
                            ],
                        ];
                        unsafe {
                            ::matrixmultiply::sgemm(
                                1usize,
                                32usize,
                                6usize,
                                1f32,
                                _encoder_encoder_net_mlp_mlp_1_Tanh_output_0
                                    .as_flattened()
                                    .as_ptr(),
                                32isize,
                                1isize,
                                pi_net_mlp_0_weight.as_flattened().as_ptr(),
                                1isize,
                                32isize,
                                1f32,
                                c.as_flattened_mut().as_mut_ptr(),
                                1isize,
                                1isize,
                            );
                        }
                        c
                    };
                    const _pi_Constant_output_0: [i64; 1usize] = [-1i64];
                    const _pi_Constant_1_output_0: [i64; 1usize] = [0i64];
                    const _pi_Mul_output_0: [i64; 1usize] = [3i64];
                    let (_pi_Slice_output_0) = [
                        [
                            _pi_net_mlp_mlp_0_Gemm_output_0[0usize][0usize],
                            _pi_net_mlp_mlp_0_Gemm_output_0[0usize][1usize],
                            _pi_net_mlp_mlp_0_Gemm_output_0[0usize][2usize],
                        ],
                    ];
                    const _pi_Mul_1_output_0: [i64; 1usize] = [6i64];
                    let (_pi_Slice_1_output_0) = [
                        [
                            _pi_net_mlp_mlp_0_Gemm_output_0[0usize][3usize],
                            _pi_net_mlp_mlp_0_Gemm_output_0[0usize][4usize],
                            _pi_net_mlp_mlp_0_Gemm_output_0[0usize][5usize],
                        ],
                    ];
                    let (_pi_Max_output_0) = [
                        [
                            _pi_Slice_1_output_0[0usize][0usize]
                                .max(onnx__Max_34[0usize]),
                            _pi_Slice_1_output_0[0usize][1usize]
                                .max(onnx__Max_34[0usize]),
                            _pi_Slice_1_output_0[0usize][2usize]
                                .max(onnx__Max_34[0usize]),
                        ],
                    ];
                    let (_pi_Min_output_0) = [
                        [
                            _pi_Max_output_0[0usize][0usize].min(onnx__Min_35[0usize]),
                            _pi_Max_output_0[0usize][1usize].min(onnx__Min_35[0usize]),
                            _pi_Max_output_0[0usize][2usize].min(onnx__Min_35[0usize]),
                        ],
                    ];
                    let (_30) = [
                        [
                            _pi_Slice_output_0[0usize][0usize],
                            _pi_Slice_output_0[0usize][1usize],
                            _pi_Slice_output_0[0usize][2usize],
                            _pi_Min_output_0[0usize][0usize],
                            _pi_Min_output_0[0usize][1usize],
                            _pi_Min_output_0[0usize][2usize],
                        ],
                    ];
                    _30
                }
            }
            use std::{collections::VecDeque, f32::consts::PI};
            use avian2d::{collision::collider::contact_query, prelude::*};
            use bevy::{
                ecs::{
                    query::QueryEntityError, relationship::Relationship,
                    system::SystemParam,
                },
                prelude::*,
            };
            use tokio::sync::oneshot;
            use crate::{
                layer::Layer, map::{mesh::MapMesh, room::ContainingRoom, wall::Wall},
                pawn::{Pawn, ai::Task},
            };
            const POSITION_EPSILON: f32 = Pawn::MAX_VELOCITY / 64.;
            pub struct PathTaskBundle {
                task: Task,
                path: PathTask,
            }
            #[allow(deprecated)]
            unsafe impl bevy::ecs::bundle::Bundle for PathTaskBundle {
                fn component_ids(
                    components: &mut bevy::ecs::component::ComponentsRegistrator,
                    ids: &mut impl FnMut(bevy::ecs::component::ComponentId),
                ) {
                    <Task as bevy::ecs::bundle::Bundle>::component_ids(
                        components,
                        &mut *ids,
                    );
                    <PathTask as bevy::ecs::bundle::Bundle>::component_ids(
                        components,
                        &mut *ids,
                    );
                }
                fn get_component_ids(
                    components: &bevy::ecs::component::Components,
                    ids: &mut impl FnMut(Option<bevy::ecs::component::ComponentId>),
                ) {
                    <Task as bevy::ecs::bundle::Bundle>::get_component_ids(
                        components,
                        &mut *ids,
                    );
                    <PathTask as bevy::ecs::bundle::Bundle>::get_component_ids(
                        components,
                        &mut *ids,
                    );
                }
                fn register_required_components(
                    components: &mut bevy::ecs::component::ComponentsRegistrator,
                    required_components: &mut bevy::ecs::component::RequiredComponents,
                ) {
                    <Task as bevy::ecs::bundle::Bundle>::register_required_components(
                        components,
                        required_components,
                    );
                    <PathTask as bevy::ecs::bundle::Bundle>::register_required_components(
                        components,
                        required_components,
                    );
                }
            }
            #[allow(deprecated)]
            unsafe impl bevy::ecs::bundle::BundleFromComponents for PathTaskBundle {
                #[allow(unused_variables, non_snake_case)]
                unsafe fn from_components<__T, __F>(
                    ctx: &mut __T,
                    func: &mut __F,
                ) -> Self
                where
                    __F: FnMut(&mut __T) -> bevy::ecs::ptr::OwningPtr<'_>,
                {
                    Self {
                        task: <Task as bevy::ecs::bundle::BundleFromComponents>::from_components(
                            ctx,
                            &mut *func,
                        ),
                        path: <PathTask as bevy::ecs::bundle::BundleFromComponents>::from_components(
                            ctx,
                            &mut *func,
                        ),
                    }
                }
            }
            #[allow(deprecated)]
            impl bevy::ecs::bundle::DynamicBundle for PathTaskBundle {
                type Effect = ();
                #[allow(unused_variables)]
                #[inline]
                fn get_components(
                    self,
                    func: &mut impl FnMut(
                        bevy::ecs::component::StorageType,
                        bevy::ecs::ptr::OwningPtr<'_>,
                    ),
                ) {
                    self.task.get_components(&mut *func);
                    self.path.get_components(&mut *func);
                }
            }
            pub enum PathTask {
                Pending(oneshot::Receiver<Option<VecDeque<Vec2>>>),
                Running(VecDeque<Vec2>),
            }
            #[automatically_derived]
            impl ::core::fmt::Debug for PathTask {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    match self {
                        PathTask::Pending(__self_0) => {
                            ::core::fmt::Formatter::debug_tuple_field1_finish(
                                f,
                                "Pending",
                                &__self_0,
                            )
                        }
                        PathTask::Running(__self_0) => {
                            ::core::fmt::Formatter::debug_tuple_field1_finish(
                                f,
                                "Running",
                                &__self_0,
                            )
                        }
                    }
                }
            }
            impl bevy::ecs::component::Component for PathTask
            where
                Self: Send + Sync + 'static,
            {
                const STORAGE_TYPE: bevy::ecs::component::StorageType = bevy::ecs::component::StorageType::Table;
                type Mutability = bevy::ecs::component::Mutable;
                fn register_required_components(
                    requiree: bevy::ecs::component::ComponentId,
                    components: &mut bevy::ecs::component::ComponentsRegistrator,
                    required_components: &mut bevy::ecs::component::RequiredComponents,
                    inheritance_depth: u16,
                    recursion_check_stack: &mut bevy::ecs::__macro_exports::Vec<
                        bevy::ecs::component::ComponentId,
                    >,
                ) {
                    bevy::ecs::component::enforce_no_required_components_recursion(
                        components,
                        recursion_check_stack,
                    );
                    let self_id = components.register_component::<Self>();
                    recursion_check_stack.push(self_id);
                    recursion_check_stack.pop();
                }
                fn clone_behavior() -> bevy::ecs::component::ComponentCloneBehavior {
                    use bevy::ecs::component::{
                        DefaultCloneBehaviorBase, DefaultCloneBehaviorViaClone,
                    };
                    (&&&bevy::ecs::component::DefaultCloneBehaviorSpecialization::<
                        Self,
                    >::default())
                        .default_clone_behavior()
                }
                fn map_entities<M: bevy::ecs::entity::EntityMapper>(
                    this: &mut Self,
                    mapper: &mut M,
                ) {
                    use bevy::ecs::entity::MapEntities;
                    match this {
                        Self::Pending { .. } => {}
                        Self::Running { .. } => {}
                        _ => {}
                    }
                }
            }
            pub struct MovementQuery<'w, 's> {
                spatial_query: SpatialQuery<'w, 's>,
                pawn_q: Query<
                    'w,
                    's,
                    (
                        &'static mut Pawn,
                        &'static Position,
                        &'static Rotation,
                        &'static Collider,
                        &'static LinearVelocity,
                        &'static AngularVelocity,
                    ),
                    With<Pawn>,
                >,
                collider_q: Query<
                    'w,
                    's,
                    (
                        &'static Position,
                        &'static Rotation,
                        &'static Collider,
                        &'static LinearVelocity,
                        Has<Wall>,
                        Has<Pawn>,
                    ),
                >,
                config: Res<'w, PathQueryConfig>,
            }
            const _: () = {
                type __StructFieldsAlias<'w, 's> = (
                    SpatialQuery<'w, 's>,
                    Query<
                        'w,
                        's,
                        (
                            &'static mut Pawn,
                            &'static Position,
                            &'static Rotation,
                            &'static Collider,
                            &'static LinearVelocity,
                            &'static AngularVelocity,
                        ),
                        With<Pawn>,
                    >,
                    Query<
                        'w,
                        's,
                        (
                            &'static Position,
                            &'static Rotation,
                            &'static Collider,
                            &'static LinearVelocity,
                            Has<Wall>,
                            Has<Pawn>,
                        ),
                    >,
                    Res<'w, PathQueryConfig>,
                );
                #[doc(hidden)]
                pub struct FetchState {
                    state: <__StructFieldsAlias<
                        'static,
                        'static,
                    > as bevy::ecs::system::SystemParam>::State,
                }
                unsafe impl bevy::ecs::system::SystemParam for MovementQuery<'_, '_> {
                    type State = FetchState;
                    type Item<'w, 's> = MovementQuery<'w, 's>;
                    fn init_state(
                        world: &mut bevy::ecs::world::World,
                        system_meta: &mut bevy::ecs::system::SystemMeta,
                    ) -> Self::State {
                        FetchState {
                            state: <__StructFieldsAlias<
                                '_,
                                '_,
                            > as bevy::ecs::system::SystemParam>::init_state(
                                world,
                                system_meta,
                            ),
                        }
                    }
                    unsafe fn new_archetype(
                        state: &mut Self::State,
                        archetype: &bevy::ecs::archetype::Archetype,
                        system_meta: &mut bevy::ecs::system::SystemMeta,
                    ) {
                        unsafe {
                            <__StructFieldsAlias<
                                '_,
                                '_,
                            > as bevy::ecs::system::SystemParam>::new_archetype(
                                &mut state.state,
                                archetype,
                                system_meta,
                            )
                        }
                    }
                    fn apply(
                        state: &mut Self::State,
                        system_meta: &bevy::ecs::system::SystemMeta,
                        world: &mut bevy::ecs::world::World,
                    ) {
                        <__StructFieldsAlias<
                            '_,
                            '_,
                        > as bevy::ecs::system::SystemParam>::apply(
                            &mut state.state,
                            system_meta,
                            world,
                        );
                    }
                    fn queue(
                        state: &mut Self::State,
                        system_meta: &bevy::ecs::system::SystemMeta,
                        world: bevy::ecs::world::DeferredWorld,
                    ) {
                        <__StructFieldsAlias<
                            '_,
                            '_,
                        > as bevy::ecs::system::SystemParam>::queue(
                            &mut state.state,
                            system_meta,
                            world,
                        );
                    }
                    #[inline]
                    unsafe fn validate_param<'w, 's>(
                        state: &'s Self::State,
                        _system_meta: &bevy::ecs::system::SystemMeta,
                        _world: bevy::ecs::world::unsafe_world_cell::UnsafeWorldCell<'w>,
                    ) -> Result<(), bevy::ecs::system::SystemParamValidationError> {
                        let FetchState { state: (f0, f1, f2, f3) } = state;
                        <SpatialQuery<
                            'w,
                            's,
                        > as bevy::ecs::system::SystemParam>::validate_param(
                                f0,
                                _system_meta,
                                _world,
                            )
                            .map_err(|err| bevy::ecs::system::SystemParamValidationError::new::<
                                Self,
                            >(err.skipped, err.message, "::spatial_query"))?;
                        <Query<
                            'w,
                            's,
                            (
                                &'static mut Pawn,
                                &'static Position,
                                &'static Rotation,
                                &'static Collider,
                                &'static LinearVelocity,
                                &'static AngularVelocity,
                            ),
                            With<Pawn>,
                        > as bevy::ecs::system::SystemParam>::validate_param(
                                f1,
                                _system_meta,
                                _world,
                            )
                            .map_err(|err| bevy::ecs::system::SystemParamValidationError::new::<
                                Self,
                            >(err.skipped, err.message, "::pawn_q"))?;
                        <Query<
                            'w,
                            's,
                            (
                                &'static Position,
                                &'static Rotation,
                                &'static Collider,
                                &'static LinearVelocity,
                                Has<Wall>,
                                Has<Pawn>,
                            ),
                        > as bevy::ecs::system::SystemParam>::validate_param(
                                f2,
                                _system_meta,
                                _world,
                            )
                            .map_err(|err| bevy::ecs::system::SystemParamValidationError::new::<
                                Self,
                            >(err.skipped, err.message, "::collider_q"))?;
                        <Res<
                            'w,
                            PathQueryConfig,
                        > as bevy::ecs::system::SystemParam>::validate_param(
                                f3,
                                _system_meta,
                                _world,
                            )
                            .map_err(|err| bevy::ecs::system::SystemParamValidationError::new::<
                                Self,
                            >(err.skipped, err.message, "::config"))?;
                        Result::Ok(())
                    }
                    #[inline]
                    unsafe fn get_param<'w, 's>(
                        state: &'s mut Self::State,
                        system_meta: &bevy::ecs::system::SystemMeta,
                        world: bevy::ecs::world::unsafe_world_cell::UnsafeWorldCell<'w>,
                        change_tick: bevy::ecs::component::Tick,
                    ) -> Self::Item<'w, 's> {
                        let (f0, f1, f2, f3) = <(
                            SpatialQuery<'w, 's>,
                            Query<
                                'w,
                                's,
                                (
                                    &'static mut Pawn,
                                    &'static Position,
                                    &'static Rotation,
                                    &'static Collider,
                                    &'static LinearVelocity,
                                    &'static AngularVelocity,
                                ),
                                With<Pawn>,
                            >,
                            Query<
                                'w,
                                's,
                                (
                                    &'static Position,
                                    &'static Rotation,
                                    &'static Collider,
                                    &'static LinearVelocity,
                                    Has<Wall>,
                                    Has<Pawn>,
                                ),
                            >,
                            Res<'w, PathQueryConfig>,
                        ) as bevy::ecs::system::SystemParam>::get_param(
                            &mut state.state,
                            system_meta,
                            world,
                            change_tick,
                        );
                        MovementQuery {
                            spatial_query: f0,
                            pawn_q: f1,
                            collider_q: f2,
                            config: f3,
                        }
                    }
                }
                unsafe impl<'w, 's> bevy::ecs::system::ReadOnlySystemParam
                for MovementQuery<'w, 's>
                where
                    SpatialQuery<'w, 's>: bevy::ecs::system::ReadOnlySystemParam,
                    Query<
                        'w,
                        's,
                        (
                            &'static mut Pawn,
                            &'static Position,
                            &'static Rotation,
                            &'static Collider,
                            &'static LinearVelocity,
                            &'static AngularVelocity,
                        ),
                        With<Pawn>,
                    >: bevy::ecs::system::ReadOnlySystemParam,
                    Query<
                        'w,
                        's,
                        (
                            &'static Position,
                            &'static Rotation,
                            &'static Collider,
                            &'static LinearVelocity,
                            Has<Wall>,
                            Has<Pawn>,
                        ),
                    >: bevy::ecs::system::ReadOnlySystemParam,
                    Res<'w, PathQueryConfig>: bevy::ecs::system::ReadOnlySystemParam,
                {}
            };
            pub struct PathQuery<'w, 's> {
                pawn_q: Query<'w, 's, (&'static Position, &'static ContainingRoom)>,
                parent_q: Query<'w, 's, &'static ChildOf>,
                mesh_q: Query<'w, 's, &'static MapMesh>,
            }
            const _: () = {
                type __StructFieldsAlias<'w, 's> = (
                    Query<'w, 's, (&'static Position, &'static ContainingRoom)>,
                    Query<'w, 's, &'static ChildOf>,
                    Query<'w, 's, &'static MapMesh>,
                );
                #[doc(hidden)]
                pub struct FetchState {
                    state: <__StructFieldsAlias<
                        'static,
                        'static,
                    > as bevy::ecs::system::SystemParam>::State,
                }
                unsafe impl bevy::ecs::system::SystemParam for PathQuery<'_, '_> {
                    type State = FetchState;
                    type Item<'w, 's> = PathQuery<'w, 's>;
                    fn init_state(
                        world: &mut bevy::ecs::world::World,
                        system_meta: &mut bevy::ecs::system::SystemMeta,
                    ) -> Self::State {
                        FetchState {
                            state: <__StructFieldsAlias<
                                '_,
                                '_,
                            > as bevy::ecs::system::SystemParam>::init_state(
                                world,
                                system_meta,
                            ),
                        }
                    }
                    unsafe fn new_archetype(
                        state: &mut Self::State,
                        archetype: &bevy::ecs::archetype::Archetype,
                        system_meta: &mut bevy::ecs::system::SystemMeta,
                    ) {
                        unsafe {
                            <__StructFieldsAlias<
                                '_,
                                '_,
                            > as bevy::ecs::system::SystemParam>::new_archetype(
                                &mut state.state,
                                archetype,
                                system_meta,
                            )
                        }
                    }
                    fn apply(
                        state: &mut Self::State,
                        system_meta: &bevy::ecs::system::SystemMeta,
                        world: &mut bevy::ecs::world::World,
                    ) {
                        <__StructFieldsAlias<
                            '_,
                            '_,
                        > as bevy::ecs::system::SystemParam>::apply(
                            &mut state.state,
                            system_meta,
                            world,
                        );
                    }
                    fn queue(
                        state: &mut Self::State,
                        system_meta: &bevy::ecs::system::SystemMeta,
                        world: bevy::ecs::world::DeferredWorld,
                    ) {
                        <__StructFieldsAlias<
                            '_,
                            '_,
                        > as bevy::ecs::system::SystemParam>::queue(
                            &mut state.state,
                            system_meta,
                            world,
                        );
                    }
                    #[inline]
                    unsafe fn validate_param<'w, 's>(
                        state: &'s Self::State,
                        _system_meta: &bevy::ecs::system::SystemMeta,
                        _world: bevy::ecs::world::unsafe_world_cell::UnsafeWorldCell<'w>,
                    ) -> Result<(), bevy::ecs::system::SystemParamValidationError> {
                        let FetchState { state: (f0, f1, f2) } = state;
                        <Query<
                            'w,
                            's,
                            (&'static Position, &'static ContainingRoom),
                        > as bevy::ecs::system::SystemParam>::validate_param(
                                f0,
                                _system_meta,
                                _world,
                            )
                            .map_err(|err| bevy::ecs::system::SystemParamValidationError::new::<
                                Self,
                            >(err.skipped, err.message, "::pawn_q"))?;
                        <Query<
                            'w,
                            's,
                            &'static ChildOf,
                        > as bevy::ecs::system::SystemParam>::validate_param(
                                f1,
                                _system_meta,
                                _world,
                            )
                            .map_err(|err| bevy::ecs::system::SystemParamValidationError::new::<
                                Self,
                            >(err.skipped, err.message, "::parent_q"))?;
                        <Query<
                            'w,
                            's,
                            &'static MapMesh,
                        > as bevy::ecs::system::SystemParam>::validate_param(
                                f2,
                                _system_meta,
                                _world,
                            )
                            .map_err(|err| bevy::ecs::system::SystemParamValidationError::new::<
                                Self,
                            >(err.skipped, err.message, "::mesh_q"))?;
                        Result::Ok(())
                    }
                    #[inline]
                    unsafe fn get_param<'w, 's>(
                        state: &'s mut Self::State,
                        system_meta: &bevy::ecs::system::SystemMeta,
                        world: bevy::ecs::world::unsafe_world_cell::UnsafeWorldCell<'w>,
                        change_tick: bevy::ecs::component::Tick,
                    ) -> Self::Item<'w, 's> {
                        let (f0, f1, f2) = <(
                            Query<'w, 's, (&'static Position, &'static ContainingRoom)>,
                            Query<'w, 's, &'static ChildOf>,
                            Query<'w, 's, &'static MapMesh>,
                        ) as bevy::ecs::system::SystemParam>::get_param(
                            &mut state.state,
                            system_meta,
                            world,
                            change_tick,
                        );
                        PathQuery {
                            pawn_q: f0,
                            parent_q: f1,
                            mesh_q: f2,
                        }
                    }
                }
                unsafe impl<'w, 's> bevy::ecs::system::ReadOnlySystemParam
                for PathQuery<'w, 's>
                where
                    Query<
                        'w,
                        's,
                        (&'static Position, &'static ContainingRoom),
                    >: bevy::ecs::system::ReadOnlySystemParam,
                    Query<
                        'w,
                        's,
                        &'static ChildOf,
                    >: bevy::ecs::system::ReadOnlySystemParam,
                    Query<
                        'w,
                        's,
                        &'static MapMesh,
                    >: bevy::ecs::system::ReadOnlySystemParam,
                {}
            };
            pub struct PathQueryConfig {
                collider: Collider,
                all_filter: SpatialQueryFilter,
                wall_filter: SpatialQueryFilter,
            }
            impl bevy::ecs::resource::Resource for PathQueryConfig
            where
                Self: Send + Sync + 'static,
            {}
            pub struct PathObservation {
                pub linear_velocity_t: f32,
                pub linear_velocity_r: f32,
                pub angular_velocity: f32,
                pub target_t: f32,
                pub target_r: f32,
                pub collision_t: f32,
                pub collision_r: f32,
                pub collision_normal_t: f32,
                pub collision_is_wall: f32,
                pub collision_is_pawn: f32,
                pub collision_velocity_t: f32,
                pub collision_velocity_r: f32,
            }
            #[automatically_derived]
            impl ::core::fmt::Debug for PathObservation {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    let names: &'static _ = &[
                        "linear_velocity_t",
                        "linear_velocity_r",
                        "angular_velocity",
                        "target_t",
                        "target_r",
                        "collision_t",
                        "collision_r",
                        "collision_normal_t",
                        "collision_is_wall",
                        "collision_is_pawn",
                        "collision_velocity_t",
                        "collision_velocity_r",
                    ];
                    let values: &[&dyn ::core::fmt::Debug] = &[
                        &self.linear_velocity_t,
                        &self.linear_velocity_r,
                        &self.angular_velocity,
                        &self.target_t,
                        &self.target_r,
                        &self.collision_t,
                        &self.collision_r,
                        &self.collision_normal_t,
                        &self.collision_is_wall,
                        &self.collision_is_pawn,
                        &self.collision_velocity_t,
                        &&self.collision_velocity_r,
                    ];
                    ::core::fmt::Formatter::debug_struct_fields_finish(
                        f,
                        "PathObservation",
                        names,
                        values,
                    )
                }
            }
            impl PathTaskBundle {
                pub fn move_to(actor: Entity, to: Vec2) -> Self {
                    PathTaskBundle {
                        task: Task::new(actor),
                        path: PathTask::Running(VecDeque::from_iter([to])),
                    }
                }
            }
            pub fn update(
                mut commands: Commands,
                mut task_q: Query<(Entity, &Task, &mut PathTask)>,
                mut path_q: MovementQuery,
            ) -> Result {
                for (id, task, mut path) in &mut task_q {
                    let Some(steps) = path.poll() else {
                        return Ok(());
                    };
                    if steps.is_empty() {
                        {
                            use ::tracing::__macro_support::Callsite as _;
                            static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                static META: ::tracing::Metadata<'static> = {
                                    ::tracing_core::metadata::Metadata::new(
                                        "event pb-engine\\src\\pawn\\ai\\path\\mod.rs:114",
                                        "pb_engine::pawn::ai::path",
                                        ::tracing::Level::INFO,
                                        ::tracing_core::__macro_support::Option::Some(
                                            "pb-engine\\src\\pawn\\ai\\path\\mod.rs",
                                        ),
                                        ::tracing_core::__macro_support::Option::Some(114u32),
                                        ::tracing_core::__macro_support::Option::Some(
                                            "pb_engine::pawn::ai::path",
                                        ),
                                        ::tracing_core::field::FieldSet::new(
                                            &["message"],
                                            ::tracing_core::callsite::Identifier(&__CALLSITE),
                                        ),
                                        ::tracing::metadata::Kind::EVENT,
                                    )
                                };
                                ::tracing::callsite::DefaultCallsite::new(&META)
                            };
                            let enabled = ::tracing::Level::INFO
                                <= ::tracing::level_filters::STATIC_MAX_LEVEL
                                && ::tracing::Level::INFO
                                    <= ::tracing::level_filters::LevelFilter::current()
                                && {
                                    let interest = __CALLSITE.interest();
                                    !interest.is_never()
                                        && ::tracing::__macro_support::__is_enabled(
                                            __CALLSITE.metadata(),
                                            interest,
                                        )
                                };
                            if enabled {
                                (|value_set: ::tracing::field::ValueSet| {
                                    let meta = __CALLSITE.metadata();
                                    ::tracing::Event::dispatch(meta, &value_set);
                                })({
                                    #[allow(unused_imports)]
                                    use ::tracing::field::{debug, display, Value};
                                    let mut iter = __CALLSITE.metadata().fields().iter();
                                    __CALLSITE
                                        .metadata()
                                        .fields()
                                        .value_set(
                                            &[
                                                (
                                                    &::tracing::__macro_support::Iterator::next(&mut iter)
                                                        .expect("FieldSet corrupted (this is a bug)"),
                                                    ::tracing::__macro_support::Option::Some(
                                                        &format_args!("completed path") as &dyn Value,
                                                    ),
                                                ),
                                            ],
                                        )
                                });
                            } else {
                            }
                        };
                        path_q.act(task.target, 0., 0., 0.)?;
                        commands.entity(id).despawn();
                        return Ok(());
                    }
                    let obs = path_q.observe(task.target, steps)?;
                    let [[angle, force, torque, _, _, _]] = model::main_graph([
                        obs.into(),
                    ]);
                    path_q.act(task.target, angle, force, torque)?;
                }
                Ok(())
            }
            impl PathQuery<'_, '_> {
                pub fn path(&self, entity: Entity, to: Vec2) -> Option<PathTaskBundle> {
                    let (pos, containing_room) = self.pawn_q.get(entity).ok()?;
                    let map = self.parent_q.get(containing_room.get()).ok()?.parent();
                    let room = self.mesh_q.get(map).ok()?;
                    if let Some(path) = room.path(pos.0, to) {
                        return Some(PathTaskBundle {
                            task: Task::new(entity),
                            path: PathTask::Running(path.path.into_iter().collect()),
                        });
                    }
                    None
                }
            }
            impl MovementQuery<'_, '_> {
                pub fn observe(
                    &self,
                    entity: Entity,
                    steps: &mut VecDeque<Vec2>,
                ) -> Result<PathObservation, QueryEntityError> {
                    let (
                        _,
                        position,
                        rotation,
                        collider,
                        linear_velocity,
                        angular_velocity,
                    ) = self.pawn_q.get(entity)?;
                    let target = loop {
                        let Some(&current_step) = steps.front() else {
                            break None;
                        };
                        if position.distance_squared(current_step)
                            < (POSITION_EPSILON * POSITION_EPSILON)
                        {
                            steps.pop_front();
                            continue;
                        }
                        if let Some(&next_step) = steps.get(1) {
                            if self.visible(position.0, next_step) {
                                steps.pop_front();
                                continue;
                            }
                        }
                        break Some(current_step);
                    };
                    let collision = self
                        .collision(entity, *position, *rotation, collider);
                    Ok(
                        PathObservation::new(
                            position,
                            rotation,
                            linear_velocity,
                            angular_velocity,
                            collision,
                            target.unwrap_or(position.0),
                        ),
                    )
                }
                pub fn act(
                    &mut self,
                    entity: Entity,
                    angle: f32,
                    force: f32,
                    torque: f32,
                ) -> Result<(), QueryEntityError> {
                    let (mut pawn, _, _, _, _, _) = self.pawn_q.get_mut(entity)?;
                    pawn.update_movement(angle, force, torque);
                    Ok(())
                }
                fn collision(
                    &self,
                    entity: Entity,
                    pawn_position: Position,
                    pawn_rotation: Rotation,
                    pawn_collider: &Collider,
                ) -> Option<PathCollision> {
                    let inv_isometry = Isometry2d::new(
                            pawn_position.0,
                            pawn_rotation.into(),
                        )
                        .inverse();
                    let mut result = None;
                    self.spatial_query
                        .shape_intersections_callback(
                            &self.config.collider,
                            pawn_position.0,
                            pawn_rotation.as_radians(),
                            &self.config.all_filter,
                            |collider_entity| {
                                if collider_entity == entity {
                                    return true;
                                }
                                let Ok(
                                    (
                                        collider_position,
                                        collider_rotation,
                                        collider_shape,
                                        collider_velocity,
                                        collider_is_wall,
                                        collider_is_pawn,
                                    ),
                                ) = self.collider_q.get(collider_entity) else {
                                    {
                                        use ::tracing::__macro_support::Callsite as _;
                                        static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                            static META: ::tracing::Metadata<'static> = {
                                                ::tracing_core::metadata::Metadata::new(
                                                    "event pb-engine\\src\\pawn\\ai\\path\\mod.rs:226",
                                                    "pb_engine::pawn::ai::path",
                                                    ::tracing::Level::WARN,
                                                    ::tracing_core::__macro_support::Option::Some(
                                                        "pb-engine\\src\\pawn\\ai\\path\\mod.rs",
                                                    ),
                                                    ::tracing_core::__macro_support::Option::Some(226u32),
                                                    ::tracing_core::__macro_support::Option::Some(
                                                        "pb_engine::pawn::ai::path",
                                                    ),
                                                    ::tracing_core::field::FieldSet::new(
                                                        &["message"],
                                                        ::tracing_core::callsite::Identifier(&__CALLSITE),
                                                    ),
                                                    ::tracing::metadata::Kind::EVENT,
                                                )
                                            };
                                            ::tracing::callsite::DefaultCallsite::new(&META)
                                        };
                                        let enabled = ::tracing::Level::WARN
                                            <= ::tracing::level_filters::STATIC_MAX_LEVEL
                                            && ::tracing::Level::WARN
                                                <= ::tracing::level_filters::LevelFilter::current()
                                            && {
                                                let interest = __CALLSITE.interest();
                                                !interest.is_never()
                                                    && ::tracing::__macro_support::__is_enabled(
                                                        __CALLSITE.metadata(),
                                                        interest,
                                                    )
                                            };
                                        if enabled {
                                            (|value_set: ::tracing::field::ValueSet| {
                                                let meta = __CALLSITE.metadata();
                                                ::tracing::Event::dispatch(meta, &value_set);
                                            })({
                                                #[allow(unused_imports)]
                                                use ::tracing::field::{debug, display, Value};
                                                let mut iter = __CALLSITE.metadata().fields().iter();
                                                __CALLSITE
                                                    .metadata()
                                                    .fields()
                                                    .value_set(
                                                        &[
                                                            (
                                                                &::tracing::__macro_support::Iterator::next(&mut iter)
                                                                    .expect("FieldSet corrupted (this is a bug)"),
                                                                ::tracing::__macro_support::Option::Some(
                                                                    &format_args!("invalid collision entity") as &dyn Value,
                                                                ),
                                                            ),
                                                        ],
                                                    )
                                            });
                                        } else {
                                        }
                                    };
                                    return true;
                                };
                                if let Some(contact)
                                    = contact_query::contact(
                                            pawn_collider,
                                            pawn_position,
                                            pawn_rotation,
                                            collider_shape,
                                            *collider_position,
                                            *collider_rotation,
                                            Pawn::VISION_RADIUS,
                                        )
                                        .unwrap()
                                {
                                    let point2 = collider_rotation * contact.local_point2
                                        - pawn_position.0;
                                    let normal = inv_isometry.rotation
                                        * contact.global_normal2(collider_rotation);
                                    let pawn_space_velocity = inv_isometry.rotation
                                        * collider_velocity.0;
                                    let contact = PathCollision {
                                        angle: point2.to_angle(),
                                        distance: -contact.penetration,
                                        normal: normal.to_angle(),
                                        velocity_t: pawn_space_velocity.to_angle(),
                                        velocity_r: pawn_space_velocity.length_squared()
                                            / (Pawn::MAX_VELOCITY * Pawn::MAX_VELOCITY),
                                        is_pawn: collider_is_pawn,
                                        is_wall: collider_is_wall,
                                    };
                                    match &result {
                                        None => result = Some(contact),
                                        Some(
                                            closest_contact,
                                        ) if contact.distance < closest_contact.distance => {
                                            result = Some(contact);
                                        }
                                        _ => {}
                                    }
                                }
                                true
                            },
                        );
                    result
                }
                fn visible(&self, position: Vec2, target: Vec2) -> bool {
                    let delta = target - position;
                    let Ok(dir) = Dir2::new(delta) else {
                        return true;
                    };
                    self.spatial_query
                        .cast_ray(
                            position,
                            dir,
                            delta.length(),
                            true,
                            &self.config.wall_filter,
                        )
                        .is_none()
                }
            }
            impl Default for PathQueryConfig {
                fn default() -> Self {
                    Self {
                        collider: Collider::circle(Pawn::VISION_RADIUS),
                        all_filter: SpatialQueryFilter {
                            mask: LayerMask(
                                Layer::Wall.to_bits() | Layer::Perimeter.to_bits(),
                            ),
                            ..Default::default()
                        },
                        wall_filter: SpatialQueryFilter {
                            mask: Layer::Wall.into(),
                            ..Default::default()
                        },
                    }
                }
            }
            pub struct PathCollision {
                angle: f32,
                distance: f32,
                normal: f32,
                velocity_t: f32,
                velocity_r: f32,
                is_pawn: bool,
                is_wall: bool,
            }
            #[automatically_derived]
            impl ::core::fmt::Debug for PathCollision {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    let names: &'static _ = &[
                        "angle",
                        "distance",
                        "normal",
                        "velocity_t",
                        "velocity_r",
                        "is_pawn",
                        "is_wall",
                    ];
                    let values: &[&dyn ::core::fmt::Debug] = &[
                        &self.angle,
                        &self.distance,
                        &self.normal,
                        &self.velocity_t,
                        &self.velocity_r,
                        &self.is_pawn,
                        &&self.is_wall,
                    ];
                    ::core::fmt::Formatter::debug_struct_fields_finish(
                        f,
                        "PathCollision",
                        names,
                        values,
                    )
                }
            }
            impl Default for PathCollision {
                fn default() -> Self {
                    Self {
                        angle: 1.,
                        distance: Pawn::VISION_RADIUS,
                        normal: 0.,
                        velocity_t: 0.,
                        velocity_r: 0.,
                        is_pawn: false,
                        is_wall: false,
                    }
                }
            }
            impl PathObservation {
                pub const SIZE: usize = 12;
                pub fn new(
                    position: &Position,
                    rotation: &Rotation,
                    linear_velocity: &LinearVelocity,
                    angular_velocity: &AngularVelocity,
                    collision: Option<PathCollision>,
                    target: Vec2,
                ) -> Self {
                    let inv_isometry = Isometry2d::new(position.0, (*rotation).into())
                        .inverse();
                    let pawn_space_target = inv_isometry * target;
                    let pawn_space_linear_velocity = inv_isometry.rotation
                        * linear_velocity.0;
                    let collision = collision.unwrap_or_default();
                    PathObservation {
                        linear_velocity_t: pawn_space_linear_velocity.to_angle() / PI,
                        linear_velocity_r: pawn_space_linear_velocity.length_squared()
                            / (Pawn::MAX_VELOCITY * Pawn::MAX_VELOCITY),
                        angular_velocity: angular_velocity.0
                            / Pawn::MAX_ANGULAR_VELOCITY,
                        target_t: pawn_space_target.to_angle() / PI,
                        target_r: pawn_space_target.length().min(Pawn::VISION_RADIUS),
                        collision_t: collision.angle / PI,
                        collision_r: collision.distance,
                        collision_normal_t: collision.normal / PI,
                        collision_velocity_t: collision.velocity_t / PI,
                        collision_velocity_r: collision.velocity_r,
                        collision_is_wall: collision.is_wall as u32 as f32,
                        collision_is_pawn: collision.is_pawn as u32 as f32,
                    }
                }
                pub fn velocity_reward(&self) -> f32 {
                    ((self.linear_velocity_t - self.target_t) * PI).cos()
                        * self.linear_velocity_r
                }
                pub fn rotation_penalty(&self) -> f32 {
                    -self.target_t.abs()
                }
                pub fn collision_penalty(&self) -> f32 {
                    if self.collision_r < 0.1 { 10. * self.collision_r - 1. } else { 0. }
                }
            }
            impl From<PathObservation> for [f32; PathObservation::SIZE] {
                fn from(obs: PathObservation) -> [f32; PathObservation::SIZE] {
                    [
                        obs.linear_velocity_t,
                        obs.linear_velocity_r,
                        obs.angular_velocity,
                        obs.target_t,
                        obs.target_r,
                        obs.collision_t,
                        obs.collision_r,
                        obs.collision_normal_t,
                        obs.collision_velocity_t,
                        obs.collision_velocity_r,
                        obs.collision_is_wall,
                        obs.collision_is_pawn,
                    ]
                }
            }
            impl PathTask {
                fn poll(&mut self) -> Option<&mut VecDeque<Vec2>> {
                    match self {
                        PathTask::Running(steps) => Some(steps),
                        PathTask::Pending(receiver) => {
                            if let Ok(Some(steps)) = receiver.try_recv() {
                                *self = PathTask::Running(steps);
                                match self {
                                    PathTask::Pending(_) => {
                                        ::core::panicking::panic(
                                            "internal error: entered unreachable code",
                                        )
                                    }
                                    PathTask::Running(steps) => Some(steps),
                                }
                            } else {
                                None
                            }
                        }
                    }
                }
                pub fn steps(&self) -> Option<&VecDeque<Vec2>> {
                    match self {
                        PathTask::Pending(_) => None,
                        PathTask::Running(steps) => Some(steps),
                    }
                }
            }
        }
        use bevy::prelude::*;
        #[relationship(relationship_target = TaskStack)]
        pub struct Task {
            target: Entity,
        }
        #[automatically_derived]
        impl ::core::clone::Clone for Task {
            #[inline]
            fn clone(&self) -> Task {
                Task {
                    target: ::core::clone::Clone::clone(&self.target),
                }
            }
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Task {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field1_finish(
                    f,
                    "Task",
                    "target",
                    &&self.target,
                )
            }
        }
        impl bevy::ecs::component::Component for Task
        where
            Self: Send + Sync + 'static,
        {
            const STORAGE_TYPE: bevy::ecs::component::StorageType = bevy::ecs::component::StorageType::Table;
            type Mutability = bevy::ecs::component::Immutable;
            fn register_required_components(
                requiree: bevy::ecs::component::ComponentId,
                components: &mut bevy::ecs::component::ComponentsRegistrator,
                required_components: &mut bevy::ecs::component::RequiredComponents,
                inheritance_depth: u16,
                recursion_check_stack: &mut bevy::ecs::__macro_exports::Vec<
                    bevy::ecs::component::ComponentId,
                >,
            ) {
                bevy::ecs::component::enforce_no_required_components_recursion(
                    components,
                    recursion_check_stack,
                );
                let self_id = components.register_component::<Self>();
                recursion_check_stack.push(self_id);
                recursion_check_stack.pop();
            }
            fn on_insert() -> ::core::option::Option<
                bevy::ecs::component::ComponentHook,
            > {
                ::core::option::Option::Some(
                    <Self as bevy::ecs::relationship::Relationship>::on_insert,
                )
            }
            fn on_replace() -> ::core::option::Option<
                bevy::ecs::component::ComponentHook,
            > {
                ::core::option::Option::Some(
                    <Self as bevy::ecs::relationship::Relationship>::on_replace,
                )
            }
            fn clone_behavior() -> bevy::ecs::component::ComponentCloneBehavior {
                use bevy::ecs::component::{
                    DefaultCloneBehaviorBase, DefaultCloneBehaviorViaClone,
                };
                (&&&bevy::ecs::component::DefaultCloneBehaviorSpecialization::<
                    Self,
                >::default())
                    .default_clone_behavior()
            }
            fn map_entities<M: bevy::ecs::entity::EntityMapper>(
                this: &mut Self,
                mapper: &mut M,
            ) {
                use bevy::ecs::entity::MapEntities;
                this.target.map_entities(mapper);
            }
        }
        impl bevy::ecs::relationship::Relationship for Task {
            type RelationshipTarget = TaskStack;
            #[inline(always)]
            fn get(&self) -> bevy::ecs::entity::Entity {
                self.target
            }
            #[inline]
            fn from(entity: bevy::ecs::entity::Entity) -> Self {
                Self { target: entity }
            }
        }
        #[relationship_target(relationship = Task)]
        pub struct TaskStack {
            tasks: Vec<Entity>,
        }
        #[automatically_derived]
        impl ::core::default::Default for TaskStack {
            #[inline]
            fn default() -> TaskStack {
                TaskStack {
                    tasks: ::core::default::Default::default(),
                }
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for TaskStack {
            #[inline]
            fn clone(&self) -> TaskStack {
                TaskStack {
                    tasks: ::core::clone::Clone::clone(&self.tasks),
                }
            }
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for TaskStack {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field1_finish(
                    f,
                    "TaskStack",
                    "tasks",
                    &&self.tasks,
                )
            }
        }
        impl bevy::ecs::component::Component for TaskStack
        where
            Self: Send + Sync + 'static,
        {
            const STORAGE_TYPE: bevy::ecs::component::StorageType = bevy::ecs::component::StorageType::Table;
            type Mutability = bevy::ecs::component::Mutable;
            fn register_required_components(
                requiree: bevy::ecs::component::ComponentId,
                components: &mut bevy::ecs::component::ComponentsRegistrator,
                required_components: &mut bevy::ecs::component::RequiredComponents,
                inheritance_depth: u16,
                recursion_check_stack: &mut bevy::ecs::__macro_exports::Vec<
                    bevy::ecs::component::ComponentId,
                >,
            ) {
                bevy::ecs::component::enforce_no_required_components_recursion(
                    components,
                    recursion_check_stack,
                );
                let self_id = components.register_component::<Self>();
                recursion_check_stack.push(self_id);
                recursion_check_stack.pop();
            }
            fn on_replace() -> ::core::option::Option<
                bevy::ecs::component::ComponentHook,
            > {
                ::core::option::Option::Some(
                    <Self as bevy::ecs::relationship::RelationshipTarget>::on_replace,
                )
            }
            fn clone_behavior() -> bevy::ecs::component::ComponentCloneBehavior {
                bevy::ecs::component::ComponentCloneBehavior::Custom(
                    bevy::ecs::relationship::clone_relationship_target::<Self>,
                )
            }
            fn map_entities<M: bevy::ecs::entity::EntityMapper>(
                this: &mut Self,
                mapper: &mut M,
            ) {
                use bevy::ecs::entity::MapEntities;
                this.tasks.map_entities(mapper);
            }
        }
        impl bevy::ecs::relationship::RelationshipTarget for TaskStack {
            const LINKED_SPAWN: bool = false;
            type Relationship = Task;
            type Collection = Vec<Entity>;
            #[inline]
            fn collection(&self) -> &Self::Collection {
                &self.tasks
            }
            #[inline]
            fn collection_mut_risky(&mut self) -> &mut Self::Collection {
                &mut self.tasks
            }
            #[inline]
            fn from_collection_risky(collection: Self::Collection) -> Self {
                Self { tasks: collection }
            }
        }
        impl Task {
            pub fn new(target: Entity) -> Self {
                Task { target }
            }
            pub fn actor(&self) -> Entity {
                self.target
            }
        }
        pub fn task_added(
            trigger: Trigger<OnInsert, Task>,
            mut commands: Commands,
            task_q: Query<&Task>,
            mut actor_q: Query<&mut Actor>,
        ) -> Result {
            let task = task_q.get(trigger.target())?;
            let mut actor = actor_q.get_mut(task.target)?;
            if let Some(prev_task) = actor.task.replace(trigger.target()) {
                commands.entity(prev_task).despawn();
            }
            Ok(())
        }
        pub fn task_removed(
            trigger: Trigger<OnReplace, Task>,
            task_q: Query<&Task>,
            mut actor_q: Query<&mut Actor>,
        ) -> Result {
            let task = task_q.get(trigger.target())?;
            if let Ok(mut actor) = actor_q.get_mut(task.target) {
                if actor.task == Some(trigger.target()) {
                    actor.task = None;
                }
            }
            Ok(())
        }
        pub fn actor_removed(
            trigger: Trigger<OnReplace, Actor>,
            mut commands: Commands,
            actor_q: Query<&Actor>,
        ) -> Result {
            let actor = actor_q.get(trigger.target())?;
            if let Some(task) = actor.task {
                commands.entity(task).despawn();
            }
            Ok(())
        }
    }
    use std::f32::consts::{PI, TAU};
    use ai::Actor;
    use approx::relative_ne;
    use avian2d::prelude::*;
    use bevy::prelude::*;
    use pb_util::math::to_finite_f32_lossy;
    use serde::{Deserialize, Serialize};
    use crate::layer::Layer;
    #[reflect(Component, Serialize, Deserialize)]
    #[require(
        Name::new("Pawn"),
        Actor,
        RigidBody::Dynamic,
        Collider::circle(Pawn::RADIUS),
        CollisionLayers::new(Layer::Pawn, LayerMask::ALL),
        CollidingEntities,
        TranslationInterpolation,
        LinearDamping(0.5),
        AngularDamping(0.5)
    )]
    pub struct Pawn {
        pub dir: Vec2,
        pub accel: f32,
        pub torque: f32,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Pawn {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field3_finish(
                f,
                "Pawn",
                "dir",
                &self.dir,
                "accel",
                &self.accel,
                "torque",
                &&self.torque,
            )
        }
    }
    #[automatically_derived]
    impl ::core::default::Default for Pawn {
        #[inline]
        fn default() -> Pawn {
            Pawn {
                dir: ::core::default::Default::default(),
                accel: ::core::default::Default::default(),
                torque: ::core::default::Default::default(),
            }
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for Pawn {}
    #[automatically_derived]
    impl ::core::clone::Clone for Pawn {
        #[inline]
        fn clone(&self) -> Pawn {
            let _: ::core::clone::AssertParamIsClone<Vec2>;
            let _: ::core::clone::AssertParamIsClone<f32>;
            *self
        }
    }
    #[doc = "**Required Components**: [`Name`], [`Actor`], [`RigidBody`], [`Collider`], [`CollisionLayers`], [`CollidingEntities`], [`TranslationInterpolation`], [`LinearDamping`], [`AngularDamping`]. \n\n A component's Required Components are inserted whenever it is inserted. Note that this will also insert the required components _of_ the required components, recursively, in depth-first order."]
    impl bevy::ecs::component::Component for Pawn
    where
        Self: Send + Sync + 'static,
    {
        const STORAGE_TYPE: bevy::ecs::component::StorageType = bevy::ecs::component::StorageType::Table;
        type Mutability = bevy::ecs::component::Mutable;
        fn register_required_components(
            requiree: bevy::ecs::component::ComponentId,
            components: &mut bevy::ecs::component::ComponentsRegistrator,
            required_components: &mut bevy::ecs::component::RequiredComponents,
            inheritance_depth: u16,
            recursion_check_stack: &mut bevy::ecs::__macro_exports::Vec<
                bevy::ecs::component::ComponentId,
            >,
        ) {
            bevy::ecs::component::enforce_no_required_components_recursion(
                components,
                recursion_check_stack,
            );
            let self_id = components.register_component::<Self>();
            recursion_check_stack.push(self_id);
            components
                .register_required_components_manual::<
                    Self,
                    Name,
                >(
                    required_components,
                    || {
                        let x: Name = (|| Name::new("Pawn"))().into();
                        x
                    },
                    inheritance_depth,
                    recursion_check_stack,
                );
            components
                .register_required_components_manual::<
                    Self,
                    Actor,
                >(
                    required_components,
                    <Actor as Default>::default,
                    inheritance_depth,
                    recursion_check_stack,
                );
            components
                .register_required_components_manual::<
                    Self,
                    RigidBody,
                >(
                    required_components,
                    || {
                        let x: RigidBody = (|| RigidBody::Dynamic)().into();
                        x
                    },
                    inheritance_depth,
                    recursion_check_stack,
                );
            components
                .register_required_components_manual::<
                    Self,
                    Collider,
                >(
                    required_components,
                    || {
                        let x: Collider = (|| Collider::circle(Pawn::RADIUS))().into();
                        x
                    },
                    inheritance_depth,
                    recursion_check_stack,
                );
            components
                .register_required_components_manual::<
                    Self,
                    CollisionLayers,
                >(
                    required_components,
                    || {
                        let x: CollisionLayers = (|| CollisionLayers::new(
                            Layer::Pawn,
                            LayerMask::ALL,
                        ))()
                            .into();
                        x
                    },
                    inheritance_depth,
                    recursion_check_stack,
                );
            components
                .register_required_components_manual::<
                    Self,
                    CollidingEntities,
                >(
                    required_components,
                    <CollidingEntities as Default>::default,
                    inheritance_depth,
                    recursion_check_stack,
                );
            components
                .register_required_components_manual::<
                    Self,
                    TranslationInterpolation,
                >(
                    required_components,
                    <TranslationInterpolation as Default>::default,
                    inheritance_depth,
                    recursion_check_stack,
                );
            components
                .register_required_components_manual::<
                    Self,
                    LinearDamping,
                >(
                    required_components,
                    || {
                        let x: LinearDamping = (|| LinearDamping(0.5))().into();
                        x
                    },
                    inheritance_depth,
                    recursion_check_stack,
                );
            components
                .register_required_components_manual::<
                    Self,
                    AngularDamping,
                >(
                    required_components,
                    || {
                        let x: AngularDamping = (|| AngularDamping(0.5))().into();
                        x
                    },
                    inheritance_depth,
                    recursion_check_stack,
                );
            <Name as bevy::ecs::component::Component>::register_required_components(
                requiree,
                components,
                required_components,
                inheritance_depth + 1,
                recursion_check_stack,
            );
            <Actor as bevy::ecs::component::Component>::register_required_components(
                requiree,
                components,
                required_components,
                inheritance_depth + 1,
                recursion_check_stack,
            );
            <RigidBody as bevy::ecs::component::Component>::register_required_components(
                requiree,
                components,
                required_components,
                inheritance_depth + 1,
                recursion_check_stack,
            );
            <Collider as bevy::ecs::component::Component>::register_required_components(
                requiree,
                components,
                required_components,
                inheritance_depth + 1,
                recursion_check_stack,
            );
            <CollisionLayers as bevy::ecs::component::Component>::register_required_components(
                requiree,
                components,
                required_components,
                inheritance_depth + 1,
                recursion_check_stack,
            );
            <CollidingEntities as bevy::ecs::component::Component>::register_required_components(
                requiree,
                components,
                required_components,
                inheritance_depth + 1,
                recursion_check_stack,
            );
            <TranslationInterpolation as bevy::ecs::component::Component>::register_required_components(
                requiree,
                components,
                required_components,
                inheritance_depth + 1,
                recursion_check_stack,
            );
            <LinearDamping as bevy::ecs::component::Component>::register_required_components(
                requiree,
                components,
                required_components,
                inheritance_depth + 1,
                recursion_check_stack,
            );
            <AngularDamping as bevy::ecs::component::Component>::register_required_components(
                requiree,
                components,
                required_components,
                inheritance_depth + 1,
                recursion_check_stack,
            );
            recursion_check_stack.pop();
        }
        fn clone_behavior() -> bevy::ecs::component::ComponentCloneBehavior {
            use bevy::ecs::component::{
                DefaultCloneBehaviorBase, DefaultCloneBehaviorViaClone,
            };
            (&&&bevy::ecs::component::DefaultCloneBehaviorSpecialization::<
                Self,
            >::default())
                .default_clone_behavior()
        }
    }
    const _: () = {
        #[allow(unused_mut)]
        impl bevy::reflect::GetTypeRegistration for Pawn
        where
            Pawn: ::core::any::Any + ::core::marker::Send + ::core::marker::Sync,
            Vec2: bevy::reflect::FromReflect + bevy::reflect::TypePath
                + bevy::reflect::MaybeTyped
                + bevy::reflect::__macro_exports::RegisterForReflection,
            f32: bevy::reflect::FromReflect + bevy::reflect::TypePath
                + bevy::reflect::MaybeTyped
                + bevy::reflect::__macro_exports::RegisterForReflection,
            f32: bevy::reflect::FromReflect + bevy::reflect::TypePath
                + bevy::reflect::MaybeTyped
                + bevy::reflect::__macro_exports::RegisterForReflection,
        {
            fn get_type_registration() -> bevy::reflect::TypeRegistration {
                let mut registration = bevy::reflect::TypeRegistration::of::<Self>();
                registration
                    .insert::<
                        bevy::reflect::ReflectFromPtr,
                    >(bevy::reflect::FromType::<Self>::from_type());
                registration
                    .insert::<
                        bevy::reflect::ReflectFromReflect,
                    >(bevy::reflect::FromType::<Self>::from_type());
                registration
                    .insert::<
                        ReflectComponent,
                    >(bevy::reflect::FromType::<Self>::from_type());
                registration
                    .insert::<
                        ReflectSerialize,
                    >(bevy::reflect::FromType::<Self>::from_type());
                registration
                    .insert::<
                        ReflectDeserialize,
                    >(bevy::reflect::FromType::<Self>::from_type());
                registration
            }
            #[inline(never)]
            fn register_type_dependencies(registry: &mut bevy::reflect::TypeRegistry) {
                <Vec2 as bevy::reflect::__macro_exports::RegisterForReflection>::__register(
                    registry,
                );
                <f32 as bevy::reflect::__macro_exports::RegisterForReflection>::__register(
                    registry,
                );
                <f32 as bevy::reflect::__macro_exports::RegisterForReflection>::__register(
                    registry,
                );
            }
        }
        impl bevy::reflect::Typed for Pawn
        where
            Pawn: ::core::any::Any + ::core::marker::Send + ::core::marker::Sync,
            Vec2: bevy::reflect::FromReflect + bevy::reflect::TypePath
                + bevy::reflect::MaybeTyped
                + bevy::reflect::__macro_exports::RegisterForReflection,
            f32: bevy::reflect::FromReflect + bevy::reflect::TypePath
                + bevy::reflect::MaybeTyped
                + bevy::reflect::__macro_exports::RegisterForReflection,
            f32: bevy::reflect::FromReflect + bevy::reflect::TypePath
                + bevy::reflect::MaybeTyped
                + bevy::reflect::__macro_exports::RegisterForReflection,
        {
            #[inline]
            fn type_info() -> &'static bevy::reflect::TypeInfo {
                static CELL: bevy::reflect::utility::NonGenericTypeInfoCell = bevy::reflect::utility::NonGenericTypeInfoCell::new();
                CELL.get_or_set(|| {
                    bevy::reflect::TypeInfo::Struct(
                        bevy::reflect::StructInfo::new::<
                            Self,
                        >(
                                &[
                                    bevy::reflect::NamedField::new::<Vec2>("dir")
                                        .with_custom_attributes(
                                            bevy::reflect::attributes::CustomAttributes::default(),
                                        ),
                                    bevy::reflect::NamedField::new::<f32>("accel")
                                        .with_custom_attributes(
                                            bevy::reflect::attributes::CustomAttributes::default(),
                                        ),
                                    bevy::reflect::NamedField::new::<f32>("torque")
                                        .with_custom_attributes(
                                            bevy::reflect::attributes::CustomAttributes::default(),
                                        ),
                                ],
                            )
                            .with_custom_attributes(
                                bevy::reflect::attributes::CustomAttributes::default(),
                            ),
                    )
                })
            }
        }
        const _: () = {
            extern crate alloc;
            use alloc::string::ToString;
            impl bevy::reflect::TypePath for Pawn
            where
                Pawn: ::core::any::Any + ::core::marker::Send + ::core::marker::Sync,
            {
                fn type_path() -> &'static str {
                    "pb_engine::pawn::Pawn"
                }
                fn short_type_path() -> &'static str {
                    "Pawn"
                }
                fn type_ident() -> Option<&'static str> {
                    ::core::option::Option::Some("Pawn")
                }
                fn crate_name() -> Option<&'static str> {
                    ::core::option::Option::Some(
                        "pb_engine::pawn".split(':').next().unwrap(),
                    )
                }
                fn module_path() -> Option<&'static str> {
                    ::core::option::Option::Some("pb_engine::pawn")
                }
            }
        };
        impl bevy::reflect::Reflect for Pawn
        where
            Pawn: ::core::any::Any + ::core::marker::Send + ::core::marker::Sync,
            Vec2: bevy::reflect::FromReflect + bevy::reflect::TypePath
                + bevy::reflect::MaybeTyped
                + bevy::reflect::__macro_exports::RegisterForReflection,
            f32: bevy::reflect::FromReflect + bevy::reflect::TypePath
                + bevy::reflect::MaybeTyped
                + bevy::reflect::__macro_exports::RegisterForReflection,
            f32: bevy::reflect::FromReflect + bevy::reflect::TypePath
                + bevy::reflect::MaybeTyped
                + bevy::reflect::__macro_exports::RegisterForReflection,
        {
            #[inline]
            fn into_any(
                self: bevy::reflect::__macro_exports::alloc_utils::Box<Self>,
            ) -> bevy::reflect::__macro_exports::alloc_utils::Box<dyn ::core::any::Any> {
                self
            }
            #[inline]
            fn as_any(&self) -> &dyn ::core::any::Any {
                self
            }
            #[inline]
            fn as_any_mut(&mut self) -> &mut dyn ::core::any::Any {
                self
            }
            #[inline]
            fn into_reflect(
                self: bevy::reflect::__macro_exports::alloc_utils::Box<Self>,
            ) -> bevy::reflect::__macro_exports::alloc_utils::Box<
                dyn bevy::reflect::Reflect,
            > {
                self
            }
            #[inline]
            fn as_reflect(&self) -> &dyn bevy::reflect::Reflect {
                self
            }
            #[inline]
            fn as_reflect_mut(&mut self) -> &mut dyn bevy::reflect::Reflect {
                self
            }
            #[inline]
            fn set(
                &mut self,
                value: bevy::reflect::__macro_exports::alloc_utils::Box<
                    dyn bevy::reflect::Reflect,
                >,
            ) -> ::core::result::Result<
                (),
                bevy::reflect::__macro_exports::alloc_utils::Box<
                    dyn bevy::reflect::Reflect,
                >,
            > {
                *self = <dyn bevy::reflect::Reflect>::take(value)?;
                ::core::result::Result::Ok(())
            }
        }
        impl bevy::reflect::Struct for Pawn
        where
            Pawn: ::core::any::Any + ::core::marker::Send + ::core::marker::Sync,
            Vec2: bevy::reflect::FromReflect + bevy::reflect::TypePath
                + bevy::reflect::MaybeTyped
                + bevy::reflect::__macro_exports::RegisterForReflection,
            f32: bevy::reflect::FromReflect + bevy::reflect::TypePath
                + bevy::reflect::MaybeTyped
                + bevy::reflect::__macro_exports::RegisterForReflection,
            f32: bevy::reflect::FromReflect + bevy::reflect::TypePath
                + bevy::reflect::MaybeTyped
                + bevy::reflect::__macro_exports::RegisterForReflection,
        {
            fn field(
                &self,
                name: &str,
            ) -> ::core::option::Option<&dyn bevy::reflect::PartialReflect> {
                match name {
                    "dir" => ::core::option::Option::Some(&self.dir),
                    "accel" => ::core::option::Option::Some(&self.accel),
                    "torque" => ::core::option::Option::Some(&self.torque),
                    _ => ::core::option::Option::None,
                }
            }
            fn field_mut(
                &mut self,
                name: &str,
            ) -> ::core::option::Option<&mut dyn bevy::reflect::PartialReflect> {
                match name {
                    "dir" => ::core::option::Option::Some(&mut self.dir),
                    "accel" => ::core::option::Option::Some(&mut self.accel),
                    "torque" => ::core::option::Option::Some(&mut self.torque),
                    _ => ::core::option::Option::None,
                }
            }
            fn field_at(
                &self,
                index: usize,
            ) -> ::core::option::Option<&dyn bevy::reflect::PartialReflect> {
                match index {
                    0usize => ::core::option::Option::Some(&self.dir),
                    1usize => ::core::option::Option::Some(&self.accel),
                    2usize => ::core::option::Option::Some(&self.torque),
                    _ => ::core::option::Option::None,
                }
            }
            fn field_at_mut(
                &mut self,
                index: usize,
            ) -> ::core::option::Option<&mut dyn bevy::reflect::PartialReflect> {
                match index {
                    0usize => ::core::option::Option::Some(&mut self.dir),
                    1usize => ::core::option::Option::Some(&mut self.accel),
                    2usize => ::core::option::Option::Some(&mut self.torque),
                    _ => ::core::option::Option::None,
                }
            }
            fn name_at(&self, index: usize) -> ::core::option::Option<&str> {
                match index {
                    0usize => ::core::option::Option::Some("dir"),
                    1usize => ::core::option::Option::Some("accel"),
                    2usize => ::core::option::Option::Some("torque"),
                    _ => ::core::option::Option::None,
                }
            }
            fn field_len(&self) -> usize {
                3usize
            }
            fn iter_fields(&self) -> bevy::reflect::FieldIter {
                bevy::reflect::FieldIter::new(self)
            }
            fn to_dynamic_struct(&self) -> bevy::reflect::DynamicStruct {
                let mut dynamic: bevy::reflect::DynamicStruct = ::core::default::Default::default();
                dynamic
                    .set_represented_type(
                        bevy::reflect::PartialReflect::get_represented_type_info(self),
                    );
                dynamic
                    .insert_boxed(
                        "dir",
                        bevy::reflect::PartialReflect::to_dynamic(&self.dir),
                    );
                dynamic
                    .insert_boxed(
                        "accel",
                        bevy::reflect::PartialReflect::to_dynamic(&self.accel),
                    );
                dynamic
                    .insert_boxed(
                        "torque",
                        bevy::reflect::PartialReflect::to_dynamic(&self.torque),
                    );
                dynamic
            }
        }
        impl bevy::reflect::PartialReflect for Pawn
        where
            Pawn: ::core::any::Any + ::core::marker::Send + ::core::marker::Sync,
            Vec2: bevy::reflect::FromReflect + bevy::reflect::TypePath
                + bevy::reflect::MaybeTyped
                + bevy::reflect::__macro_exports::RegisterForReflection,
            f32: bevy::reflect::FromReflect + bevy::reflect::TypePath
                + bevy::reflect::MaybeTyped
                + bevy::reflect::__macro_exports::RegisterForReflection,
            f32: bevy::reflect::FromReflect + bevy::reflect::TypePath
                + bevy::reflect::MaybeTyped
                + bevy::reflect::__macro_exports::RegisterForReflection,
        {
            #[inline]
            fn get_represented_type_info(
                &self,
            ) -> ::core::option::Option<&'static bevy::reflect::TypeInfo> {
                ::core::option::Option::Some(<Self as bevy::reflect::Typed>::type_info())
            }
            #[inline]
            fn try_apply(
                &mut self,
                value: &dyn bevy::reflect::PartialReflect,
            ) -> ::core::result::Result<(), bevy::reflect::ApplyError> {
                if let bevy::reflect::ReflectRef::Struct(struct_value)
                    = bevy::reflect::PartialReflect::reflect_ref(value) {
                    for (i, value) in ::core::iter::Iterator::enumerate(
                        bevy::reflect::Struct::iter_fields(struct_value),
                    ) {
                        let name = bevy::reflect::Struct::name_at(struct_value, i)
                            .unwrap();
                        if let ::core::option::Option::Some(v)
                            = bevy::reflect::Struct::field_mut(self, name) {
                            bevy::reflect::PartialReflect::try_apply(v, value)?;
                        }
                    }
                } else {
                    return ::core::result::Result::Err(bevy::reflect::ApplyError::MismatchedKinds {
                        from_kind: bevy::reflect::PartialReflect::reflect_kind(value),
                        to_kind: bevy::reflect::ReflectKind::Struct,
                    });
                }
                ::core::result::Result::Ok(())
            }
            #[inline]
            fn reflect_kind(&self) -> bevy::reflect::ReflectKind {
                bevy::reflect::ReflectKind::Struct
            }
            #[inline]
            fn reflect_ref(&self) -> bevy::reflect::ReflectRef {
                bevy::reflect::ReflectRef::Struct(self)
            }
            #[inline]
            fn reflect_mut(&mut self) -> bevy::reflect::ReflectMut {
                bevy::reflect::ReflectMut::Struct(self)
            }
            #[inline]
            fn reflect_owned(
                self: bevy::reflect::__macro_exports::alloc_utils::Box<Self>,
            ) -> bevy::reflect::ReflectOwned {
                bevy::reflect::ReflectOwned::Struct(self)
            }
            #[inline]
            fn try_into_reflect(
                self: bevy::reflect::__macro_exports::alloc_utils::Box<Self>,
            ) -> ::core::result::Result<
                bevy::reflect::__macro_exports::alloc_utils::Box<
                    dyn bevy::reflect::Reflect,
                >,
                bevy::reflect::__macro_exports::alloc_utils::Box<
                    dyn bevy::reflect::PartialReflect,
                >,
            > {
                ::core::result::Result::Ok(self)
            }
            #[inline]
            fn try_as_reflect(
                &self,
            ) -> ::core::option::Option<&dyn bevy::reflect::Reflect> {
                ::core::option::Option::Some(self)
            }
            #[inline]
            fn try_as_reflect_mut(
                &mut self,
            ) -> ::core::option::Option<&mut dyn bevy::reflect::Reflect> {
                ::core::option::Option::Some(self)
            }
            #[inline]
            fn into_partial_reflect(
                self: bevy::reflect::__macro_exports::alloc_utils::Box<Self>,
            ) -> bevy::reflect::__macro_exports::alloc_utils::Box<
                dyn bevy::reflect::PartialReflect,
            > {
                self
            }
            #[inline]
            fn as_partial_reflect(&self) -> &dyn bevy::reflect::PartialReflect {
                self
            }
            #[inline]
            fn as_partial_reflect_mut(
                &mut self,
            ) -> &mut dyn bevy::reflect::PartialReflect {
                self
            }
            fn reflect_partial_eq(
                &self,
                value: &dyn bevy::reflect::PartialReflect,
            ) -> ::core::option::Option<bool> {
                (bevy::reflect::struct_partial_eq)(self, value)
            }
            #[inline]
            #[allow(
                unreachable_code,
                reason = "Ignored fields without a `clone` attribute will early-return with an error"
            )]
            fn reflect_clone(
                &self,
            ) -> ::core::result::Result<
                bevy::reflect::__macro_exports::alloc_utils::Box<
                    dyn bevy::reflect::Reflect,
                >,
                bevy::reflect::ReflectCloneError,
            > {
                ::core::result::Result::Ok(
                    bevy::reflect::__macro_exports::alloc_utils::Box::new(Self {
                        dir: bevy::reflect::PartialReflect::reflect_clone(&self.dir)?
                            .take()
                            .map_err(|value| bevy::reflect::ReflectCloneError::FailedDowncast {
                                expected: bevy::reflect::__macro_exports::alloc_utils::Cow::Borrowed(
                                    <Vec2 as bevy::reflect::TypePath>::type_path(),
                                ),
                                received: bevy::reflect::__macro_exports::alloc_utils::Cow::Owned(
                                    bevy::reflect::__macro_exports::alloc_utils::ToString::to_string(
                                        bevy::reflect::DynamicTypePath::reflect_type_path(&*value),
                                    ),
                                ),
                            })?,
                        accel: bevy::reflect::PartialReflect::reflect_clone(&self.accel)?
                            .take()
                            .map_err(|value| bevy::reflect::ReflectCloneError::FailedDowncast {
                                expected: bevy::reflect::__macro_exports::alloc_utils::Cow::Borrowed(
                                    <f32 as bevy::reflect::TypePath>::type_path(),
                                ),
                                received: bevy::reflect::__macro_exports::alloc_utils::Cow::Owned(
                                    bevy::reflect::__macro_exports::alloc_utils::ToString::to_string(
                                        bevy::reflect::DynamicTypePath::reflect_type_path(&*value),
                                    ),
                                ),
                            })?,
                        torque: bevy::reflect::PartialReflect::reflect_clone(
                                &self.torque,
                            )?
                            .take()
                            .map_err(|value| bevy::reflect::ReflectCloneError::FailedDowncast {
                                expected: bevy::reflect::__macro_exports::alloc_utils::Cow::Borrowed(
                                    <f32 as bevy::reflect::TypePath>::type_path(),
                                ),
                                received: bevy::reflect::__macro_exports::alloc_utils::Cow::Owned(
                                    bevy::reflect::__macro_exports::alloc_utils::ToString::to_string(
                                        bevy::reflect::DynamicTypePath::reflect_type_path(&*value),
                                    ),
                                ),
                            })?,
                    }),
                )
            }
        }
        impl bevy::reflect::FromReflect for Pawn
        where
            Pawn: ::core::any::Any + ::core::marker::Send + ::core::marker::Sync,
            Vec2: bevy::reflect::FromReflect + bevy::reflect::TypePath
                + bevy::reflect::MaybeTyped
                + bevy::reflect::__macro_exports::RegisterForReflection,
            f32: bevy::reflect::FromReflect + bevy::reflect::TypePath
                + bevy::reflect::MaybeTyped
                + bevy::reflect::__macro_exports::RegisterForReflection,
            f32: bevy::reflect::FromReflect + bevy::reflect::TypePath
                + bevy::reflect::MaybeTyped
                + bevy::reflect::__macro_exports::RegisterForReflection,
        {
            fn from_reflect(
                reflect: &dyn bevy::reflect::PartialReflect,
            ) -> ::core::option::Option<Self> {
                if let bevy::reflect::ReflectRef::Struct(__ref_struct)
                    = bevy::reflect::PartialReflect::reflect_ref(reflect) {
                    let __this = Self {
                        dir: (|| <Vec2 as bevy::reflect::FromReflect>::from_reflect(
                            bevy::reflect::Struct::field(__ref_struct, "dir")?,
                        ))()?,
                        accel: (|| <f32 as bevy::reflect::FromReflect>::from_reflect(
                            bevy::reflect::Struct::field(__ref_struct, "accel")?,
                        ))()?,
                        torque: (|| <f32 as bevy::reflect::FromReflect>::from_reflect(
                            bevy::reflect::Struct::field(__ref_struct, "torque")?,
                        ))()?,
                    };
                    ::core::option::Option::Some(__this)
                } else {
                    ::core::option::Option::None
                }
            }
        }
    };
    #[doc(hidden)]
    #[allow(
        non_upper_case_globals,
        unused_attributes,
        unused_qualifications,
        clippy::absolute_paths,
    )]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for Pawn {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = _serde::Serializer::serialize_struct(
                    __serializer,
                    "Pawn",
                    false as usize + 1 + 1 + 1,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "dir",
                    &self.dir,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "accel",
                    &self.accel,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "torque",
                    &self.torque,
                )?;
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    #[doc(hidden)]
    #[allow(
        non_upper_case_globals,
        unused_attributes,
        unused_qualifications,
        clippy::absolute_paths,
    )]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for Pawn {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                #[doc(hidden)]
                enum __Field {
                    __field0,
                    __field1,
                    __field2,
                    __ignore,
                }
                #[doc(hidden)]
                struct __FieldVisitor;
                #[automatically_derived]
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "field identifier",
                        )
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private::Ok(__Field::__field0),
                            1u64 => _serde::__private::Ok(__Field::__field1),
                            2u64 => _serde::__private::Ok(__Field::__field2),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "dir" => _serde::__private::Ok(__Field::__field0),
                            "accel" => _serde::__private::Ok(__Field::__field1),
                            "torque" => _serde::__private::Ok(__Field::__field2),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"dir" => _serde::__private::Ok(__Field::__field0),
                            b"accel" => _serde::__private::Ok(__Field::__field1),
                            b"torque" => _serde::__private::Ok(__Field::__field2),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                }
                #[automatically_derived]
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(
                            __deserializer,
                            __FieldVisitor,
                        )
                    }
                }
                #[doc(hidden)]
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<Pawn>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                #[automatically_derived]
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = Pawn;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "struct Pawn",
                        )
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = match _serde::de::SeqAccess::next_element::<
                            Vec2,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        0usize,
                                        &"struct Pawn with 3 elements",
                                    ),
                                );
                            }
                        };
                        let __field1 = match _serde::de::SeqAccess::next_element::<
                            f32,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        1usize,
                                        &"struct Pawn with 3 elements",
                                    ),
                                );
                            }
                        };
                        let __field2 = match _serde::de::SeqAccess::next_element::<
                            f32,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        2usize,
                                        &"struct Pawn with 3 elements",
                                    ),
                                );
                            }
                        };
                        _serde::__private::Ok(Pawn {
                            dir: __field0,
                            accel: __field1,
                            torque: __field2,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field0: _serde::__private::Option<Vec2> = _serde::__private::None;
                        let mut __field1: _serde::__private::Option<f32> = _serde::__private::None;
                        let mut __field2: _serde::__private::Option<f32> = _serde::__private::None;
                        while let _serde::__private::Some(__key)
                            = _serde::de::MapAccess::next_key::<__Field>(&mut __map)? {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::__private::Option::is_some(&__field0) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("dir"),
                                        );
                                    }
                                    __field0 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<Vec2>(&mut __map)?,
                                    );
                                }
                                __Field::__field1 => {
                                    if _serde::__private::Option::is_some(&__field1) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("accel"),
                                        );
                                    }
                                    __field1 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<f32>(&mut __map)?,
                                    );
                                }
                                __Field::__field2 => {
                                    if _serde::__private::Option::is_some(&__field2) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("torque"),
                                        );
                                    }
                                    __field2 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<f32>(&mut __map)?,
                                    );
                                }
                                _ => {
                                    let _ = _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)?;
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::__private::Some(__field0) => __field0,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("dir")?
                            }
                        };
                        let __field1 = match __field1 {
                            _serde::__private::Some(__field1) => __field1,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("accel")?
                            }
                        };
                        let __field2 = match __field2 {
                            _serde::__private::Some(__field2) => __field2,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("torque")?
                            }
                        };
                        _serde::__private::Ok(Pawn {
                            dir: __field0,
                            accel: __field1,
                            torque: __field2,
                        })
                    }
                }
                #[doc(hidden)]
                const FIELDS: &'static [&'static str] = &["dir", "accel", "torque"];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "Pawn",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<Pawn>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    pub struct PawnBundle {
        pawn: Pawn,
        transform: Transform,
        position: Position,
        rotation: Rotation,
    }
    #[automatically_derived]
    impl ::core::default::Default for PawnBundle {
        #[inline]
        fn default() -> PawnBundle {
            PawnBundle {
                pawn: ::core::default::Default::default(),
                transform: ::core::default::Default::default(),
                position: ::core::default::Default::default(),
                rotation: ::core::default::Default::default(),
            }
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for PawnBundle {
        #[inline]
        fn clone(&self) -> PawnBundle {
            PawnBundle {
                pawn: ::core::clone::Clone::clone(&self.pawn),
                transform: ::core::clone::Clone::clone(&self.transform),
                position: ::core::clone::Clone::clone(&self.position),
                rotation: ::core::clone::Clone::clone(&self.rotation),
            }
        }
    }
    #[allow(deprecated)]
    unsafe impl bevy::ecs::bundle::Bundle for PawnBundle {
        fn component_ids(
            components: &mut bevy::ecs::component::ComponentsRegistrator,
            ids: &mut impl FnMut(bevy::ecs::component::ComponentId),
        ) {
            <Pawn as bevy::ecs::bundle::Bundle>::component_ids(components, &mut *ids);
            <Transform as bevy::ecs::bundle::Bundle>::component_ids(
                components,
                &mut *ids,
            );
            <Position as bevy::ecs::bundle::Bundle>::component_ids(
                components,
                &mut *ids,
            );
            <Rotation as bevy::ecs::bundle::Bundle>::component_ids(
                components,
                &mut *ids,
            );
        }
        fn get_component_ids(
            components: &bevy::ecs::component::Components,
            ids: &mut impl FnMut(Option<bevy::ecs::component::ComponentId>),
        ) {
            <Pawn as bevy::ecs::bundle::Bundle>::get_component_ids(
                components,
                &mut *ids,
            );
            <Transform as bevy::ecs::bundle::Bundle>::get_component_ids(
                components,
                &mut *ids,
            );
            <Position as bevy::ecs::bundle::Bundle>::get_component_ids(
                components,
                &mut *ids,
            );
            <Rotation as bevy::ecs::bundle::Bundle>::get_component_ids(
                components,
                &mut *ids,
            );
        }
        fn register_required_components(
            components: &mut bevy::ecs::component::ComponentsRegistrator,
            required_components: &mut bevy::ecs::component::RequiredComponents,
        ) {
            <Pawn as bevy::ecs::bundle::Bundle>::register_required_components(
                components,
                required_components,
            );
            <Transform as bevy::ecs::bundle::Bundle>::register_required_components(
                components,
                required_components,
            );
            <Position as bevy::ecs::bundle::Bundle>::register_required_components(
                components,
                required_components,
            );
            <Rotation as bevy::ecs::bundle::Bundle>::register_required_components(
                components,
                required_components,
            );
        }
    }
    #[allow(deprecated)]
    unsafe impl bevy::ecs::bundle::BundleFromComponents for PawnBundle {
        #[allow(unused_variables, non_snake_case)]
        unsafe fn from_components<__T, __F>(ctx: &mut __T, func: &mut __F) -> Self
        where
            __F: FnMut(&mut __T) -> bevy::ecs::ptr::OwningPtr<'_>,
        {
            Self {
                pawn: <Pawn as bevy::ecs::bundle::BundleFromComponents>::from_components(
                    ctx,
                    &mut *func,
                ),
                transform: <Transform as bevy::ecs::bundle::BundleFromComponents>::from_components(
                    ctx,
                    &mut *func,
                ),
                position: <Position as bevy::ecs::bundle::BundleFromComponents>::from_components(
                    ctx,
                    &mut *func,
                ),
                rotation: <Rotation as bevy::ecs::bundle::BundleFromComponents>::from_components(
                    ctx,
                    &mut *func,
                ),
            }
        }
    }
    #[allow(deprecated)]
    impl bevy::ecs::bundle::DynamicBundle for PawnBundle {
        type Effect = ();
        #[allow(unused_variables)]
        #[inline]
        fn get_components(
            self,
            func: &mut impl FnMut(
                bevy::ecs::component::StorageType,
                bevy::ecs::ptr::OwningPtr<'_>,
            ),
        ) {
            self.pawn.get_components(&mut *func);
            self.transform.get_components(&mut *func);
            self.position.get_components(&mut *func);
            self.rotation.get_components(&mut *func);
        }
    }
    impl Pawn {
        pub const RADIUS: f32 = 0.16;
        pub const AREA: f32 = Self::RADIUS * Self::RADIUS * PI;
        pub const MAX_ACCELERATION: f32 = 0.68;
        pub const MAX_VELOCITY: f32 = 1.5;
        pub const REVERSE_VELOCITY: f32 = 0.7;
        pub const MAX_TORQUE: f32 = TAU;
        pub const MAX_ANGULAR_VELOCITY: f32 = PI;
        pub const VISION_RADIUS: f32 = 4.;
        pub fn update_movement(&mut self, angle: f32, accel: f32, torque: f32) {
            self.dir = Vec2::from_angle(to_finite_f32_lossy(angle).clamp(-1., 1.) * PI);
            self.accel = to_finite_f32_lossy(accel).clamp(0., 1.);
            self.torque = to_finite_f32_lossy(torque).clamp(-1., 1.);
        }
    }
    impl PawnBundle {
        pub fn new(position: Vec2, rotation: f32) -> Self {
            Self {
                pawn: default(),
                transform: Transform::from_translation(position.extend(0.))
                    .with_rotation(Quat::from_axis_angle(Vec3::Z, rotation)),
                position: Position(position),
                rotation: Rotation::radians(rotation),
            }
        }
    }
    pub fn movement(
        mut pawn_q: Query<
            (
                &Pawn,
                &Rotation,
                &LinearVelocity,
                &AngularVelocity,
                &mut ExternalForce,
                &mut ExternalTorque,
            ),
        >,
    ) {
        pawn_q
            .par_iter_mut()
            .for_each(|
                (
                    pawn,
                    rotation,
                    linear_velocity,
                    angular_velocity,
                    mut force,
                    mut torque,
                )|
            {
                force.persistent = false;
                torque.persistent = false;
                if ::approx::Relative::default().ne(&pawn.dir, &Vec2::ZERO) {
                    let movement_dir = rotation * pawn.dir;
                    force
                        .set_force(
                            movement_dir.normalize() * pawn.accel
                                * Pawn::MAX_ACCELERATION,
                        );
                } else if ::approx::Relative::default()
                    .ne(&linear_velocity.0, &Vec2::ZERO)
                {
                    force
                        .set_force(
                            (-linear_velocity.0).normalize() * Pawn::MAX_ACCELERATION,
                        );
                }
                if ::approx::Relative::default().ne(&pawn.torque, &0.) {
                    torque.apply_torque(pawn.torque * Pawn::MAX_TORQUE);
                } else if ::approx::Relative::default().ne(&angular_velocity.0, &0.) {
                    torque
                        .apply_torque((-angular_velocity.0).signum() * Pawn::MAX_TORQUE);
                }
            });
    }
    pub fn clamp_velocity(
        mut pawn_q: Query<
            (&Rotation, &mut LinearVelocity, &mut AngularVelocity),
            With<Pawn>,
        >,
    ) {
        pawn_q
            .par_iter_mut()
            .for_each(|(rotation, mut linear_velocity, mut angular_velocity)| {
                let mut velocity = linear_velocity.length();
                if ::approx::Relative::default().ne(&velocity, &0.0) {
                    let forward_velocity = rotation.inverse() * linear_velocity.0;
                    let angle_t = forward_velocity.to_angle().abs() / PI;
                    let max_velocity = Pawn::MAX_VELOCITY
                        .lerp(Pawn::MAX_VELOCITY / 2., angle_t);
                    if velocity > max_velocity {
                        linear_velocity.0 *= max_velocity / velocity;
                        velocity = max_velocity;
                    }
                }
                if ::approx::Relative::default().ne(&angular_velocity.0, &0.0) {
                    let max_angular_velocity = Pawn::MAX_ANGULAR_VELOCITY
                        .lerp(
                            Pawn::MAX_ANGULAR_VELOCITY / 2.,
                            velocity / Pawn::MAX_VELOCITY,
                        );
                    angular_velocity
                        .0 = angular_velocity
                        .clamp(-max_angular_velocity, max_angular_velocity);
                }
            });
    }
}
pub mod root {
    use bevy::prelude::*;
    use serde::{Deserialize, Serialize};
    #[reflect(Component, Serialize, Deserialize)]
    #[require(Transform, Visibility, ChildOfRoot, Name::new(Root::type_path()))]
    pub struct Root;
    #[automatically_derived]
    impl ::core::default::Default for Root {
        #[inline]
        fn default() -> Root {
            Root {}
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for Root {}
    #[automatically_derived]
    impl ::core::clone::Clone for Root {
        #[inline]
        fn clone(&self) -> Root {
            *self
        }
    }
    #[doc = "**Required Components**: [`Transform`], [`Visibility`], [`ChildOfRoot`], [`Name`]. \n\n A component's Required Components are inserted whenever it is inserted. Note that this will also insert the required components _of_ the required components, recursively, in depth-first order."]
    impl bevy::ecs::component::Component for Root
    where
        Self: Send + Sync + 'static,
    {
        const STORAGE_TYPE: bevy::ecs::component::StorageType = bevy::ecs::component::StorageType::Table;
        type Mutability = bevy::ecs::component::Mutable;
        fn register_required_components(
            requiree: bevy::ecs::component::ComponentId,
            components: &mut bevy::ecs::component::ComponentsRegistrator,
            required_components: &mut bevy::ecs::component::RequiredComponents,
            inheritance_depth: u16,
            recursion_check_stack: &mut bevy::ecs::__macro_exports::Vec<
                bevy::ecs::component::ComponentId,
            >,
        ) {
            bevy::ecs::component::enforce_no_required_components_recursion(
                components,
                recursion_check_stack,
            );
            let self_id = components.register_component::<Self>();
            recursion_check_stack.push(self_id);
            components
                .register_required_components_manual::<
                    Self,
                    Transform,
                >(
                    required_components,
                    <Transform as Default>::default,
                    inheritance_depth,
                    recursion_check_stack,
                );
            components
                .register_required_components_manual::<
                    Self,
                    Visibility,
                >(
                    required_components,
                    <Visibility as Default>::default,
                    inheritance_depth,
                    recursion_check_stack,
                );
            components
                .register_required_components_manual::<
                    Self,
                    ChildOfRoot,
                >(
                    required_components,
                    <ChildOfRoot as Default>::default,
                    inheritance_depth,
                    recursion_check_stack,
                );
            components
                .register_required_components_manual::<
                    Self,
                    Name,
                >(
                    required_components,
                    || {
                        let x: Name = (|| Name::new(Root::type_path()))().into();
                        x
                    },
                    inheritance_depth,
                    recursion_check_stack,
                );
            <Transform as bevy::ecs::component::Component>::register_required_components(
                requiree,
                components,
                required_components,
                inheritance_depth + 1,
                recursion_check_stack,
            );
            <Visibility as bevy::ecs::component::Component>::register_required_components(
                requiree,
                components,
                required_components,
                inheritance_depth + 1,
                recursion_check_stack,
            );
            <ChildOfRoot as bevy::ecs::component::Component>::register_required_components(
                requiree,
                components,
                required_components,
                inheritance_depth + 1,
                recursion_check_stack,
            );
            <Name as bevy::ecs::component::Component>::register_required_components(
                requiree,
                components,
                required_components,
                inheritance_depth + 1,
                recursion_check_stack,
            );
            recursion_check_stack.pop();
        }
        fn clone_behavior() -> bevy::ecs::component::ComponentCloneBehavior {
            use bevy::ecs::component::{
                DefaultCloneBehaviorBase, DefaultCloneBehaviorViaClone,
            };
            (&&&bevy::ecs::component::DefaultCloneBehaviorSpecialization::<
                Self,
            >::default())
                .default_clone_behavior()
        }
    }
    const _: () = {
        #[allow(unused_mut)]
        impl bevy::reflect::GetTypeRegistration for Root
        where
            Root: ::core::any::Any + ::core::marker::Send + ::core::marker::Sync,
        {
            fn get_type_registration() -> bevy::reflect::TypeRegistration {
                let mut registration = bevy::reflect::TypeRegistration::of::<Self>();
                registration
                    .insert::<
                        bevy::reflect::ReflectFromPtr,
                    >(bevy::reflect::FromType::<Self>::from_type());
                registration
                    .insert::<
                        bevy::reflect::ReflectFromReflect,
                    >(bevy::reflect::FromType::<Self>::from_type());
                registration
                    .insert::<
                        ReflectComponent,
                    >(bevy::reflect::FromType::<Self>::from_type());
                registration
                    .insert::<
                        ReflectSerialize,
                    >(bevy::reflect::FromType::<Self>::from_type());
                registration
                    .insert::<
                        ReflectDeserialize,
                    >(bevy::reflect::FromType::<Self>::from_type());
                registration
            }
            #[inline(never)]
            fn register_type_dependencies(registry: &mut bevy::reflect::TypeRegistry) {}
        }
        impl bevy::reflect::Typed for Root
        where
            Root: ::core::any::Any + ::core::marker::Send + ::core::marker::Sync,
        {
            #[inline]
            fn type_info() -> &'static bevy::reflect::TypeInfo {
                static CELL: bevy::reflect::utility::NonGenericTypeInfoCell = bevy::reflect::utility::NonGenericTypeInfoCell::new();
                CELL.get_or_set(|| {
                    bevy::reflect::TypeInfo::Struct(
                        bevy::reflect::StructInfo::new::<Self>(&[])
                            .with_custom_attributes(
                                bevy::reflect::attributes::CustomAttributes::default(),
                            ),
                    )
                })
            }
        }
        const _: () = {
            extern crate alloc;
            use alloc::string::ToString;
            impl bevy::reflect::TypePath for Root
            where
                Root: ::core::any::Any + ::core::marker::Send + ::core::marker::Sync,
            {
                fn type_path() -> &'static str {
                    "pb_engine::root::Root"
                }
                fn short_type_path() -> &'static str {
                    "Root"
                }
                fn type_ident() -> Option<&'static str> {
                    ::core::option::Option::Some("Root")
                }
                fn crate_name() -> Option<&'static str> {
                    ::core::option::Option::Some(
                        "pb_engine::root".split(':').next().unwrap(),
                    )
                }
                fn module_path() -> Option<&'static str> {
                    ::core::option::Option::Some("pb_engine::root")
                }
            }
        };
        impl bevy::reflect::Reflect for Root
        where
            Root: ::core::any::Any + ::core::marker::Send + ::core::marker::Sync,
        {
            #[inline]
            fn into_any(
                self: bevy::reflect::__macro_exports::alloc_utils::Box<Self>,
            ) -> bevy::reflect::__macro_exports::alloc_utils::Box<dyn ::core::any::Any> {
                self
            }
            #[inline]
            fn as_any(&self) -> &dyn ::core::any::Any {
                self
            }
            #[inline]
            fn as_any_mut(&mut self) -> &mut dyn ::core::any::Any {
                self
            }
            #[inline]
            fn into_reflect(
                self: bevy::reflect::__macro_exports::alloc_utils::Box<Self>,
            ) -> bevy::reflect::__macro_exports::alloc_utils::Box<
                dyn bevy::reflect::Reflect,
            > {
                self
            }
            #[inline]
            fn as_reflect(&self) -> &dyn bevy::reflect::Reflect {
                self
            }
            #[inline]
            fn as_reflect_mut(&mut self) -> &mut dyn bevy::reflect::Reflect {
                self
            }
            #[inline]
            fn set(
                &mut self,
                value: bevy::reflect::__macro_exports::alloc_utils::Box<
                    dyn bevy::reflect::Reflect,
                >,
            ) -> ::core::result::Result<
                (),
                bevy::reflect::__macro_exports::alloc_utils::Box<
                    dyn bevy::reflect::Reflect,
                >,
            > {
                *self = <dyn bevy::reflect::Reflect>::take(value)?;
                ::core::result::Result::Ok(())
            }
        }
        impl bevy::reflect::Struct for Root
        where
            Root: ::core::any::Any + ::core::marker::Send + ::core::marker::Sync,
        {
            fn field(
                &self,
                name: &str,
            ) -> ::core::option::Option<&dyn bevy::reflect::PartialReflect> {
                match name {
                    _ => ::core::option::Option::None,
                }
            }
            fn field_mut(
                &mut self,
                name: &str,
            ) -> ::core::option::Option<&mut dyn bevy::reflect::PartialReflect> {
                match name {
                    _ => ::core::option::Option::None,
                }
            }
            fn field_at(
                &self,
                index: usize,
            ) -> ::core::option::Option<&dyn bevy::reflect::PartialReflect> {
                match index {
                    _ => ::core::option::Option::None,
                }
            }
            fn field_at_mut(
                &mut self,
                index: usize,
            ) -> ::core::option::Option<&mut dyn bevy::reflect::PartialReflect> {
                match index {
                    _ => ::core::option::Option::None,
                }
            }
            fn name_at(&self, index: usize) -> ::core::option::Option<&str> {
                match index {
                    _ => ::core::option::Option::None,
                }
            }
            fn field_len(&self) -> usize {
                0usize
            }
            fn iter_fields(&self) -> bevy::reflect::FieldIter {
                bevy::reflect::FieldIter::new(self)
            }
            fn to_dynamic_struct(&self) -> bevy::reflect::DynamicStruct {
                let mut dynamic: bevy::reflect::DynamicStruct = ::core::default::Default::default();
                dynamic
                    .set_represented_type(
                        bevy::reflect::PartialReflect::get_represented_type_info(self),
                    );
                dynamic
            }
        }
        impl bevy::reflect::PartialReflect for Root
        where
            Root: ::core::any::Any + ::core::marker::Send + ::core::marker::Sync,
        {
            #[inline]
            fn get_represented_type_info(
                &self,
            ) -> ::core::option::Option<&'static bevy::reflect::TypeInfo> {
                ::core::option::Option::Some(<Self as bevy::reflect::Typed>::type_info())
            }
            #[inline]
            fn try_apply(
                &mut self,
                value: &dyn bevy::reflect::PartialReflect,
            ) -> ::core::result::Result<(), bevy::reflect::ApplyError> {
                if let bevy::reflect::ReflectRef::Struct(struct_value)
                    = bevy::reflect::PartialReflect::reflect_ref(value) {
                    for (i, value) in ::core::iter::Iterator::enumerate(
                        bevy::reflect::Struct::iter_fields(struct_value),
                    ) {
                        let name = bevy::reflect::Struct::name_at(struct_value, i)
                            .unwrap();
                        if let ::core::option::Option::Some(v)
                            = bevy::reflect::Struct::field_mut(self, name) {
                            bevy::reflect::PartialReflect::try_apply(v, value)?;
                        }
                    }
                } else {
                    return ::core::result::Result::Err(bevy::reflect::ApplyError::MismatchedKinds {
                        from_kind: bevy::reflect::PartialReflect::reflect_kind(value),
                        to_kind: bevy::reflect::ReflectKind::Struct,
                    });
                }
                ::core::result::Result::Ok(())
            }
            #[inline]
            fn reflect_kind(&self) -> bevy::reflect::ReflectKind {
                bevy::reflect::ReflectKind::Struct
            }
            #[inline]
            fn reflect_ref(&self) -> bevy::reflect::ReflectRef {
                bevy::reflect::ReflectRef::Struct(self)
            }
            #[inline]
            fn reflect_mut(&mut self) -> bevy::reflect::ReflectMut {
                bevy::reflect::ReflectMut::Struct(self)
            }
            #[inline]
            fn reflect_owned(
                self: bevy::reflect::__macro_exports::alloc_utils::Box<Self>,
            ) -> bevy::reflect::ReflectOwned {
                bevy::reflect::ReflectOwned::Struct(self)
            }
            #[inline]
            fn try_into_reflect(
                self: bevy::reflect::__macro_exports::alloc_utils::Box<Self>,
            ) -> ::core::result::Result<
                bevy::reflect::__macro_exports::alloc_utils::Box<
                    dyn bevy::reflect::Reflect,
                >,
                bevy::reflect::__macro_exports::alloc_utils::Box<
                    dyn bevy::reflect::PartialReflect,
                >,
            > {
                ::core::result::Result::Ok(self)
            }
            #[inline]
            fn try_as_reflect(
                &self,
            ) -> ::core::option::Option<&dyn bevy::reflect::Reflect> {
                ::core::option::Option::Some(self)
            }
            #[inline]
            fn try_as_reflect_mut(
                &mut self,
            ) -> ::core::option::Option<&mut dyn bevy::reflect::Reflect> {
                ::core::option::Option::Some(self)
            }
            #[inline]
            fn into_partial_reflect(
                self: bevy::reflect::__macro_exports::alloc_utils::Box<Self>,
            ) -> bevy::reflect::__macro_exports::alloc_utils::Box<
                dyn bevy::reflect::PartialReflect,
            > {
                self
            }
            #[inline]
            fn as_partial_reflect(&self) -> &dyn bevy::reflect::PartialReflect {
                self
            }
            #[inline]
            fn as_partial_reflect_mut(
                &mut self,
            ) -> &mut dyn bevy::reflect::PartialReflect {
                self
            }
            fn reflect_partial_eq(
                &self,
                value: &dyn bevy::reflect::PartialReflect,
            ) -> ::core::option::Option<bool> {
                (bevy::reflect::struct_partial_eq)(self, value)
            }
            #[inline]
            #[allow(
                unreachable_code,
                reason = "Ignored fields without a `clone` attribute will early-return with an error"
            )]
            fn reflect_clone(
                &self,
            ) -> ::core::result::Result<
                bevy::reflect::__macro_exports::alloc_utils::Box<
                    dyn bevy::reflect::Reflect,
                >,
                bevy::reflect::ReflectCloneError,
            > {
                ::core::result::Result::Ok(
                    bevy::reflect::__macro_exports::alloc_utils::Box::new(Self {}),
                )
            }
        }
        impl bevy::reflect::FromReflect for Root
        where
            Root: ::core::any::Any + ::core::marker::Send + ::core::marker::Sync,
        {
            fn from_reflect(
                reflect: &dyn bevy::reflect::PartialReflect,
            ) -> ::core::option::Option<Self> {
                if let bevy::reflect::ReflectRef::Struct(__ref_struct)
                    = bevy::reflect::PartialReflect::reflect_ref(reflect) {
                    let __this = Self {};
                    ::core::option::Option::Some(__this)
                } else {
                    ::core::option::Option::None
                }
            }
        }
    };
    #[doc(hidden)]
    #[allow(
        non_upper_case_globals,
        unused_attributes,
        unused_qualifications,
        clippy::absolute_paths,
    )]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for Root {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                _serde::Serializer::serialize_unit_struct(__serializer, "Root")
            }
        }
    };
    #[doc(hidden)]
    #[allow(
        non_upper_case_globals,
        unused_attributes,
        unused_qualifications,
        clippy::absolute_paths,
    )]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for Root {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[doc(hidden)]
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<Root>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                #[automatically_derived]
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = Root;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "unit struct Root",
                        )
                    }
                    #[inline]
                    fn visit_unit<__E>(
                        self,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(Root)
                    }
                }
                _serde::Deserializer::deserialize_unit_struct(
                    __deserializer,
                    "Root",
                    __Visitor {
                        marker: _serde::__private::PhantomData::<Root>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    pub struct ChildOfRoot;
    #[automatically_derived]
    impl ::core::default::Default for ChildOfRoot {
        #[inline]
        fn default() -> ChildOfRoot {
            ChildOfRoot {}
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for ChildOfRoot {}
    #[automatically_derived]
    impl ::core::clone::Clone for ChildOfRoot {
        #[inline]
        fn clone(&self) -> ChildOfRoot {
            *self
        }
    }
    impl bevy::ecs::component::Component for ChildOfRoot
    where
        Self: Send + Sync + 'static,
    {
        const STORAGE_TYPE: bevy::ecs::component::StorageType = bevy::ecs::component::StorageType::Table;
        type Mutability = bevy::ecs::component::Mutable;
        fn register_required_components(
            requiree: bevy::ecs::component::ComponentId,
            components: &mut bevy::ecs::component::ComponentsRegistrator,
            required_components: &mut bevy::ecs::component::RequiredComponents,
            inheritance_depth: u16,
            recursion_check_stack: &mut bevy::ecs::__macro_exports::Vec<
                bevy::ecs::component::ComponentId,
            >,
        ) {
            bevy::ecs::component::enforce_no_required_components_recursion(
                components,
                recursion_check_stack,
            );
            let self_id = components.register_component::<Self>();
            recursion_check_stack.push(self_id);
            recursion_check_stack.pop();
        }
        fn clone_behavior() -> bevy::ecs::component::ComponentCloneBehavior {
            use bevy::ecs::component::{
                DefaultCloneBehaviorBase, DefaultCloneBehaviorViaClone,
            };
            (&&&bevy::ecs::component::DefaultCloneBehaviorSpecialization::<
                Self,
            >::default())
                .default_clone_behavior()
        }
    }
    pub fn child_added(
        trigger: Trigger<OnInsert, ChildOf>,
        mut commands: Commands,
        parent_q: Query<&ChildOf>,
        root_q: Query<Entity, With<ChildOfRoot>>,
    ) -> Result {
        let parent = parent_q.get(trigger.target())?;
        if root_q.contains(parent.parent()) {
            commands.entity(trigger.target()).insert_recursive::<Children>(ChildOfRoot);
        }
        Ok(())
    }
}
pub mod save {
    use avian2d::prelude::*;
    use bevy::{
        ecs::{entity::EntityHashMap, system::SystemParam},
        prelude::*,
    };
    use glam::Vec2;
    use serde::{Deserialize, Serialize};
    use crate::{
        EngineState, map::{Map, corner::Corner, room::Room, wall::Wall},
        pawn::{Pawn, PawnBundle},
        root::Root,
    };
    pub struct SaveParam<'w, 's> {
        state: Res<'w, State<EngineState>>,
        pawn_q: Query<
            'w,
            's,
            (
                Entity,
                &'static Pawn,
                &'static ChildOf,
                &'static Position,
                &'static Rotation,
                &'static LinearVelocity,
                &'static AngularVelocity,
            ),
        >,
        map_q: Query<'w, 's, (Entity, &'static Map, &'static ChildOf)>,
        corner_q: Query<'w, 's, &'static Corner>,
        wall_q: Query<'w, 's, &'static Wall>,
        room_q: Query<'w, 's, &'static Room>,
    }
    const _: () = {
        type __StructFieldsAlias<'w, 's> = (
            Res<'w, State<EngineState>>,
            Query<
                'w,
                's,
                (
                    Entity,
                    &'static Pawn,
                    &'static ChildOf,
                    &'static Position,
                    &'static Rotation,
                    &'static LinearVelocity,
                    &'static AngularVelocity,
                ),
            >,
            Query<'w, 's, (Entity, &'static Map, &'static ChildOf)>,
            Query<'w, 's, &'static Corner>,
            Query<'w, 's, &'static Wall>,
            Query<'w, 's, &'static Room>,
        );
        #[doc(hidden)]
        pub struct FetchState {
            state: <__StructFieldsAlias<
                'static,
                'static,
            > as bevy::ecs::system::SystemParam>::State,
        }
        unsafe impl bevy::ecs::system::SystemParam for SaveParam<'_, '_> {
            type State = FetchState;
            type Item<'w, 's> = SaveParam<'w, 's>;
            fn init_state(
                world: &mut bevy::ecs::world::World,
                system_meta: &mut bevy::ecs::system::SystemMeta,
            ) -> Self::State {
                FetchState {
                    state: <__StructFieldsAlias<
                        '_,
                        '_,
                    > as bevy::ecs::system::SystemParam>::init_state(world, system_meta),
                }
            }
            unsafe fn new_archetype(
                state: &mut Self::State,
                archetype: &bevy::ecs::archetype::Archetype,
                system_meta: &mut bevy::ecs::system::SystemMeta,
            ) {
                unsafe {
                    <__StructFieldsAlias<
                        '_,
                        '_,
                    > as bevy::ecs::system::SystemParam>::new_archetype(
                        &mut state.state,
                        archetype,
                        system_meta,
                    )
                }
            }
            fn apply(
                state: &mut Self::State,
                system_meta: &bevy::ecs::system::SystemMeta,
                world: &mut bevy::ecs::world::World,
            ) {
                <__StructFieldsAlias<
                    '_,
                    '_,
                > as bevy::ecs::system::SystemParam>::apply(
                    &mut state.state,
                    system_meta,
                    world,
                );
            }
            fn queue(
                state: &mut Self::State,
                system_meta: &bevy::ecs::system::SystemMeta,
                world: bevy::ecs::world::DeferredWorld,
            ) {
                <__StructFieldsAlias<
                    '_,
                    '_,
                > as bevy::ecs::system::SystemParam>::queue(
                    &mut state.state,
                    system_meta,
                    world,
                );
            }
            #[inline]
            unsafe fn validate_param<'w, 's>(
                state: &'s Self::State,
                _system_meta: &bevy::ecs::system::SystemMeta,
                _world: bevy::ecs::world::unsafe_world_cell::UnsafeWorldCell<'w>,
            ) -> Result<(), bevy::ecs::system::SystemParamValidationError> {
                let FetchState { state: (f0, f1, f2, f3, f4, f5) } = state;
                <Res<
                    'w,
                    State<EngineState>,
                > as bevy::ecs::system::SystemParam>::validate_param(
                        f0,
                        _system_meta,
                        _world,
                    )
                    .map_err(|err| bevy::ecs::system::SystemParamValidationError::new::<
                        Self,
                    >(err.skipped, err.message, "::state"))?;
                <Query<
                    'w,
                    's,
                    (
                        Entity,
                        &'static Pawn,
                        &'static ChildOf,
                        &'static Position,
                        &'static Rotation,
                        &'static LinearVelocity,
                        &'static AngularVelocity,
                    ),
                > as bevy::ecs::system::SystemParam>::validate_param(
                        f1,
                        _system_meta,
                        _world,
                    )
                    .map_err(|err| bevy::ecs::system::SystemParamValidationError::new::<
                        Self,
                    >(err.skipped, err.message, "::pawn_q"))?;
                <Query<
                    'w,
                    's,
                    (Entity, &'static Map, &'static ChildOf),
                > as bevy::ecs::system::SystemParam>::validate_param(
                        f2,
                        _system_meta,
                        _world,
                    )
                    .map_err(|err| bevy::ecs::system::SystemParamValidationError::new::<
                        Self,
                    >(err.skipped, err.message, "::map_q"))?;
                <Query<
                    'w,
                    's,
                    &'static Corner,
                > as bevy::ecs::system::SystemParam>::validate_param(
                        f3,
                        _system_meta,
                        _world,
                    )
                    .map_err(|err| bevy::ecs::system::SystemParamValidationError::new::<
                        Self,
                    >(err.skipped, err.message, "::corner_q"))?;
                <Query<
                    'w,
                    's,
                    &'static Wall,
                > as bevy::ecs::system::SystemParam>::validate_param(
                        f4,
                        _system_meta,
                        _world,
                    )
                    .map_err(|err| bevy::ecs::system::SystemParamValidationError::new::<
                        Self,
                    >(err.skipped, err.message, "::wall_q"))?;
                <Query<
                    'w,
                    's,
                    &'static Room,
                > as bevy::ecs::system::SystemParam>::validate_param(
                        f5,
                        _system_meta,
                        _world,
                    )
                    .map_err(|err| bevy::ecs::system::SystemParamValidationError::new::<
                        Self,
                    >(err.skipped, err.message, "::room_q"))?;
                Result::Ok(())
            }
            #[inline]
            unsafe fn get_param<'w, 's>(
                state: &'s mut Self::State,
                system_meta: &bevy::ecs::system::SystemMeta,
                world: bevy::ecs::world::unsafe_world_cell::UnsafeWorldCell<'w>,
                change_tick: bevy::ecs::component::Tick,
            ) -> Self::Item<'w, 's> {
                let (f0, f1, f2, f3, f4, f5) = <(
                    Res<'w, State<EngineState>>,
                    Query<
                        'w,
                        's,
                        (
                            Entity,
                            &'static Pawn,
                            &'static ChildOf,
                            &'static Position,
                            &'static Rotation,
                            &'static LinearVelocity,
                            &'static AngularVelocity,
                        ),
                    >,
                    Query<'w, 's, (Entity, &'static Map, &'static ChildOf)>,
                    Query<'w, 's, &'static Corner>,
                    Query<'w, 's, &'static Wall>,
                    Query<'w, 's, &'static Room>,
                ) as bevy::ecs::system::SystemParam>::get_param(
                    &mut state.state,
                    system_meta,
                    world,
                    change_tick,
                );
                SaveParam {
                    state: f0,
                    pawn_q: f1,
                    map_q: f2,
                    corner_q: f3,
                    wall_q: f4,
                    room_q: f5,
                }
            }
        }
        unsafe impl<'w, 's> bevy::ecs::system::ReadOnlySystemParam for SaveParam<'w, 's>
        where
            Res<'w, State<EngineState>>: bevy::ecs::system::ReadOnlySystemParam,
            Query<
                'w,
                's,
                (
                    Entity,
                    &'static Pawn,
                    &'static ChildOf,
                    &'static Position,
                    &'static Rotation,
                    &'static LinearVelocity,
                    &'static AngularVelocity,
                ),
            >: bevy::ecs::system::ReadOnlySystemParam,
            Query<
                'w,
                's,
                (Entity, &'static Map, &'static ChildOf),
            >: bevy::ecs::system::ReadOnlySystemParam,
            Query<'w, 's, &'static Corner>: bevy::ecs::system::ReadOnlySystemParam,
            Query<'w, 's, &'static Wall>: bevy::ecs::system::ReadOnlySystemParam,
            Query<'w, 's, &'static Room>: bevy::ecs::system::ReadOnlySystemParam,
        {}
    };
    pub struct SaveModel {
        pub pawns: Vec<PawnModel>,
        pub maps: Vec<MapModel>,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for SaveModel {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field2_finish(
                f,
                "SaveModel",
                "pawns",
                &self.pawns,
                "maps",
                &&self.maps,
            )
        }
    }
    #[doc(hidden)]
    #[allow(
        non_upper_case_globals,
        unused_attributes,
        unused_qualifications,
        clippy::absolute_paths,
    )]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for SaveModel {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = _serde::Serializer::serialize_struct(
                    __serializer,
                    "SaveModel",
                    false as usize + 1 + 1,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "pawns",
                    &self.pawns,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "maps",
                    &self.maps,
                )?;
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    #[doc(hidden)]
    #[allow(
        non_upper_case_globals,
        unused_attributes,
        unused_qualifications,
        clippy::absolute_paths,
    )]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for SaveModel {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                #[doc(hidden)]
                enum __Field {
                    __field0,
                    __field1,
                    __ignore,
                }
                #[doc(hidden)]
                struct __FieldVisitor;
                #[automatically_derived]
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "field identifier",
                        )
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private::Ok(__Field::__field0),
                            1u64 => _serde::__private::Ok(__Field::__field1),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "pawns" => _serde::__private::Ok(__Field::__field0),
                            "maps" => _serde::__private::Ok(__Field::__field1),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"pawns" => _serde::__private::Ok(__Field::__field0),
                            b"maps" => _serde::__private::Ok(__Field::__field1),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                }
                #[automatically_derived]
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(
                            __deserializer,
                            __FieldVisitor,
                        )
                    }
                }
                #[doc(hidden)]
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<SaveModel>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                #[automatically_derived]
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = SaveModel;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "struct SaveModel",
                        )
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = match _serde::de::SeqAccess::next_element::<
                            Vec<PawnModel>,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        0usize,
                                        &"struct SaveModel with 2 elements",
                                    ),
                                );
                            }
                        };
                        let __field1 = match _serde::de::SeqAccess::next_element::<
                            Vec<MapModel>,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        1usize,
                                        &"struct SaveModel with 2 elements",
                                    ),
                                );
                            }
                        };
                        _serde::__private::Ok(SaveModel {
                            pawns: __field0,
                            maps: __field1,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field0: _serde::__private::Option<Vec<PawnModel>> = _serde::__private::None;
                        let mut __field1: _serde::__private::Option<Vec<MapModel>> = _serde::__private::None;
                        while let _serde::__private::Some(__key)
                            = _serde::de::MapAccess::next_key::<__Field>(&mut __map)? {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::__private::Option::is_some(&__field0) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("pawns"),
                                        );
                                    }
                                    __field0 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<
                                            Vec<PawnModel>,
                                        >(&mut __map)?,
                                    );
                                }
                                __Field::__field1 => {
                                    if _serde::__private::Option::is_some(&__field1) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("maps"),
                                        );
                                    }
                                    __field1 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<
                                            Vec<MapModel>,
                                        >(&mut __map)?,
                                    );
                                }
                                _ => {
                                    let _ = _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)?;
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::__private::Some(__field0) => __field0,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("pawns")?
                            }
                        };
                        let __field1 = match __field1 {
                            _serde::__private::Some(__field1) => __field1,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("maps")?
                            }
                        };
                        _serde::__private::Ok(SaveModel {
                            pawns: __field0,
                            maps: __field1,
                        })
                    }
                }
                #[doc(hidden)]
                const FIELDS: &'static [&'static str] = &["pawns", "maps"];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "SaveModel",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<SaveModel>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    const _: () = {
        const _: () = {
            extern crate alloc;
            use alloc::string::ToString;
            impl bevy::reflect::TypePath for SaveModel
            where
                SaveModel: ::core::any::Any + ::core::marker::Send
                    + ::core::marker::Sync,
            {
                fn type_path() -> &'static str {
                    "pb_engine::save::SaveModel"
                }
                fn short_type_path() -> &'static str {
                    "SaveModel"
                }
                fn type_ident() -> Option<&'static str> {
                    ::core::option::Option::Some("SaveModel")
                }
                fn crate_name() -> Option<&'static str> {
                    ::core::option::Option::Some(
                        "pb_engine::save".split(':').next().unwrap(),
                    )
                }
                fn module_path() -> Option<&'static str> {
                    ::core::option::Option::Some("pb_engine::save")
                }
            }
        };
    };
    pub struct PawnModel {
        pub id: Entity,
        pub position: Vec2,
        pub rotation: f32,
        pub linear_velocity: Vec2,
        pub angular_velocity: f32,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for PawnModel {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field5_finish(
                f,
                "PawnModel",
                "id",
                &self.id,
                "position",
                &self.position,
                "rotation",
                &self.rotation,
                "linear_velocity",
                &self.linear_velocity,
                "angular_velocity",
                &&self.angular_velocity,
            )
        }
    }
    #[doc(hidden)]
    #[allow(
        non_upper_case_globals,
        unused_attributes,
        unused_qualifications,
        clippy::absolute_paths,
    )]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for PawnModel {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = _serde::Serializer::serialize_struct(
                    __serializer,
                    "PawnModel",
                    false as usize + 1 + 1 + 1 + 1 + 1,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "id",
                    &self.id,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "position",
                    &self.position,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "rotation",
                    &self.rotation,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "linear_velocity",
                    &self.linear_velocity,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "angular_velocity",
                    &self.angular_velocity,
                )?;
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    #[doc(hidden)]
    #[allow(
        non_upper_case_globals,
        unused_attributes,
        unused_qualifications,
        clippy::absolute_paths,
    )]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for PawnModel {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                #[doc(hidden)]
                enum __Field {
                    __field0,
                    __field1,
                    __field2,
                    __field3,
                    __field4,
                    __ignore,
                }
                #[doc(hidden)]
                struct __FieldVisitor;
                #[automatically_derived]
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "field identifier",
                        )
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private::Ok(__Field::__field0),
                            1u64 => _serde::__private::Ok(__Field::__field1),
                            2u64 => _serde::__private::Ok(__Field::__field2),
                            3u64 => _serde::__private::Ok(__Field::__field3),
                            4u64 => _serde::__private::Ok(__Field::__field4),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "id" => _serde::__private::Ok(__Field::__field0),
                            "position" => _serde::__private::Ok(__Field::__field1),
                            "rotation" => _serde::__private::Ok(__Field::__field2),
                            "linear_velocity" => _serde::__private::Ok(__Field::__field3),
                            "angular_velocity" => {
                                _serde::__private::Ok(__Field::__field4)
                            }
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"id" => _serde::__private::Ok(__Field::__field0),
                            b"position" => _serde::__private::Ok(__Field::__field1),
                            b"rotation" => _serde::__private::Ok(__Field::__field2),
                            b"linear_velocity" => {
                                _serde::__private::Ok(__Field::__field3)
                            }
                            b"angular_velocity" => {
                                _serde::__private::Ok(__Field::__field4)
                            }
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                }
                #[automatically_derived]
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(
                            __deserializer,
                            __FieldVisitor,
                        )
                    }
                }
                #[doc(hidden)]
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<PawnModel>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                #[automatically_derived]
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = PawnModel;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "struct PawnModel",
                        )
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = match _serde::de::SeqAccess::next_element::<
                            Entity,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        0usize,
                                        &"struct PawnModel with 5 elements",
                                    ),
                                );
                            }
                        };
                        let __field1 = match _serde::de::SeqAccess::next_element::<
                            Vec2,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        1usize,
                                        &"struct PawnModel with 5 elements",
                                    ),
                                );
                            }
                        };
                        let __field2 = match _serde::de::SeqAccess::next_element::<
                            f32,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        2usize,
                                        &"struct PawnModel with 5 elements",
                                    ),
                                );
                            }
                        };
                        let __field3 = match _serde::de::SeqAccess::next_element::<
                            Vec2,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        3usize,
                                        &"struct PawnModel with 5 elements",
                                    ),
                                );
                            }
                        };
                        let __field4 = match _serde::de::SeqAccess::next_element::<
                            f32,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        4usize,
                                        &"struct PawnModel with 5 elements",
                                    ),
                                );
                            }
                        };
                        _serde::__private::Ok(PawnModel {
                            id: __field0,
                            position: __field1,
                            rotation: __field2,
                            linear_velocity: __field3,
                            angular_velocity: __field4,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field0: _serde::__private::Option<Entity> = _serde::__private::None;
                        let mut __field1: _serde::__private::Option<Vec2> = _serde::__private::None;
                        let mut __field2: _serde::__private::Option<f32> = _serde::__private::None;
                        let mut __field3: _serde::__private::Option<Vec2> = _serde::__private::None;
                        let mut __field4: _serde::__private::Option<f32> = _serde::__private::None;
                        while let _serde::__private::Some(__key)
                            = _serde::de::MapAccess::next_key::<__Field>(&mut __map)? {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::__private::Option::is_some(&__field0) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("id"),
                                        );
                                    }
                                    __field0 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<Entity>(&mut __map)?,
                                    );
                                }
                                __Field::__field1 => {
                                    if _serde::__private::Option::is_some(&__field1) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "position",
                                            ),
                                        );
                                    }
                                    __field1 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<Vec2>(&mut __map)?,
                                    );
                                }
                                __Field::__field2 => {
                                    if _serde::__private::Option::is_some(&__field2) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "rotation",
                                            ),
                                        );
                                    }
                                    __field2 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<f32>(&mut __map)?,
                                    );
                                }
                                __Field::__field3 => {
                                    if _serde::__private::Option::is_some(&__field3) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "linear_velocity",
                                            ),
                                        );
                                    }
                                    __field3 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<Vec2>(&mut __map)?,
                                    );
                                }
                                __Field::__field4 => {
                                    if _serde::__private::Option::is_some(&__field4) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "angular_velocity",
                                            ),
                                        );
                                    }
                                    __field4 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<f32>(&mut __map)?,
                                    );
                                }
                                _ => {
                                    let _ = _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)?;
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::__private::Some(__field0) => __field0,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("id")?
                            }
                        };
                        let __field1 = match __field1 {
                            _serde::__private::Some(__field1) => __field1,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("position")?
                            }
                        };
                        let __field2 = match __field2 {
                            _serde::__private::Some(__field2) => __field2,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("rotation")?
                            }
                        };
                        let __field3 = match __field3 {
                            _serde::__private::Some(__field3) => __field3,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("linear_velocity")?
                            }
                        };
                        let __field4 = match __field4 {
                            _serde::__private::Some(__field4) => __field4,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("angular_velocity")?
                            }
                        };
                        _serde::__private::Ok(PawnModel {
                            id: __field0,
                            position: __field1,
                            rotation: __field2,
                            linear_velocity: __field3,
                            angular_velocity: __field4,
                        })
                    }
                }
                #[doc(hidden)]
                const FIELDS: &'static [&'static str] = &[
                    "id",
                    "position",
                    "rotation",
                    "linear_velocity",
                    "angular_velocity",
                ];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "PawnModel",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<PawnModel>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    pub struct MapModel {
        pub id: Entity,
        pub corners: Vec<CornerModel>,
        pub walls: Vec<WallModel>,
        pub rooms: Vec<RoomModel>,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for MapModel {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field4_finish(
                f,
                "MapModel",
                "id",
                &self.id,
                "corners",
                &self.corners,
                "walls",
                &self.walls,
                "rooms",
                &&self.rooms,
            )
        }
    }
    #[doc(hidden)]
    #[allow(
        non_upper_case_globals,
        unused_attributes,
        unused_qualifications,
        clippy::absolute_paths,
    )]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for MapModel {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = _serde::Serializer::serialize_struct(
                    __serializer,
                    "MapModel",
                    false as usize + 1 + 1 + 1 + 1,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "id",
                    &self.id,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "corners",
                    &self.corners,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "walls",
                    &self.walls,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "rooms",
                    &self.rooms,
                )?;
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    #[doc(hidden)]
    #[allow(
        non_upper_case_globals,
        unused_attributes,
        unused_qualifications,
        clippy::absolute_paths,
    )]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for MapModel {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                #[doc(hidden)]
                enum __Field {
                    __field0,
                    __field1,
                    __field2,
                    __field3,
                    __ignore,
                }
                #[doc(hidden)]
                struct __FieldVisitor;
                #[automatically_derived]
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "field identifier",
                        )
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private::Ok(__Field::__field0),
                            1u64 => _serde::__private::Ok(__Field::__field1),
                            2u64 => _serde::__private::Ok(__Field::__field2),
                            3u64 => _serde::__private::Ok(__Field::__field3),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "id" => _serde::__private::Ok(__Field::__field0),
                            "corners" => _serde::__private::Ok(__Field::__field1),
                            "walls" => _serde::__private::Ok(__Field::__field2),
                            "rooms" => _serde::__private::Ok(__Field::__field3),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"id" => _serde::__private::Ok(__Field::__field0),
                            b"corners" => _serde::__private::Ok(__Field::__field1),
                            b"walls" => _serde::__private::Ok(__Field::__field2),
                            b"rooms" => _serde::__private::Ok(__Field::__field3),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                }
                #[automatically_derived]
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(
                            __deserializer,
                            __FieldVisitor,
                        )
                    }
                }
                #[doc(hidden)]
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<MapModel>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                #[automatically_derived]
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = MapModel;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "struct MapModel",
                        )
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = match _serde::de::SeqAccess::next_element::<
                            Entity,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        0usize,
                                        &"struct MapModel with 4 elements",
                                    ),
                                );
                            }
                        };
                        let __field1 = match _serde::de::SeqAccess::next_element::<
                            Vec<CornerModel>,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        1usize,
                                        &"struct MapModel with 4 elements",
                                    ),
                                );
                            }
                        };
                        let __field2 = match _serde::de::SeqAccess::next_element::<
                            Vec<WallModel>,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        2usize,
                                        &"struct MapModel with 4 elements",
                                    ),
                                );
                            }
                        };
                        let __field3 = match _serde::de::SeqAccess::next_element::<
                            Vec<RoomModel>,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        3usize,
                                        &"struct MapModel with 4 elements",
                                    ),
                                );
                            }
                        };
                        _serde::__private::Ok(MapModel {
                            id: __field0,
                            corners: __field1,
                            walls: __field2,
                            rooms: __field3,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field0: _serde::__private::Option<Entity> = _serde::__private::None;
                        let mut __field1: _serde::__private::Option<Vec<CornerModel>> = _serde::__private::None;
                        let mut __field2: _serde::__private::Option<Vec<WallModel>> = _serde::__private::None;
                        let mut __field3: _serde::__private::Option<Vec<RoomModel>> = _serde::__private::None;
                        while let _serde::__private::Some(__key)
                            = _serde::de::MapAccess::next_key::<__Field>(&mut __map)? {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::__private::Option::is_some(&__field0) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("id"),
                                        );
                                    }
                                    __field0 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<Entity>(&mut __map)?,
                                    );
                                }
                                __Field::__field1 => {
                                    if _serde::__private::Option::is_some(&__field1) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "corners",
                                            ),
                                        );
                                    }
                                    __field1 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<
                                            Vec<CornerModel>,
                                        >(&mut __map)?,
                                    );
                                }
                                __Field::__field2 => {
                                    if _serde::__private::Option::is_some(&__field2) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("walls"),
                                        );
                                    }
                                    __field2 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<
                                            Vec<WallModel>,
                                        >(&mut __map)?,
                                    );
                                }
                                __Field::__field3 => {
                                    if _serde::__private::Option::is_some(&__field3) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("rooms"),
                                        );
                                    }
                                    __field3 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<
                                            Vec<RoomModel>,
                                        >(&mut __map)?,
                                    );
                                }
                                _ => {
                                    let _ = _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)?;
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::__private::Some(__field0) => __field0,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("id")?
                            }
                        };
                        let __field1 = match __field1 {
                            _serde::__private::Some(__field1) => __field1,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("corners")?
                            }
                        };
                        let __field2 = match __field2 {
                            _serde::__private::Some(__field2) => __field2,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("walls")?
                            }
                        };
                        let __field3 = match __field3 {
                            _serde::__private::Some(__field3) => __field3,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("rooms")?
                            }
                        };
                        _serde::__private::Ok(MapModel {
                            id: __field0,
                            corners: __field1,
                            walls: __field2,
                            rooms: __field3,
                        })
                    }
                }
                #[doc(hidden)]
                const FIELDS: &'static [&'static str] = &[
                    "id",
                    "corners",
                    "walls",
                    "rooms",
                ];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "MapModel",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<MapModel>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    pub struct CornerModel {
        pub id: Entity,
        pub position: Vec2,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for CornerModel {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field2_finish(
                f,
                "CornerModel",
                "id",
                &self.id,
                "position",
                &&self.position,
            )
        }
    }
    #[doc(hidden)]
    #[allow(
        non_upper_case_globals,
        unused_attributes,
        unused_qualifications,
        clippy::absolute_paths,
    )]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for CornerModel {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = _serde::Serializer::serialize_struct(
                    __serializer,
                    "CornerModel",
                    false as usize + 1 + 1,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "id",
                    &self.id,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "position",
                    &self.position,
                )?;
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    #[doc(hidden)]
    #[allow(
        non_upper_case_globals,
        unused_attributes,
        unused_qualifications,
        clippy::absolute_paths,
    )]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for CornerModel {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                #[doc(hidden)]
                enum __Field {
                    __field0,
                    __field1,
                    __ignore,
                }
                #[doc(hidden)]
                struct __FieldVisitor;
                #[automatically_derived]
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "field identifier",
                        )
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private::Ok(__Field::__field0),
                            1u64 => _serde::__private::Ok(__Field::__field1),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "id" => _serde::__private::Ok(__Field::__field0),
                            "position" => _serde::__private::Ok(__Field::__field1),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"id" => _serde::__private::Ok(__Field::__field0),
                            b"position" => _serde::__private::Ok(__Field::__field1),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                }
                #[automatically_derived]
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(
                            __deserializer,
                            __FieldVisitor,
                        )
                    }
                }
                #[doc(hidden)]
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<CornerModel>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                #[automatically_derived]
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = CornerModel;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "struct CornerModel",
                        )
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = match _serde::de::SeqAccess::next_element::<
                            Entity,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        0usize,
                                        &"struct CornerModel with 2 elements",
                                    ),
                                );
                            }
                        };
                        let __field1 = match _serde::de::SeqAccess::next_element::<
                            Vec2,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        1usize,
                                        &"struct CornerModel with 2 elements",
                                    ),
                                );
                            }
                        };
                        _serde::__private::Ok(CornerModel {
                            id: __field0,
                            position: __field1,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field0: _serde::__private::Option<Entity> = _serde::__private::None;
                        let mut __field1: _serde::__private::Option<Vec2> = _serde::__private::None;
                        while let _serde::__private::Some(__key)
                            = _serde::de::MapAccess::next_key::<__Field>(&mut __map)? {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::__private::Option::is_some(&__field0) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("id"),
                                        );
                                    }
                                    __field0 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<Entity>(&mut __map)?,
                                    );
                                }
                                __Field::__field1 => {
                                    if _serde::__private::Option::is_some(&__field1) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "position",
                                            ),
                                        );
                                    }
                                    __field1 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<Vec2>(&mut __map)?,
                                    );
                                }
                                _ => {
                                    let _ = _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)?;
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::__private::Some(__field0) => __field0,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("id")?
                            }
                        };
                        let __field1 = match __field1 {
                            _serde::__private::Some(__field1) => __field1,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("position")?
                            }
                        };
                        _serde::__private::Ok(CornerModel {
                            id: __field0,
                            position: __field1,
                        })
                    }
                }
                #[doc(hidden)]
                const FIELDS: &'static [&'static str] = &["id", "position"];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "CornerModel",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<CornerModel>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    pub struct WallModel {
        pub id: Entity,
        pub corners: [Entity; 2],
        pub rooms: [Entity; 2],
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for WallModel {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field3_finish(
                f,
                "WallModel",
                "id",
                &self.id,
                "corners",
                &self.corners,
                "rooms",
                &&self.rooms,
            )
        }
    }
    #[doc(hidden)]
    #[allow(
        non_upper_case_globals,
        unused_attributes,
        unused_qualifications,
        clippy::absolute_paths,
    )]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for WallModel {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = _serde::Serializer::serialize_struct(
                    __serializer,
                    "WallModel",
                    false as usize + 1 + 1 + 1,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "id",
                    &self.id,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "corners",
                    &self.corners,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "rooms",
                    &self.rooms,
                )?;
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    #[doc(hidden)]
    #[allow(
        non_upper_case_globals,
        unused_attributes,
        unused_qualifications,
        clippy::absolute_paths,
    )]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for WallModel {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                #[doc(hidden)]
                enum __Field {
                    __field0,
                    __field1,
                    __field2,
                    __ignore,
                }
                #[doc(hidden)]
                struct __FieldVisitor;
                #[automatically_derived]
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "field identifier",
                        )
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private::Ok(__Field::__field0),
                            1u64 => _serde::__private::Ok(__Field::__field1),
                            2u64 => _serde::__private::Ok(__Field::__field2),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "id" => _serde::__private::Ok(__Field::__field0),
                            "corners" => _serde::__private::Ok(__Field::__field1),
                            "rooms" => _serde::__private::Ok(__Field::__field2),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"id" => _serde::__private::Ok(__Field::__field0),
                            b"corners" => _serde::__private::Ok(__Field::__field1),
                            b"rooms" => _serde::__private::Ok(__Field::__field2),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                }
                #[automatically_derived]
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(
                            __deserializer,
                            __FieldVisitor,
                        )
                    }
                }
                #[doc(hidden)]
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<WallModel>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                #[automatically_derived]
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = WallModel;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "struct WallModel",
                        )
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = match _serde::de::SeqAccess::next_element::<
                            Entity,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        0usize,
                                        &"struct WallModel with 3 elements",
                                    ),
                                );
                            }
                        };
                        let __field1 = match _serde::de::SeqAccess::next_element::<
                            [Entity; 2],
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        1usize,
                                        &"struct WallModel with 3 elements",
                                    ),
                                );
                            }
                        };
                        let __field2 = match _serde::de::SeqAccess::next_element::<
                            [Entity; 2],
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        2usize,
                                        &"struct WallModel with 3 elements",
                                    ),
                                );
                            }
                        };
                        _serde::__private::Ok(WallModel {
                            id: __field0,
                            corners: __field1,
                            rooms: __field2,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field0: _serde::__private::Option<Entity> = _serde::__private::None;
                        let mut __field1: _serde::__private::Option<[Entity; 2]> = _serde::__private::None;
                        let mut __field2: _serde::__private::Option<[Entity; 2]> = _serde::__private::None;
                        while let _serde::__private::Some(__key)
                            = _serde::de::MapAccess::next_key::<__Field>(&mut __map)? {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::__private::Option::is_some(&__field0) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("id"),
                                        );
                                    }
                                    __field0 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<Entity>(&mut __map)?,
                                    );
                                }
                                __Field::__field1 => {
                                    if _serde::__private::Option::is_some(&__field1) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "corners",
                                            ),
                                        );
                                    }
                                    __field1 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<
                                            [Entity; 2],
                                        >(&mut __map)?,
                                    );
                                }
                                __Field::__field2 => {
                                    if _serde::__private::Option::is_some(&__field2) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("rooms"),
                                        );
                                    }
                                    __field2 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<
                                            [Entity; 2],
                                        >(&mut __map)?,
                                    );
                                }
                                _ => {
                                    let _ = _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)?;
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::__private::Some(__field0) => __field0,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("id")?
                            }
                        };
                        let __field1 = match __field1 {
                            _serde::__private::Some(__field1) => __field1,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("corners")?
                            }
                        };
                        let __field2 = match __field2 {
                            _serde::__private::Some(__field2) => __field2,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("rooms")?
                            }
                        };
                        _serde::__private::Ok(WallModel {
                            id: __field0,
                            corners: __field1,
                            rooms: __field2,
                        })
                    }
                }
                #[doc(hidden)]
                const FIELDS: &'static [&'static str] = &["id", "corners", "rooms"];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "WallModel",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<WallModel>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    pub struct RoomModel {
        pub id: Entity,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for RoomModel {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field1_finish(
                f,
                "RoomModel",
                "id",
                &&self.id,
            )
        }
    }
    #[doc(hidden)]
    #[allow(
        non_upper_case_globals,
        unused_attributes,
        unused_qualifications,
        clippy::absolute_paths,
    )]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for RoomModel {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = _serde::Serializer::serialize_struct(
                    __serializer,
                    "RoomModel",
                    false as usize + 1,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "id",
                    &self.id,
                )?;
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    #[doc(hidden)]
    #[allow(
        non_upper_case_globals,
        unused_attributes,
        unused_qualifications,
        clippy::absolute_paths,
    )]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for RoomModel {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                #[doc(hidden)]
                enum __Field {
                    __field0,
                    __ignore,
                }
                #[doc(hidden)]
                struct __FieldVisitor;
                #[automatically_derived]
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "field identifier",
                        )
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private::Ok(__Field::__field0),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "id" => _serde::__private::Ok(__Field::__field0),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"id" => _serde::__private::Ok(__Field::__field0),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                }
                #[automatically_derived]
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(
                            __deserializer,
                            __FieldVisitor,
                        )
                    }
                }
                #[doc(hidden)]
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<RoomModel>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                #[automatically_derived]
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = RoomModel;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "struct RoomModel",
                        )
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = match _serde::de::SeqAccess::next_element::<
                            Entity,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        0usize,
                                        &"struct RoomModel with 1 element",
                                    ),
                                );
                            }
                        };
                        _serde::__private::Ok(RoomModel { id: __field0 })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field0: _serde::__private::Option<Entity> = _serde::__private::None;
                        while let _serde::__private::Some(__key)
                            = _serde::de::MapAccess::next_key::<__Field>(&mut __map)? {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::__private::Option::is_some(&__field0) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("id"),
                                        );
                                    }
                                    __field0 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<Entity>(&mut __map)?,
                                    );
                                }
                                _ => {
                                    let _ = _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)?;
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::__private::Some(__field0) => __field0,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("id")?
                            }
                        };
                        _serde::__private::Ok(RoomModel { id: __field0 })
                    }
                }
                #[doc(hidden)]
                const FIELDS: &'static [&'static str] = &["id"];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "RoomModel",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<RoomModel>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    impl SaveParam<'_, '_> {
        pub fn save(&self) -> Result<SaveModel> {
            let &EngineState::Running(root) = self.state.get() else {
                return Err("no active game".into());
            };
            let pawns = self
                .pawn_q
                .iter()
                .filter(|(_, _, parent, _, _, _, _)| parent.parent() == root)
                .map(|(id, _, _, position, rotation, linear_velocity, angular_velocity)| PawnModel {
                    id,
                    position: position.0,
                    rotation: rotation.as_radians(),
                    linear_velocity: linear_velocity.0,
                    angular_velocity: angular_velocity.0,
                })
                .collect();
            let maps = self
                .map_q
                .iter()
                .filter(|(_, _, parent)| parent.parent() == root)
                .map(|(id, map, _)| {
                    let corners = map
                        .corners()
                        .map(|id| {
                            let corner = self.corner_q.get(id.id())?;
                            Ok(CornerModel {
                                id: id.id(),
                                position: corner.position(),
                            })
                        })
                        .collect::<Result<Vec<_>>>()?;
                    let walls = map
                        .walls()
                        .map(|id| {
                            let wall = self.wall_q.get(id.id())?;
                            Ok(WallModel {
                                id: id.id(),
                                corners: wall.corners(),
                                rooms: map.wall_rooms(wall),
                            })
                        })
                        .collect::<Result<Vec<_>>>()?;
                    let rooms = map
                        .rooms_deduped()
                        .map(|id| {
                            let _room = self.room_q.get(id.id())?;
                            Ok(RoomModel { id: id.id() })
                        })
                        .collect::<Result<Vec<_>>>()?;
                    Ok(MapModel {
                        id,
                        corners,
                        walls,
                        rooms,
                    })
                })
                .collect::<Result<Vec<_>>>()?;
            Ok(SaveModel { pawns, maps })
        }
    }
    impl SaveModel {
        pub fn spawn(self, commands: &mut Commands) -> Entity {
            let root = commands.spawn(Root).id();
            let mut entity_map = EntityHashMap::<Entity>::new();
            commands
                .queue(move |world: &mut World| -> Result {
                    for (entity, pawn) in world
                        .spawn_batch(
                            self
                                .pawns
                                .iter()
                                .map(|pawn| (
                                    PawnBundle::new(pawn.position, pawn.rotation),
                                    ChildOf(root),
                                )),
                        )
                        .zip(&self.pawns)
                    {
                        entity_map.insert(pawn.id, entity);
                    }
                    for map in &self.maps {
                        let map_id = world.spawn(ChildOf(root)).id();
                        for (entity, corner) in world
                            .spawn_batch(map.corners.iter().map(|_| ChildOf(map_id)))
                            .zip(&map.corners)
                        {
                            entity_map.insert(corner.id, entity);
                        }
                        for (entity, room) in world
                            .spawn_batch(map.rooms.iter().map(|_| ChildOf(map_id)))
                            .zip(&map.rooms)
                        {
                            entity_map.insert(room.id, entity);
                        }
                        for (entity, wall) in world
                            .spawn_batch(map.walls.iter().map(|_| ChildOf(map_id)))
                            .zip(&map.walls)
                        {
                            entity_map.insert(wall.id, entity);
                        }
                        world
                            .entity_mut(map_id)
                            .insert(Map::from_model(map, &mut entity_map)?);
                    }
                    Ok(())
                });
            root
        }
    }
}
use avian2d::{
    dynamics::{integrator::IntegrationSet, solver::schedule::SubstepSolverSet},
    prelude::*,
};
use bevy::prelude::*;
use dev::DevSettings;
use pawn::{Pawn, ai::path::PathQueryConfig};
use pb_util::event::AddComponentEvent;
use root::Root;
pub enum EngineState {
    #[default]
    Disabled,
    Running(Entity),
}
#[automatically_derived]
impl ::core::clone::Clone for EngineState {
    #[inline]
    fn clone(&self) -> EngineState {
        let _: ::core::clone::AssertParamIsClone<Entity>;
        *self
    }
}
#[automatically_derived]
impl ::core::marker::Copy for EngineState {}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for EngineState {}
#[automatically_derived]
impl ::core::cmp::PartialEq for EngineState {
    #[inline]
    fn eq(&self, other: &EngineState) -> bool {
        let __self_discr = ::core::intrinsics::discriminant_value(self);
        let __arg1_discr = ::core::intrinsics::discriminant_value(other);
        __self_discr == __arg1_discr
            && match (self, other) {
                (EngineState::Running(__self_0), EngineState::Running(__arg1_0)) => {
                    __self_0 == __arg1_0
                }
                _ => true,
            }
    }
}
#[automatically_derived]
impl ::core::cmp::Eq for EngineState {
    #[inline]
    #[doc(hidden)]
    #[coverage(off)]
    fn assert_receiver_is_total_eq(&self) -> () {
        let _: ::core::cmp::AssertParamIsEq<Entity>;
    }
}
#[automatically_derived]
impl ::core::hash::Hash for EngineState {
    #[inline]
    fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
        let __self_discr = ::core::intrinsics::discriminant_value(self);
        ::core::hash::Hash::hash(&__self_discr, state);
        match self {
            EngineState::Running(__self_0) => ::core::hash::Hash::hash(__self_0, state),
            _ => {}
        }
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for EngineState {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match self {
            EngineState::Disabled => ::core::fmt::Formatter::write_str(f, "Disabled"),
            EngineState::Running(__self_0) => {
                ::core::fmt::Formatter::debug_tuple_field1_finish(
                    f,
                    "Running",
                    &__self_0,
                )
            }
        }
    }
}
#[automatically_derived]
impl ::core::default::Default for EngineState {
    #[inline]
    fn default() -> EngineState {
        Self::Disabled
    }
}
impl bevy::state::state::States for EngineState {
    const SCOPED_ENTITIES_ENABLED: bool = false;
}
impl bevy::state::state::FreelyMutableState for EngineState {}
pub struct PbEnginePlugin;
impl Plugin for PbEnginePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Root>().register_type::<Pawn>();
        app.init_state::<EngineState>();
        app.add_plugins(PhysicsPlugins::default());
        app.insert_resource(Gravity::ZERO);
        app.init_resource::<PathQueryConfig>().init_resource::<DevSettings>();
        app.add_observer(root::child_added)
            .add_observer(map::map_inserted)
            .add_observer(map::room::room_replaced)
            .add_observer(map::door::wall_replaced)
            .add_insert_event::<map::corner::Corner>()
            .add_insert_event::<map::wall::Wall>()
            .add_insert_event::<map::perimeter::Perimeter>()
            .add_insert_event::<map::door::Door>()
            .add_observer(pawn::ai::task_added)
            .add_observer(pawn::ai::task_removed)
            .add_observer(pawn::ai::actor_removed)
            .add_systems(
                FixedPreUpdate,
                (
                    map::door::validate,
                    map::door::remove_links,
                    map::wall::add_colliders.after(map::door::validate),
                    map::door::add_links
                        .after(map::door::validate)
                        .after(map::door::remove_links),
                    map::corner::add_colliders,
                    map::perimeter::add_colliders,
                    map::mesh::update_mesh,
                    map::room::update_containing_room,
                ),
            )
            .add_systems(FixedUpdate, (pawn::ai::path::update, pawn::movement).chain())
            .add_systems(
                SubstepSchedule,
                pawn::clamp_velocity
                    .after(SubstepSolverSet::SolveConstraints)
                    .before(IntegrationSet::Position),
            )
            .add_systems(
                Update,
                (
                    dev::draw_meshes.run_if(dev::draw_meshes_condition),
                    dev::draw_paths.run_if(dev::draw_paths_condition),
                ),
            );
    }
}
