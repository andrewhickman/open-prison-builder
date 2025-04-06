use std::{f32::consts::PI, time::Duration};

use avian2d::prelude::*;
use bevy::{prelude::*, scene::ScenePlugin, state::app::StatesPlugin, time::TimeUpdateStrategy};
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
        &'static Position,
        &'static Rotation,
        &'static LinearVelocity,
        &'static AngularVelocity,
        &'static TargetPosition,
    )>,
    entities: Vec<Entity>,
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

pub struct Step<B: Backend> {
    pub state: Tensor<B, 2>,
    pub reward: Tensor<B, 1>,
    pub done: Tensor<B, 1, Bool>,
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

        Environment {
            app,
            rng,
            query,
            entities: Vec::new(),
        }
    }

    pub fn action_space(&self) -> usize {
        Action::SIZE
    }

    pub fn observation_space(&self) -> &[usize] {
        static OBSERVATION_SPACE: [usize; 1] = [Observation::SIZE];
        OBSERVATION_SPACE.as_ref()
    }

    pub fn entity_count(&self) -> usize {
        self.entities.len()
    }

    pub fn reset(&mut self) -> Vec<Observation> {
        for entity in self.entities.drain(..) {
            self.app.world_mut().entity_mut(entity).despawn();
        }

        let mut position: Vec2 = self.rng.random::<[f32; 2]>().into();
        position *= 5.;
        let rotation = self.rng.random_range(-PI..PI);
        let mut target: Vec2 = self.rng.random::<[f32; 2]>().into();
        target *= 5.;

        let linear_velocity_angle = self.rng.random_range(-PI..PI);
        let max_velocity = MAX_VELOCITY.lerp(MAX_VELOCITY / 2., linear_velocity_angle.abs() / PI);
        let linear_velocity = Vec2::from_angle(rotation + linear_velocity_angle)
            * self.rng.random_range(0.0..max_velocity);

        let max_angular_velocity = MAX_ANGULAR_VELOCITY.lerp(
            MAX_ANGULAR_VELOCITY / 2.,
            linear_velocity.length() / MAX_VELOCITY,
        );
        let angular_velocity = self
            .rng
            .random_range(-max_angular_velocity..max_angular_velocity);

        let entity = self
            .app
            .world_mut()
            .spawn(PawnBundle::new(Vec2::ZERO))
            .insert((
                Rotation::radians(rotation),
                LinearVelocity(linear_velocity),
                AngularVelocity(angular_velocity),
                TargetPosition(target),
            ))
            .id();
        self.entities.push(entity);
        self.observe()
    }

    pub fn step<B: Backend>(&mut self, device: &Device<B>, actions: &[Action]) -> Step<B> {
        let prev_state = self.observe();

        for (&entity, action) in self.entities.iter().zip(actions) {
            self.app.world_mut().entity_mut(entity).insert((
                ExternalForce::new(action.force).with_persistence(false),
                ExternalTorque::new(action.torque),
            ));
        }

        self.app.update();

        let state = self.observe();

        Step {
            state: Observation::to_tensor(&state, device),
            reward: rewards(device, &prev_state, &state),
            done: done(device, &prev_state, &state),
        }
    }

    pub fn observe(&mut self) -> Vec<Observation> {
        let world = self.app.world();
        self.entities
            .iter()
            .map(|&entity| {
                let (position, rotation, linear_velocity, angular_velocity, target) =
                    self.query.get(world, entity).unwrap();
                Observation {
                    rotation: Vec2::new(rotation.cos, rotation.sin),
                    linear_velocity: linear_velocity.0,
                    angular_velocity: angular_velocity.0,
                    target: target.0 - position.0,
                }
            })
            .collect()
    }
}

impl Action {
    pub const SIZE: usize = 3;

    pub fn from_tensor<B>(actions: &Tensor<B, 2>) -> Vec<Self>
    where
        B: Backend,
    {
        let data = actions.to_data();
        assert_eq!(data.num_elements() % Self::SIZE, 0);

        fn normalize(f: f32) -> f32 {
            if !f.is_finite() {
                0.
            } else {
                f
            }
        }

        data.as_slice::<f32>()
            .unwrap()
            .chunks_exact(Self::SIZE)
            .map(|chunk| Action {
                force: Vec2::new(
                    normalize(chunk[0] * MAX_ACCELERATION),
                    normalize(chunk[1] * MAX_ACCELERATION),
                ),
                torque: normalize(chunk[2] * MAX_TORQUE),
            })
            .collect()
    }
}

impl Observation {
    pub const SIZE: usize = 7;

    pub fn to_tensor<B>(obs: &[Self], device: &Device<B>) -> Tensor<B, 2>
    where
        B: Backend,
    {
        Tensor::from_floats(
            TensorData::new(
                obs.iter()
                    .flat_map(|obs| {
                        [
                            obs.rotation.x,
                            obs.rotation.y,
                            obs.linear_velocity.x,
                            obs.linear_velocity.y,
                            obs.angular_velocity,
                            obs.target.x,
                            obs.target.y,
                        ]
                    })
                    .collect(),
                [obs.len(), Observation::SIZE],
            ),
            device,
        )
    }
}

fn rewards<B: Backend>(
    device: &Device<B>,
    prev: &[Observation],
    curr: &[Observation],
) -> Tensor<B, 1> {
    assert_eq!(prev.len(), curr.len());
    Tensor::from_floats(
        prev.iter()
            .zip(curr.iter())
            .map(|(prev, curr)| reward(prev, curr).unwrap_or(0.))
            .collect::<Vec<_>>()
            .as_slice(),
        device,
    )
}

fn done<B: Backend>(
    device: &Device<B>,
    prev: &[Observation],
    curr: &[Observation],
) -> Tensor<B, 1, Bool> {
    assert_eq!(prev.len(), curr.len());
    Tensor::from_bool(
        prev.iter()
            .zip(curr.iter())
            .map(|(prev, curr)| reward(prev, curr).is_none())
            .collect::<Vec<_>>()
            .as_slice()
            .into(),
        device,
    )
}

fn reward(prev: &Observation, curr: &Observation) -> Option<f32> {
    if curr.target.length() < 0.1 {
        return None;
    }
    // if relative_eq!(curr.target, Vec2::ZERO) {
    //     return None;
    // }

    let delta = prev.target - curr.target;

    let on_target = delta.project_onto(prev.target);
    let off_target = delta - on_target;

    Some((on_target.length() - off_target.length()) / (MAX_VELOCITY * TIMESTEP.as_secs_f32()))
}
