#![allow(clippy::type_complexity, clippy::too_many_arguments)]

use std::time::Duration;

use avian2d::prelude::*;
use bevy::{
    ecs::world::CommandQueue, prelude::*, scene::ScenePlugin, state::app::StatesPlugin,
    time::TimeUpdateStrategy,
};
use pb_engine::{PbEnginePlugin, pawn::Pawn, save::SaveModel};
use pyo3::prelude::*;
use rand::{SeedableRng, rngs::SmallRng};

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
    step_count: usize,
}

#[derive(Copy, Clone, Debug)]
pub struct Action {}

#[derive(Copy, Clone, Debug)]
pub struct Observation {}

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

        app.update();

        Environment {
            app,
            rng: SmallRng::from_os_rng(),
            entity: Entity::PLACEHOLDER,
            step_count: 0,
        }
    }

    pub fn reset(&mut self, seed: Option<u64>) -> [f32; Observation::SIZE] {
        if let Some(seed) = seed {
            self.rng = SmallRng::seed_from_u64(seed);
        }

        if self.entity != Entity::PLACEHOLDER {
            self.app.world_mut().entity_mut(self.entity).despawn();
        }

        let position = Vec2::ZERO;
        let rotation = 0.;

        self.entity = self
            .app
            .world_mut()
            .spawn((
                Pawn::bundle(position, rotation),
                LinearVelocity(Vec2::ZERO),
                AngularVelocity(0.),
            ))
            .id();
        self.step_count = 0;
        self.observe().into()
    }

    pub fn step(
        &mut self,
        action: [f32; Action::SIZE],
    ) -> ([f32; Observation::SIZE], f32, bool, bool) {
        self.step_count += 1;

        let _prev_observation = self.observe();

        self.act(action.into());

        let observation = self.observe();

        let terminated = false;
        let truncated = self.step_count > 2000;

        let reward = 0.;

        (observation.into(), reward, terminated, truncated)
    }
}

impl Environment {
    fn act(&mut self, _action: Action) {
        self.app.update();
    }

    fn observe(&mut self) -> Observation {
        todo!()
    }
}

impl Action {
    pub const SIZE: usize = 0;
}

impl From<[f32; Self::SIZE]> for Action {
    fn from([]: [f32; Self::SIZE]) -> Self {
        Action {}
    }
}

impl Observation {
    pub const SIZE: usize = 0;
}

impl From<Observation> for [f32; Observation::SIZE] {
    fn from(Observation {}: Observation) -> Self {
        []
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
    env.step([]);
}
