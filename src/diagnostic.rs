use bevy::{
    diagnostic::{
        EntityCountDiagnosticsPlugin, FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin,
        SystemInformationDiagnosticsPlugin,
    },
    ecs::schedule::{LogLevel, ScheduleBuildSettings},
    prelude::*,
    remote::{http::RemoteHttpPlugin, RemotePlugin},
    render::diagnostic::RenderDiagnosticsPlugin,
};
use iyes_perf_ui::{prelude::PerfUiDefaultEntries, PerfUiPlugin};

pub struct DiagnosticsPlugin;

impl Plugin for DiagnosticsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            FrameTimeDiagnosticsPlugin,
            EntityCountDiagnosticsPlugin,
            SystemInformationDiagnosticsPlugin,
            RenderDiagnosticsPlugin,
            LogDiagnosticsPlugin::default(),
            RemotePlugin::default(),
            RemoteHttpPlugin::default(),
            PerfUiPlugin,
        ));

        app.configure_schedules(ScheduleBuildSettings {
            ambiguity_detection: LogLevel::Warn,
            hierarchy_detection: LogLevel::Warn,
            use_shortnames: false,
            ..default()
        });
        app.add_systems(Startup, spawn_perf_ui);
    }
}

fn spawn_perf_ui(mut commands: Commands) {
    commands.spawn(PerfUiDefaultEntries::default());
}
