#![allow(clippy::type_complexity, clippy::too_many_arguments)]

pub mod dev;
pub mod map;
pub mod pawn;
pub mod picking;
pub mod root;
pub mod save;

use avian2d::{
    dynamics::{integrator::IntegrationSet, solver::schedule::SubstepSolverSet},
    prelude::*,
};
use bevy::prelude::*;
use dev::DevSettings;
use pawn::{Pawn, ai::path::PathQueryConfig};
use pb_util::event::AddComponentEvents;
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

        app.init_resource::<PathQueryConfig>()
            .init_resource::<DevSettings>();

        app.add_observer(map::map_inserted)
            .add_observer(map::room::room_replaced)
            .add_inserted_event::<map::corner::Corner>()
            .add_inserted_event::<map::wall::Wall>()
            .add_inserted_event::<map::door::Door>()
            .add_observer(pawn::ai::task_added)
            .add_observer(pawn::ai::task_removed)
            .add_observer(pawn::ai::actor_removed)
            .add_systems(
                FixedPreUpdate,
                (
                    (map::door::validate, map::wall::add_colliders).chain(),
                    map::corner::add_colliders,
                    map::mesh::update_mesh,
                    map::room::update_containing_room,
                ),
            )
            .add_systems(
                FixedUpdate,
                (pawn::ai::path::update, pawn::movement).chain(),
            )
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

        #[cfg(feature = "dev")]
        app.add_plugins(PhysicsDebugPlugin::default());
    }
}
