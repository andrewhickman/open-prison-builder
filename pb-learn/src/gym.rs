use std::time::Duration;

use bevy::{prelude::*, time::TimeUpdateStrategy};
use pb_engine::PbEnginePlugin;

pub struct Gym {
    app: App,
}

impl Gym {
    pub fn new() -> Self {
        let mut app = App::new();

        app.add_plugins((MinimalPlugins, PbEnginePlugin));

        let timestep = Duration::from_micros(15625);
        app.insert_resource(TimeUpdateStrategy::ManualDuration(timestep));
        app.insert_resource(Time::<Fixed>::from_duration(timestep));

        app.finish();
        app.cleanup();

        Gym { app: App::new() }
    }

    pub fn step(&mut self) {
        self.app.update();
    }
}
