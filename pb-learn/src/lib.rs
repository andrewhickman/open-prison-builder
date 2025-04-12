use std::time::Duration;

use bevy::{prelude::*, scene::ScenePlugin, state::app::StatesPlugin, time::TimeUpdateStrategy};
use pb_engine::PbEnginePlugin;
use pyo3::prelude::*;
use rand::{rngs::SmallRng, Rng, SeedableRng};

#[pyclass]
pub struct Environment {
    #[allow(unused)]
    app: App,
    rng: SmallRng,
    cur_pos: u32,
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

        Environment {
            app,
            rng: SmallRng::from_os_rng(),
            cur_pos: 0,
        }
    }

    pub fn reset(&mut self, seed: Option<u64>) -> u32 {
        self.cur_pos = 0;

        if let Some(seed) = seed {
            self.rng = SmallRng::seed_from_u64(seed);
        }

        self.cur_pos
    }

    pub fn step(&mut self, action: u32) -> (u32, f32, bool, bool) {
        match action {
            0 => {
                if self.cur_pos > 0 {
                    self.cur_pos -= 1
                }
            }
            1 => self.cur_pos += 1,
            _ => panic!("unknown action {action}"),
        }

        let terminated = self.cur_pos >= 10;
        let reward = if terminated {
            self.rng.random_range(0.5..1.5)
        } else {
            -0.01
        };

        (self.cur_pos, reward, terminated, false)
    }
}

impl Default for Environment {
    fn default() -> Self {
        Self::new()
    }
}

/// A Python module implemented in Rust.
#[pymodule]
fn pb_learn(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Environment>()?;
    Ok(())
}

// HACK
unsafe impl Send for Environment {}

unsafe impl Sync for Environment {}
