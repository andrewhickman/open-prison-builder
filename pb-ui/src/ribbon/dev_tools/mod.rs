use bevy::prelude::*;

use pb_engine::dev::DevSettings;

pub mod path_stress_test;

pub fn toggle_draw_paths(_: Trigger<Pointer<Click>>, mut settings: ResMut<DevSettings>) -> Result {
    settings.draw_paths = !settings.draw_paths;
    Ok(())
}

pub fn toggle_draw_meshes(_: Trigger<Pointer<Click>>, mut settings: ResMut<DevSettings>) -> Result {
    settings.draw_meshes = !settings.draw_meshes;
    Ok(())
}
