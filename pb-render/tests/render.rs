use std::time::Duration;

use bevy::{
    asset::RenderAssetUsages,
    core_pipeline::CorePipelinePlugin,
    log::LogPlugin,
    prelude::*,
    render::{
        camera::RenderTarget,
        render_resource::{Extent3d, TextureDimension, TextureFormat, TextureUsages},
        view::screenshot::{Screenshot, ScreenshotCaptured},
        RenderPlugin,
    },
    scene::ScenePlugin,
    sprite::SpritePlugin,
    state::app::StatesPlugin,
    text::TextPlugin,
    time::common_conditions::on_timer,
    window::ExitCondition,
};

use pb_assets::PbAssetsPlugin;
use pb_engine::PbEnginePlugin;
use pb_render::{projection::projection, PbRenderPlugin};

struct HeadlessCameraPlugin;

#[derive(Resource)]
struct HeadlessRenderTarget(Handle<Image>);

#[test]
fn assets() {
    let mut app = App::new();
    app.add_plugins((
        MinimalPlugins,
        TransformPlugin,
        HierarchyPlugin,
        WindowPlugin {
            primary_window: None,
            exit_condition: ExitCondition::DontExit,
            ..default()
        },
        AssetPlugin {
            file_path: concat!(env!("CARGO_MANIFEST_DIR"), "/../assets").to_owned(),
            ..default()
        },
        ScenePlugin,
        TextPlugin,
        RenderPlugin::default(),
        ImagePlugin::default(),
        CorePipelinePlugin,
        SpritePlugin { add_picking: false },
        StatesPlugin,
        LogPlugin::default(),
    ))
    .add_plugins((
        PbAssetsPlugin,
        PbEnginePlugin,
        PbRenderPlugin,
        HeadlessCameraPlugin,
    ));

    app.add_systems(Update, screenshot.run_if(on_timer(Duration::from_secs(1))));
    let exit_code = app.run();

    assert!(exit_code.is_success());
}

impl Plugin for HeadlessCameraPlugin {
    fn build(&self, app: &mut App) {
        let mut image = Image::new_fill(
            Extent3d {
                width: 1028,
                height: 768,
                ..default()
            },
            TextureDimension::D2,
            &[0; 4],
            TextureFormat::bevy_default(),
            RenderAssetUsages::RENDER_WORLD,
        );
        image.texture_descriptor.usage = TextureUsages::COPY_SRC
            | TextureUsages::RENDER_ATTACHMENT
            | TextureUsages::TEXTURE_BINDING;
        let image = app.world_mut().resource_mut::<Assets<Image>>().add(image);

        app.world_mut().spawn((
            Camera2d,
            Camera {
                target: RenderTarget::Image(image.clone()),
                clear_color: ClearColorConfig::Custom(Srgba::hex("19ff28").unwrap().into()),
                ..Default::default()
            },
            projection(),
            Msaa::Off,
        ));

        app.insert_resource(HeadlessRenderTarget(image));
    }
}

fn screenshot(mut commands: Commands, target: Res<HeadlessRenderTarget>) {
    info!("start capturing screenshot");
    commands
        .spawn(Screenshot::image(target.0.clone()))
        .observe(screenshot_captured);
}

fn screenshot_captured(_: Trigger<ScreenshotCaptured>, mut exit: EventWriter<AppExit>) {
    info!("screenshot captured!");
    exit.send(AppExit::Success);
}
