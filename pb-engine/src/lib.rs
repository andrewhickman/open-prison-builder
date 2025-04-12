#![allow(clippy::type_complexity, clippy::too_many_arguments)]

pub mod build;
pub mod map;
pub mod pawn;
pub mod picking;
pub mod root;
pub mod save;
pub mod wall;

use avian2d::{
    dynamics::{integrator::IntegrationSet, solver::schedule::SubstepSolverSet},
    prelude::*,
};
use bevy::prelude::*;
use build::Blueprint;
use pawn::Pawn;
use root::Root;
use vleue_navigator::prelude::*;
use wall::{Vertex, Wall, WallMap};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default, States)]
pub enum EngineState {
    #[default]
    Disabled,
    Running(Entity),
}

pub struct PbEnginePlugin;

impl Plugin for PbEnginePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<WallMap>();

        app.register_type::<Root>()
            .register_type::<Blueprint>()
            .register_type::<Pawn>()
            .register_type::<Wall>()
            .register_type::<Vertex>();

        app.init_state::<EngineState>();

        app.add_plugins((
            PhysicsPlugins::default(),
            VleueNavigatorPlugin,
            NavmeshUpdaterPlugin::<Collider, Wall>::default(),
        ));

        app.insert_resource(Gravity::ZERO);

        app.add_observer(wall::wall_added)
            .add_observer(wall::wall_removed)
            .add_observer(map::map_added)
            .add_systems(Update, wall::add_colliders)
            .add_systems(
                SubstepSchedule,
                pawn::clamp_velocity
                    .after(SubstepSolverSet::SolveConstraints)
                    .before(IntegrationSet::Position),
            )
            .add_systems(FixedUpdate, pawn::movement);

        #[cfg(feature = "dev")]
        app.add_plugins(PhysicsDebugPlugin::default());

        #[cfg(feature = "dev")]
        app.insert_resource(NavMeshesDebug(
            bevy::color::palettes::tailwind::RED_800.into(),
        ));
    }
}
