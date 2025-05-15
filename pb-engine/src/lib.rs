#![allow(clippy::type_complexity, clippy::too_many_arguments)]

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

        app.init_resource::<PathQueryConfig>();

        app.add_observer(map::map_inserted)
            .add_inserted_event::<map::Wall>()
            .add_inserted_event::<map::Room>()
            .add_observer(pawn::ai::task_added)
            .add_observer(pawn::ai::task_removed)
            .add_observer(pawn::ai::actor_removed)
            .add_systems(
                FixedPreUpdate,
                (map::wall::add_colliders, map::room::update_mesh),
            )
            .add_systems(
                SubstepSchedule,
                pawn::clamp_velocity
                    .after(SubstepSolverSet::SolveConstraints)
                    .before(IntegrationSet::Position),
            )
            .add_systems(
                FixedUpdate,
                (pawn::ai::path::update, pawn::movement).chain(),
            );

        #[cfg(feature = "dev")]
        app.add_systems(Update, pawn::ai::path::debug_draw_path);

        #[cfg(feature = "dev")]
        app.add_plugins(PhysicsDebugPlugin::default());
    }
}
