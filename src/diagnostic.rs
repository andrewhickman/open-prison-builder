use bevy::{
    app::{App, Plugin},
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    ecs::schedule::{LogLevel, ScheduleBuildSettings},
    remote::{http::RemoteHttpPlugin, RemotePlugin},
    utils::default,
};

pub struct DiagnosticsPlugin;

impl Plugin for DiagnosticsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            FrameTimeDiagnosticsPlugin,
            LogDiagnosticsPlugin::default(),
            RemotePlugin::default(),
            RemoteHttpPlugin::default(),
        ));

        app.configure_schedules(ScheduleBuildSettings {
            ambiguity_detection: LogLevel::Warn,
            hierarchy_detection: LogLevel::Warn,
            use_shortnames: false,
            ..default()
        });
    }
}
