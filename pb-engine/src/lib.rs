#![allow(clippy::type_complexity, clippy::too_many_arguments)]

pub mod dev;
pub mod layer;
pub mod map;
pub mod pawn;
pub mod root;
pub mod save;

use avian2d::prelude::*;
use bevy::prelude::*;
use dev::DevSettings;
use pawn::Pawn;
use pb_util::event::AddComponentEvent;
use root::Root;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default, States)]
pub enum EngineState {
    #[default]
    Disabled,
    Running(Entity),
}

pub struct PbEnginePlugin;

impl Plugin for PbEnginePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Root>().register_type::<Pawn>();

        app.init_state::<EngineState>();

        app.add_plugins(PhysicsPlugins::default());

        app.insert_resource(Gravity::ZERO);

        app.init_resource::<DevSettings>();

        app.add_observer(root::child_added)
            .add_observer(map::map_inserted)
            .add_observer(map::room::contents::room_replaced)
            .add_observer(map::door::wall_replaced)
            .add_insert_event::<map::corner::Corner>()
            .add_insert_event::<map::wall::Wall>()
            .add_insert_event::<map::perimeter::Perimeter>()
            .add_insert_event::<map::door::Door>()
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
                    map::room::mesh::update,
                    map::room::contents::update,
                    map::room::paths::update
                        .after(map::room::mesh::update)
                        .after(map::room::contents::update),
                ),
            )
            // .add_systems(FixedUpdate, ())
            .add_systems(
                Update,
                (
                    dev::draw_meshes.run_if(dev::draw_meshes_condition),
                    dev::draw_paths.run_if(dev::draw_paths_condition),
                ),
            );

        #[cfg(feature = "dev")]
        app.add_plugins(PhysicsDebugPlugin::default());
    }
}
