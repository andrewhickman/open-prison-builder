use std::{
    f32::consts::{PI, TAU},
    time::Duration,
};

use approx::relative_eq;
use avian2d::prelude::*;
use bevy::{
    ecs::entity::EntityHashMap, prelude::*, scene::ScenePlugin, state::app::StatesPlugin,
    time::TimeUpdateStrategy,
};
use candle::{Device, Tensor};
use pb_engine::{
    pawn::{PawnBundle, MAX_ANGULAR_VELOCITY, MAX_VELOCITY},
    PbEnginePlugin,
};
use rand::{distr::Uniform, rngs::SmallRng, Rng, SeedableRng};

pub struct Environment {
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
    pub force: Vec2,
    pub torque: f32,
}

#[derive(Copy, Clone, Debug)]
pub struct Observation {
    pub position: Vec2,
    pub rotation: Vec2,
    pub linear_velocity: Vec2,
    pub angular_velocity: f32,
    pub target: Vec2,
}

const TIMESTEP: Duration = Duration::from_micros(15625);

impl Environment {
    pub fn new() -> Self {
        let mut app = App::new();

        app.add_plugins((
            MinimalPlugins,
            TransformPlugin,
            HierarchyPlugin,
            AssetPlugin {
                file_path: concat!(env!("CARGO_MANIFEST_DIR"), "/../assets").to_owned(),
                ..default()
            },
            StatesPlugin,
            ScenePlugin,
            PbEnginePlugin,
        ));

        app.insert_resource(TimeUpdateStrategy::ManualDuration(TIMESTEP));
        app.insert_resource(Time::<Fixed>::from_duration(TIMESTEP));

        app.finish();
        app.cleanup();
        app.update();

        let rng = SmallRng::from_os_rng();
        let query = app.world_mut().query();

        Environment { app, rng, query }
    }

    pub fn reset(&mut self) -> EntityHashMap<Observation> {
        for (entity, observation) in self.observe() {
            self.app.world_mut().entity_mut(entity).despawn();
        }

        let position: Vec2 = self.rng.random::<[f32; 2]>().into() * 10.;
        let rotation = self.rng.random_range(-PI..PI);
        let target: Vec2 = self.rng.random::<[f32; 2]>().into() * 10.;

        let linear_velocity_angle = self.rng.random_range(-PI..PI);
        let max_velocity = MAX_VELOCITY.lerp(MAX_VELOCITY / 2., linear_velocity_angle);
        let linear_velocity =
            Vec2::from_angle(linear_velocity_angle) * self.rng.random_range(0.0..max_velocity);

        let max_angular_velocity = MAX_ANGULAR_VELOCITY.lerp(
            MAX_ANGULAR_VELOCITY / 2.,
            linear_velocity.length() / MAX_VELOCITY,
        );
        let angular_velocity = self
            .rng
            .random_range(-max_angular_velocity..max_angular_velocity);

        self.app
            .world_mut()
            .spawn(PawnBundle::new(Vec2::ZERO))
            .insert((
                Rotation::radians(rotation),
                LinearVelocity(linear_velocity),
                AngularVelocity(angular_velocity),
                TargetPosition(target),
            ));
        self.observe()
    }

    pub fn step(&mut self, actions: &EntityHashMap<Action>) -> EntityHashMap<Observation> {
        for (&entity, action) in actions {
            self.app.world_mut().entity_mut(entity).insert((
                ExternalForce::new(action.force).with_persistence(false),
                ExternalTorque::new(action.torque),
            ));
        }

        self.app.update();
        self.observe()
    }

    pub fn observe(&mut self) -> EntityHashMap<Observation> {
        self.query
            .iter(self.app.world())
            .map(
                |(entity, position, rotation, linear_velocity, angular_velocity, target)| {
                    (
                        entity,
                        Observation {
                            position: position.0,
                            rotation: Vec2::new(rotation.cos, rotation.sin),
                            linear_velocity: linear_velocity.0,
                            angular_velocity: angular_velocity.0,
                            target: target.0,
                        },
                    )
                },
            )
            .collect()
    }
}

impl Action {
    pub const SIZE: usize = 3;

    pub fn from_tensor(tensor: &Tensor) -> Self {
        let data = tensor.to_vec1().unwrap();
        assert_eq!(data.len(), Self::SIZE);

        Action {
            force: Vec2::new(data[0], data[1]),
            torque: data[2],
        }
    }
}

impl Observation {
    pub const SIZE: usize = 9;

    pub fn to_tensor(&self, device: &Device) -> Tensor {
        Tensor::from_slice(
            &[
                self.position.x,
                self.position.y,
                self.rotation.x,
                self.rotation.y,
                self.linear_velocity.x,
                self.linear_velocity.y,
                self.angular_velocity,
                self.target.x,
                self.target.y,
            ],
            Self::SIZE,
            device,
        )
        .unwrap()
    }
}

pub fn all_rewards(prev: &EntityHashMap<Observation>, curr: &EntityHashMap<Observation>) -> Option<f32> {
    curr.keys()
        .map(|entity| reward(&prev[entity], &curr[entity]))
        .fold(None, |a, b| match (a, b) {
            (None, None) => None,
            (Some(a), None) | (None, Some(a)) => Some(a),
            (Some(a), Some(b)) => Some(a + b),
        })
}

pub fn reward(prev: &Observation, curr: &Observation) -> Option<f32> {
    if relative_eq!(curr.position, prev.position) {
        return None;
    }

    let target = prev.target - prev.position;
    let delta = curr.position - prev.position;

    let on_target = delta.project_onto(target);
    let off_target = delta - on_target;

    Some((on_target.length() - off_target.length()) / (MAX_VELOCITY * TIMESTEP.as_secs_f32()))
}
