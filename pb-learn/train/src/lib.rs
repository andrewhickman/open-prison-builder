use std::{f32::consts::PI, time::Duration};

use avian2d::prelude::*;
use bevy::{prelude::*, scene::ScenePlugin, state::app::StatesPlugin, time::TimeUpdateStrategy};
use pb_engine::{
    pawn::{Pawn, PawnBundle, MAX_ANGULAR_VELOCITY, MAX_VELOCITY},
    PbEnginePlugin,
};
use pb_util::math::normalize_angle;
use pyo3::prelude::*;
use rand::{rngs::SmallRng, Rng, SeedableRng};

#[pymodule(name = "pb_learn_env")]
pub fn pb_learn(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Environment>()?;
    Ok(())
}

#[pyclass]
pub struct Environment {
    app: App,
    rng: SmallRng,
    entity: Entity,
    query: QueryState<(
        &'static Position,
        &'static Rotation,
        &'static LinearVelocity,
        &'static AngularVelocity,
        &'static TargetPosition,
    )>,
}

#[derive(Copy, Clone, Debug)]
pub struct Action {
    pub angle: f32,
    pub torque: f32,
}

#[derive(Copy, Clone, Debug)]
pub struct Observation {
    pub linear_velocity_t: f32,
    pub linear_velocity_r: f32,
    pub angular_velocity: f32,
    pub target_t: f32,
    pub target_r: f32,
}

#[derive(Copy, Clone, Debug, Component)]
pub struct TargetPosition(pub Vec2);

const TIMESTEP: Duration = Duration::from_micros(15625);

#[pymethods]
impl Environment {
    #[new]
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

        let query = app.world_mut().query();

        Environment {
            app,
            rng: SmallRng::from_os_rng(),
            entity: Entity::PLACEHOLDER,
            query,
        }
    }

    pub fn reset(&mut self, seed: Option<u64>) -> [f32; Observation::SIZE] {
        if let Some(seed) = seed {
            self.rng = SmallRng::seed_from_u64(seed);
        }

        if self.entity != Entity::PLACEHOLDER {
            self.app.world_mut().entity_mut(self.entity).despawn();
        }

        let mut position: Vec2 = self.rng.random::<[f32; 2]>().into();
        position = (position - Vec2::splat(0.5)) * 10.;
        let rotation = self.rng.random_range(-PI..PI);

        let target: Vec2 = self.rng.random::<[f32; 2]>().into();
        let target = if self.rng.random_bool(0.5) {
            (target - Vec2::splat(0.5)) * 10.
        } else {
            position + (target - Vec2::splat(0.5))
        };

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

        self.entity = self
            .app
            .world_mut()
            .spawn((
                PawnBundle::new(position, rotation),
                LinearVelocity(linear_velocity),
                AngularVelocity(angular_velocity),
                TargetPosition(target),
            ))
            .id();
        self.observe().into()
    }

    pub fn step(
        &mut self,
        action: [f32; Action::SIZE],
    ) -> ([f32; Observation::SIZE], f32, bool, bool) {
        let action = Action::from(action);
        let prev_observation = self.observe();

        self.act(action);

        let observation = self.observe();

        let terminated = observation.target_r < (MAX_VELOCITY * TIMESTEP.as_secs_f32());
        let truncated = observation.target_r > 150.;

        let reward = if terminated {
            self.rng.random_range(2.0..5.0)
        } else {
            let dist_reward = (prev_observation.target_r - observation.target_r)
                / (MAX_VELOCITY * TIMESTEP.as_secs_f32());
            let velocity_reward =
                normalize_angle(observation.linear_velocity_t - observation.target_t).cos()
                    * (observation.linear_velocity_r / MAX_VELOCITY);
            let rotation_penalty = observation.target_t.abs() / PI;
            let angular_velocity_penalty = observation.angular_velocity / MAX_ANGULAR_VELOCITY;

            dist_reward + velocity_reward * 0.7
                - rotation_penalty * 0.5
                - angular_velocity_penalty * 0.5
        };

        (observation.into(), reward, terminated, truncated)
    }
}

impl Environment {
    fn act(&mut self, action: Action) {
        self.app
            .world_mut()
            .entity_mut(self.entity)
            .get_mut::<Pawn>()
            .unwrap()
            .update_movement(action.angle, 1., action.torque);

        self.app.update();
    }

    fn observe(&mut self) -> Observation {
        let (position, rotation, linear_velocity, angular_velocity, target) =
            self.query.get(self.app.world(), self.entity).unwrap();
        let inv_isometry = Isometry2d::new(position.0, (*rotation).into()).inverse();

        let pawn_space_target = inv_isometry * target.0;
        let pawn_space_linear_velocity = inv_isometry.rotation * linear_velocity.0;

        Observation {
            linear_velocity_t: pawn_space_linear_velocity.to_angle(),
            linear_velocity_r: pawn_space_linear_velocity.length_squared()
                / (MAX_VELOCITY * MAX_VELOCITY),
            angular_velocity: angular_velocity.0 / MAX_ANGULAR_VELOCITY,
            target_t: pawn_space_target.to_angle(),
            target_r: pawn_space_target.length(),
        }
    }
}

impl Action {
    pub const SIZE: usize = 2;
}

impl From<[f32; Self::SIZE]> for Action {
    fn from([a, t]: [f32; Self::SIZE]) -> Self {
        Action {
            angle: a,
            torque: t,
        }
    }
}

impl Observation {
    pub const SIZE: usize = 5;
}

impl From<Observation> for [f32; Observation::SIZE] {
    fn from(obs: Observation) -> [f32; Observation::SIZE] {
        [
            obs.linear_velocity_t,
            obs.linear_velocity_r,
            obs.angular_velocity,
            obs.target_t,
            obs.target_r.min(10.),
        ]
    }
}

impl Default for Environment {
    fn default() -> Self {
        Self::new()
    }
}

// HACK
unsafe impl Send for Environment {}

unsafe impl Sync for Environment {}
