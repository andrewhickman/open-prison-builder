#![allow(clippy::type_complexity, clippy::too_many_arguments)]

use std::{collections::VecDeque, f32::consts::PI, time::Duration};

use avian2d::prelude::*;
use bevy::{
    ecs::{system::SystemState, world::CommandQueue},
    prelude::*,
    scene::ScenePlugin,
    state::app::StatesPlugin,
    time::TimeUpdateStrategy,
};
use pb_engine::{
    PbEnginePlugin,
    pawn::{
        MAX_ANGULAR_VELOCITY, MAX_VELOCITY, PawnBundle,
        ai::path::{MovementQuery, PathObservation},
    },
    save::SaveModel,
};
use pyo3::prelude::*;
use rand::{Rng, SeedableRng, rngs::SmallRng};

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
    path_q: SystemState<MovementQuery<'static, 'static>>,
    step_count: usize,
    path: VecDeque<Vec2>,
}

#[derive(Copy, Clone, Debug)]
pub struct Action {
    pub angle: f32,
    pub force: f32,
    pub torque: f32,
}

#[derive(Copy, Clone, Debug)]
pub struct Observation {
    pub linear_velocity_t: f32,
    pub linear_velocity_r: f32,
    pub angular_velocity: f32,
    pub target_t: f32,
    pub target_r: f32,
    pub collision_t: f32,
    pub collision_r: f32,
    pub collision_normal_t: f32,
    pub collision_is_wall: f32,
    pub collision_is_pawn: f32,
}

const TIMESTEP: Duration = Duration::from_micros(15625);

#[pymethods]
impl Environment {
    #[new]
    pub fn new() -> Self {
        let mut app = App::new();

        app.add_plugins((
            MinimalPlugins,
            TransformPlugin,
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

        let saved_walls = serde_json::from_str(include_str!("empty.json")).unwrap();
        spawn_save(app.world_mut(), saved_walls);

        let path_q = SystemState::new(app.world_mut());

        app.update();

        Environment {
            app,
            rng: SmallRng::from_os_rng(),
            entity: Entity::PLACEHOLDER,
            path_q,
            step_count: 0,
            path: VecDeque::new(),
        }
    }

    pub fn reset(&mut self, seed: Option<u64>) -> [f32; PathObservation::SIZE] {
        if let Some(seed) = seed {
            self.rng = SmallRng::seed_from_u64(seed);
        }

        if self.entity != Entity::PLACEHOLDER {
            self.app.world_mut().entity_mut(self.entity).despawn();
        }

        let (position, path) = loop {
            let mut position: Vec2 = self.rng.random::<[f32; 2]>().into();
            position = (position - Vec2::splat(0.5)) * 8.;

            let mut target: Vec2 = self.rng.random::<[f32; 2]>().into();
            if self.rng.random_bool(0.3) {
                target = position + (target - Vec2::splat(0.5)) * 1.;
            } else {
                target = (target - Vec2::splat(0.5)) * 8.;
            }

            if let Some(steps) = self.path(position, target) {
                break (position, steps);
            }
        };

        let rotation = self.rng.random_range(-PI..PI);

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
            ))
            .id();
        self.step_count = 0;
        self.path = VecDeque::from_iter(path);
        self.observe().into()
    }

    pub fn step(
        &mut self,
        action: [f32; Action::SIZE],
    ) -> ([f32; PathObservation::SIZE], f32, bool, bool) {
        self.step_count += 1;

        let action = Action::from(action);
        let prev_observation = self.observe();
        let prev_steps_remaining = self.path.len();

        self.act(action);

        let observation = self.observe();
        let steps_completed = prev_steps_remaining - self.path.len();

        let terminated = self.path.is_empty();
        let truncated = self.step_count > 2000;

        let dist_reward = if steps_completed > 0 {
            self.rng.random_range(3.0..6.0) * steps_completed as f32
        } else {
            (prev_observation.target_r - observation.target_r)
                / (MAX_VELOCITY * TIMESTEP.as_secs_f32())
        };

        let reward = dist_reward
            + observation.velocity_reward()
            + observation.rotation_penalty()
            + observation.collision_penalty();

        (observation.into(), reward, terminated, truncated)
    }
}

impl Environment {
    fn act(&mut self, action: Action) {
        self.path_q
            .get_mut(self.app.world_mut())
            .act(self.entity, action.angle, action.force, action.torque)
            .unwrap();

        self.app.update();
    }

    fn observe(&mut self) -> PathObservation {
        self.path_q
            .get_mut(self.app.world_mut())
            .observe(self.entity, &mut self.path)
            .unwrap()
    }

    fn path(&mut self, _from: Vec2, to: Vec2) -> Option<Vec<Vec2>> {
        Some(vec![to])
    }
}

impl Action {
    pub const SIZE: usize = 3;
}

impl From<[f32; Self::SIZE]> for Action {
    fn from([angle, force, torque]: [f32; Self::SIZE]) -> Self {
        Action {
            angle,
            force,
            torque,
        }
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

fn spawn_save(world: &mut World, save: SaveModel) {
    let mut commands = CommandQueue::default();
    save.spawn(&mut Commands::new(&mut commands, world));
    commands.apply(world);
}

#[test]
fn test() {
    let mut env = Environment::new();

    env.reset(None);
    env.step([0.0, 1.0, 0.0]);
}
