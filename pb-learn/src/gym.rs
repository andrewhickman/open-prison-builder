use std::time::Duration;

use avian2d::prelude::*;
use bevy::{prelude::*, scene::ScenePlugin, state::app::StatesPlugin, time::TimeUpdateStrategy};
use pb_engine::{pawn::PawnBundle, PbEnginePlugin};
use rand::{rngs::SmallRng, Rng, SeedableRng};

pub struct Gym {
    app: App,
    rng: SmallRng,
    query: QueryState<(
        Entity,
        &'static Position,
        &'static Rotation,
        &'static LinearVelocity,
        &'static AngularVelocity,
        &'static TargetPosition,
    )>,
}

#[derive(Copy, Clone, Debug, Component)]
pub struct TargetPosition(pub Vec2);

#[derive(Copy, Clone, Debug)]
pub struct Action {
    pub entity: Entity,
    pub force: Vec2,
    pub torque: Vec2,
}

#[derive(Copy, Clone, Debug)]
pub struct Observation {
    pub entity: Entity,
    pub position: Position,
    pub rotation: Rotation,
    pub linear_velocity: LinearVelocity,
    pub angular_velocity: AngularVelocity,
    pub target: TargetPosition,
}

impl Gym {
    pub fn new() -> Self {
        let mut app = App::new();

        app.add_plugins((
            MinimalPlugins,
            AssetPlugin {
                file_path: concat!(env!("CARGO_MANIFEST_DIR"), "/../assets").to_owned(),
                ..default()
            },
            StatesPlugin,
            ScenePlugin,
            PbEnginePlugin,
        ));

        let timestep = Duration::from_micros(15625);
        app.insert_resource(TimeUpdateStrategy::ManualDuration(timestep));
        app.insert_resource(Time::<Fixed>::from_duration(timestep));

        app.finish();
        app.cleanup();
        app.update();

        let rng = SmallRng::from_os_rng();
        let query = app.world_mut().query();

        Gym { app, rng, query }
    }

    pub fn reset(&mut self) {
        let mut entities = Vec::new();
        self.observe(&mut entities);
        for observation in entities {
            self.app
                .world_mut()
                .entity_mut(observation.entity)
                .despawn();
        }

        let position: Vec2 = self.rng.random::<[f32; 2]>().into();
        let target: Vec2 = self.rng.random::<[f32; 2]>().into();

        self.app
            .world_mut()
            .spawn(PawnBundle::new(Vec2::ZERO))
            .insert(TargetPosition(target));
    }

    pub fn step(&mut self, actions: &[Action]) {
        for action in actions {
            self.app.world_mut().entity_mut(action.entity).insert((
                ExternalForce::new(action.force).with_persistence(false),
                ExternalTorque::new(action.torque.to_angle()),
            ));
        }

        self.app.update();
    }

    pub fn observe(&mut self, result: &mut Vec<Observation>) {
        result.clear();
        result.extend(self.query.iter(&self.app.world()).map(
            |(entity, position, rotation, linear_velocity, angular_velocity, target)| Observation {
                entity,
                position: *position,
                rotation: *rotation,
                linear_velocity: *linear_velocity,
                angular_velocity: *angular_velocity,
                target: *target,
            },
        ))
    }
}
