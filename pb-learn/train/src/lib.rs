use std::{f32::consts::PI, time::Duration};

use avian2d::prelude::*;
use bevy::{prelude::*, scene::ScenePlugin, state::app::StatesPlugin, time::TimeUpdateStrategy};
use pb_engine::{
    pawn::{Pawn, PawnBundle, MAX_ANGULAR_VELOCITY, MAX_VELOCITY},
    PbEnginePlugin,
};
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
    pub force: Vec2,
}

#[derive(Copy, Clone, Debug)]
pub struct Observation {
    pub linear_velocity: Vec2,
    pub angular_velocity: f32,
    pub target: Vec2,
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
        position = (position - Vec2::splat(0.5)) * 5.;
        let rotation = self.rng.random_range(-PI..PI);
        let mut target: Vec2 = self.rng.random::<[f32; 2]>().into();
        target = (target - Vec2::splat(0.5)) * 5.;

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

        let terminated = observation.target.length() < 0.1;
        let truncated = observation.target.x.abs() > 10. || observation.target.y.abs() > 10.;

        let reward = if terminated {
            self.rng.random_range(1.0..1.5)
        } else {
            (prev_observation.target.length() - observation.target.length())
                / (MAX_VELOCITY * TIMESTEP.as_secs_f32())
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
            .movement = Vec2::new(normalize(action.force.x), normalize(action.force.y));

        self.app.update();
    }

    fn observe(&mut self) -> Observation {
        let (position, rotation, linear_velocity, angular_velocity, target) =
            self.query.get(self.app.world(), self.entity).unwrap();
        let inv_isometry = Isometry2d::new(position.0, (*rotation).into()).inverse();

        let pawn_space_target = inv_isometry * target.0;
        let pawn_space_linear_velocity = inv_isometry * linear_velocity.0;

        Observation {
            linear_velocity: pawn_space_linear_velocity,
            angular_velocity: angular_velocity.0,
            target: pawn_space_target,
        }
    }
}

impl Action {
    pub const SIZE: usize = 2;
}

impl From<[f32; Self::SIZE]> for Action {
    fn from([x, y]: [f32; Self::SIZE]) -> Self {
        Action {
            force: Vec2::new(x, y),
        }
    }
}

impl Observation {
    pub const SIZE: usize = 5;
}

impl From<Observation> for [f32; Observation::SIZE] {
    fn from(obs: Observation) -> [f32; Observation::SIZE] {
        [
            obs.linear_velocity.x,
            obs.linear_velocity.y,
            obs.angular_velocity,
            obs.target.x,
            obs.target.y,
        ]
    }
}

impl Default for Environment {
    fn default() -> Self {
        Self::new()
    }
}

fn normalize(f: f32) -> f32 {
    if f.is_finite() {
        f
    } else {
        0.
    }
}

// HACK
unsafe impl Send for Environment {}

unsafe impl Sync for Environment {}
