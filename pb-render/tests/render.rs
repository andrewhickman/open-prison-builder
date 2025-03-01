#![allow(clippy::too_many_arguments)]

mod diff;

use std::{fs, path::PathBuf};

use bevy::{
    asset::{LoadState, RenderAssetUsages},
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
    window::ExitCondition,
};

use pb_assets::{AssetHandles, PbAssetsPlugin};
use pb_engine::PbEnginePlugin;
use pb_render::{projection::projection, PbRenderPlugin};

#[derive(Resource)]
struct TestConfig {
    dir: PathBuf,
}

#[derive(Default, Resource)]
enum TestState {
    #[default]
    Startup,
    Prepare {
        image: Handle<Image>,
    },
    Screenshot,
    ScreenshotCaptured {
        screenshot: Image,
    },
}

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
    .add_plugins((PbAssetsPlugin, PbEnginePlugin, PbRenderPlugin));

    app.add_systems(Update, update);

    app.init_resource::<TestState>()
        .insert_resource(TestConfig {
            dir: "tests/data/empty".into(),
        });

    let exit_code = app.run();

    assert!(exit_code.is_success());
}

fn update(
    mut commands: Commands,
    config: Res<TestConfig>,
    mut state: ResMut<TestState>,
    timer: Res<Time>,
    assets: Res<AssetHandles>,
    asset_server: Res<AssetServer>,
    mut images: ResMut<Assets<Image>>,
    // gpu_images: Res<RenderAssets<GpuImage>>,
    mut exit_e: EventWriter<AppExit>,
) {
    if timer.elapsed_secs() > 5. {
        error!("Test execution timed out");
        exit_e.send(AppExit::error());
    }

    match &*state {
        TestState::Startup => {
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

            let image = images.add(image);
            commands.spawn((
                Camera2d,
                Camera {
                    target: RenderTarget::Image(image.clone()),
                    clear_color: ClearColorConfig::Custom(Srgba::hex("192a28").unwrap().into()),
                    ..Default::default()
                },
                projection(),
                Msaa::Off,
            ));

            *state = TestState::Prepare { image };
        }
        TestState::Prepare { image } => {
            match assets.load_state(&asset_server) {
                LoadState::NotLoaded | LoadState::Loading => return,
                LoadState::Loaded => (),
                LoadState::Failed(error) => {
                    error!("Failed to load all assets, exiting: {error}");
                    exit_e.send(AppExit::error());
                }
            }

            // if gpu_images.get(image.id()).is_none() {
            //     return;
            // }

            commands.spawn(Screenshot::image(image.clone())).observe(
                |trigger: Trigger<ScreenshotCaptured>, mut state: ResMut<TestState>| {
                    *state = TestState::ScreenshotCaptured {
                        screenshot: trigger.0.clone(),
                    }
                },
            );
            *state = TestState::Screenshot;
        }
        TestState::Screenshot => (),
        TestState::ScreenshotCaptured { screenshot } => {
            let actual = screenshot.clone().try_into_dynamic().unwrap();

            fs::create_dir_all(&config.dir).unwrap();
            let expected_path = config.dir.join("expected.png");
            if !expected_path.exists() {
                actual.save(expected_path).unwrap();
                exit_e.send(AppExit::Success);
                return;
            }

            let expected = image::open(expected_path).unwrap();

            let (error, diff) = diff::diff_image(&expected, &actual);
            if error > 0 {
                let diff_path = config.dir.join("diff.png");
                let actual_path = config.dir.join("actual.png");

                error!(
                    "Difference of ({error}) in image. See '{}' for the changed pixels.",
                    diff_path.display()
                );

                diff.save(diff_path).unwrap();
                actual.save(actual_path).unwrap();

                exit_e.send(AppExit::error());
            } else {
                exit_e.send(AppExit::Success);
            }
        }
    }
}
