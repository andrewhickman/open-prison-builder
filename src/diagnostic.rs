use bevy::{
    diagnostic::{
        EntityCountDiagnosticsPlugin, FrameTimeDiagnosticsPlugin,
        SystemInformationDiagnosticsPlugin,
    },
    ecs::schedule::{LogLevel, ScheduleBuildSettings},
    prelude::*,
    remote::{RemotePlugin, http::RemoteHttpPlugin},
    render::diagnostic::RenderDiagnosticsPlugin,
};

pub struct DiagnosticsPlugin;

impl Plugin for DiagnosticsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            FrameTimeDiagnosticsPlugin::default(),
            EntityCountDiagnosticsPlugin,
            SystemInformationDiagnosticsPlugin,
            RenderDiagnosticsPlugin,
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
