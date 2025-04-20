#![allow(clippy::type_complexity, clippy::too_many_arguments)]

use std::{f32::consts::PI, time::Duration};

use avian2d::prelude::*;
use bevy::{
    ecs::system::SystemState, prelude::*, scene::ScenePlugin, state::app::StatesPlugin,
    time::TimeUpdateStrategy,
};
use pb_engine::{
    pawn::{ai::path::PathObservation, Pawn, PawnBundle, MAX_ANGULAR_VELOCITY, MAX_VELOCITY},
    save::{load, LoadSeed, Save},
    wall::Wall,
    PbEnginePlugin,
};
use pb_util::math::normalize_angle;
use pyo3::prelude::*;
use rand::{rngs::SmallRng, Rng, SeedableRng};
use serde::de::DeserializeSeed;
use vleue_navigator::{
    prelude::{ManagedNavMesh, NavMeshStatus},
    NavMesh,
};

#[pymodule(name = "pb_learn_env")]
pub fn pb_learn(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Environment>()?;
    Ok(())
}

#[pyclass]
#[derive(Debug)]
pub struct Environment {
    app: App,
    rng: SmallRng,
    entity: Entity,
    pawn_q: QueryState<
        (
            &'static Position,
            &'static Rotation,
            &'static LinearVelocity,
            &'static AngularVelocity,
            &'static TargetPosition,
            &'static ShapeHits,
        ),
        With<Pawn>,
    >,
    has_pawn_q: QueryState<(), With<Wall>>,
    has_wall_q: QueryState<(), With<Wall>>,
    navmesh_q: QueryState<(&'static ManagedNavMesh, &'static NavMeshStatus)>,
    step_count: usize,
    path: Vec<Vec2>,
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

        let type_registry = app.world().get_resource::<AppTypeRegistry>().unwrap();
        let seed = LoadSeed::new(type_registry.0.clone());
        let saved_walls = save_from_json(seed, include_str!("walls.json")).unwrap();

        let mut load_state = SystemState::new(app.world_mut());
        load(app.world_mut(), &mut load_state, &saved_walls).unwrap();

        let pawn_q = app.world_mut().query_filtered();
        let has_pawn_q = app.world_mut().query_filtered();
        let has_wall_q = app.world_mut().query_filtered();
        let navmesh_q = app.world_mut().query();

        app.update();

        Environment {
            app,
            rng: SmallRng::from_os_rng(),
            entity: Entity::PLACEHOLDER,
            pawn_q,
            has_pawn_q,
            has_wall_q,
            navmesh_q,
            step_count: 0,
            path: Vec::new(),
        }
    }

    pub fn reset(&mut self, seed: Option<u64>) -> [f32; PathObservation::SIZE] {
        if let Some(seed) = seed {
            self.rng = SmallRng::seed_from_u64(seed);
        }

        if self.entity != Entity::PLACEHOLDER {
            self.app.world_mut().entity_mut(self.entity).despawn();
        }

        let (position, mut path) = loop {
            let mut position: Vec2 = self.rng.random::<[f32; 2]>().into();
            position = (position - Vec2::splat(0.5)) * 10.;

            let mut target: Vec2 = self.rng.random::<[f32; 2]>().into();
            if self.rng.random_bool(0.3) {
                target = position + (target - Vec2::splat(0.5)) * 1.;
            } else {
                target = (target - Vec2::splat(0.5)) * 10.;
            }

            let mut offset: Vec2 = self.rng.random::<[f32; 2]>().into();
            offset = (offset - Vec2::splat(0.5)) * 15.;

            position += offset;
            target += offset;

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

        let target = path.remove(0);

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
        self.step_count = 0;
        self.path = path;
        self.observe().into()
    }

    pub fn step(
        &mut self,
        action: [f32; Action::SIZE],
    ) -> ([f32; PathObservation::SIZE], f32, bool, bool) {
        self.step_count += 1;

        let action = Action::from(action);
        let prev_observation = self.observe();

        self.act(action);

        let mut observation = self.observe();

        let step_terminated = observation.target_r < (MAX_VELOCITY * TIMESTEP.as_secs_f32());
        let terminated = step_terminated && self.path.is_empty();

        let truncated = self.step_count > 2000;

        let dist_reward = if step_terminated {
            self.rng.random_range(2.0..5.0)
        } else {
            (prev_observation.target_r - observation.target_r)
                / (MAX_VELOCITY * TIMESTEP.as_secs_f32())
        };
        let velocity_reward = normalize_angle(observation.linear_velocity_t - observation.target_t)
            .cos()
            * (observation.linear_velocity_r / MAX_VELOCITY);
        let rotation_penalty = observation.target_t.abs() / PI;
        let angular_velocity_penalty = observation.angular_velocity / MAX_ANGULAR_VELOCITY;

        let collision_penality = (-observation.collision_r * 16.).exp2();

        let reward = dist_reward * 1.2 + velocity_reward * 0.7
            - rotation_penalty * 0.5
            - angular_velocity_penalty * 0.5
            - collision_penality * 2.;

        if step_terminated && !terminated {
            self.step_count = 0;
            self.pop_path();
            observation = self.observe();
        }

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
            .update_movement(action.angle, action.force, action.torque);

        self.app.update();
    }

    fn observe(&mut self) -> PathObservation {
        let (position, rotation, linear_velocity, angular_velocity, target, collisions) =
            self.pawn_q.get(self.app.world(), self.entity).unwrap();

        PathObservation::new(
            position,
            rotation,
            linear_velocity,
            angular_velocity,
            collisions,
            target.0,
            |id| self.has_pawn_q.get(self.app.world(), id).is_ok(),
            |id| self.has_wall_q.get(self.app.world(), id).is_ok(),
        )
    }

    fn path(&mut self, from: Vec2, to: Vec2) -> Option<Vec<Vec2>> {
        loop {
            let (navmesh_id, status) = self.navmesh_q.single(self.app.world());
            match status {
                NavMeshStatus::Building => (),
                status @ (NavMeshStatus::Failed
                | NavMeshStatus::Invalid
                | NavMeshStatus::Cancelled) => panic!("unexpected navmesh status {status:?}"),
                NavMeshStatus::Built => {
                    let navmesh_assets = self.app.world().resource::<Assets<NavMesh>>();
                    let navmesh = navmesh_assets.get(navmesh_id).unwrap();

                    return navmesh.path(from, to).map(|p| p.path);
                }
            }

            self.app.update();
        }
    }

    fn pop_path(&mut self) {
        let next_target = self.path.remove(0);
        self.app
            .world_mut()
            .entity_mut(self.entity)
            .get_mut::<TargetPosition>()
            .unwrap()
            .0 = next_target;
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

fn save_from_json(seed: LoadSeed, json: &str) -> Result<Save, serde_json::Error> {
    let mut de = serde_json::Deserializer::from_str(json);
    let value = seed.deserialize(&mut de)?;
    de.end()?;
    Ok(value)
}

// HACK
unsafe impl Send for Environment {}

unsafe impl Sync for Environment {}

#[test]
fn test() {
    let mut env = Environment::new();

    let obs = env.reset(None);
    println!("{:?}", obs);
    let obs = env.step([0.0, 1.0, 0.0]);
    println!("{:?}", obs);

    assert!(env.path.is_empty());
}
