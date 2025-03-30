use std::{f32::consts::PI, time::Duration};

use approx::relative_eq;
use avian2d::prelude::*;
use bevy::{
    ecs::entity::EntityHashMap, prelude::*, scene::ScenePlugin, state::app::StatesPlugin,
    time::TimeUpdateStrategy,
};
use burn::prelude::*;
use pb_engine::{
    pawn::{PawnBundle, MAX_ACCELERATION, MAX_ANGULAR_VELOCITY, MAX_TORQUE, MAX_VELOCITY},
    PbEnginePlugin,
};
use rand::{rngs::SmallRng, Rng, SeedableRng};

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
    pub rotation: Vec2,
    pub linear_velocity: Vec2,
    pub angular_velocity: f32,
    pub target: Vec2,
}

const TIMESTEP: Duration = Duration::from_micros(15625);

impl Environment {
    pub fn new(seed: Option<u64>) -> Self {
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

        let rng = match seed {
            Some(seed) => SmallRng::seed_from_u64(seed),
            None => SmallRng::from_os_rng(),
        };
        let query = app.world_mut().query();

        Environment { app, rng, query }
    }

    pub fn action_space(&self) -> usize {
        Action::SIZE
    }

    pub fn observation_space(&self) -> &[usize] {
        static OBSERVATION_SPACE: [usize; 1] = [Observation::SIZE];
        OBSERVATION_SPACE.as_ref()
    }

    pub fn reset(&mut self) -> EntityHashMap<Observation> {
        for (entity, _) in self.observe() {
            self.app.world_mut().entity_mut(entity).despawn();
        }

        let mut position: Vec2 = self.rng.random::<[f32; 2]>().into();
        position *= 10.;
        let rotation = self.rng.random_range(-PI..PI);
        let mut target: Vec2 = self.rng.random::<[f32; 2]>().into();
        target *= 10.;

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
                            rotation: Vec2::new(rotation.cos, rotation.sin),
                            linear_velocity: linear_velocity.0,
                            angular_velocity: angular_velocity.0,
                            target: target.0 - position.0,
                        },
                    )
                },
            )
            .collect()
    }
}

impl Action {
    pub const SIZE: usize = 3;

    pub fn from_tensor<B>(tensor: &Tensor<B, 1>) -> Self
    where
        B: Backend,
    {
        let data = tensor.to_data();
        let data = data.as_slice().unwrap();
        assert_eq!(data.len(), Self::SIZE);

        Action {
            force: Vec2::new(
                f32::lerp(-MAX_ACCELERATION, MAX_ACCELERATION, data[0]),
                f32::lerp(-MAX_ACCELERATION, MAX_ACCELERATION, data[1]),
            ),
            torque: f32::lerp(-MAX_TORQUE, MAX_TORQUE, data[2]),
        }
    }
}

impl Observation {
    pub const SIZE: usize = 7;

    pub fn to_tensor<B>(&self, device: &Device<B>) -> Tensor<B, 1>
    where
        B: Backend,
    {
        Tensor::from_floats(
            [
                self.rotation.x,
                self.rotation.y,
                self.linear_velocity.x,
                self.linear_velocity.y,
                self.angular_velocity,
                self.target.x,
                self.target.y,
            ]
            .as_slice(),
            device,
        )
    }
}

pub fn all_rewards(
    prev: &EntityHashMap<Observation>,
    curr: &EntityHashMap<Observation>,
) -> Option<f32> {
    curr.keys()
        .map(|entity| reward(&prev[entity], &curr[entity]))
        .fold(None, |a, b| match (a, b) {
            (None, None) => None,
            (Some(a), None) | (None, Some(a)) => Some(a),
            (Some(a), Some(b)) => Some(a + b),
        })
}

pub fn reward(prev: &Observation, curr: &Observation) -> Option<f32> {
    if relative_eq!(curr.target, Vec2::ZERO) {
        return None;
    }

    let delta = curr.target - prev.target;

    let on_target = delta.project_onto(prev.target);
    let off_target = delta - on_target;

    Some((on_target.length() - off_target.length()) / (MAX_VELOCITY * TIMESTEP.as_secs_f32()))
}
