#![allow(clippy::too_many_arguments)]

mod diff;

use std::{fs, path::PathBuf};

use bevy::{
    asset::{LoadState, RenderAssetUsages},
    core_pipeline::CorePipelinePlugin,
    ecs::system::{ScheduleSystem, SystemState},
    log::DEFAULT_FILTER,
    prelude::*,
    render::{
        RenderPlugin,
        camera::RenderTarget,
        render_resource::{Extent3d, TextureDimension, TextureFormat, TextureUsages},
        view::{
            NoFrustumCulling,
            screenshot::{Screenshot, ScreenshotCaptured},
        },
    },
    scene::ScenePlugin,
    sprite::SpritePlugin,
    state::app::StatesPlugin,
    text::TextPlugin,
    window::ExitCondition,
};

use pb_assets::{AssetHandles, PbAssetsPlugin};
use pb_engine::{
    EngineState, PbEnginePlugin,
    save::{LoadSeed, load},
};
use pb_render::{
    PbRenderPlugin,
    grid::{GRID_MESH_HANDLE, GridMaterial},
    projection::projection,
};
use serde::de::DeserializeSeed;

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
fn render_empty() {
    run_test("empty");
}

#[test]
fn render_wall() {
    run_test("wall");
}

#[test]
fn render_grid() {
    run_test_with_setup("grid", show_grid);

    fn show_grid(mut commands: Commands, mut grids: ResMut<Assets<GridMaterial>>) {
        let grid = grids.add(GridMaterial::new(
            Srgba::hex("5f4754").unwrap().with_alpha(0.38).into(),
        ));

        commands.spawn((
            Visibility::default(),
            Mesh2d(GRID_MESH_HANDLE),
            MeshMaterial2d(grid),
            NoFrustumCulling,
        ));
    }
}

fn run_test(name: &str) {
    run_test_with_setup(name, || {});
}

fn run_test_with_setup<M, S>(name: &str, setup: S)
where
    S: IntoScheduleConfigs<ScheduleSystem, M>,
{
    let _ = tracing_subscriber::fmt()
        .with_env_filter(DEFAULT_FILTER)
        .try_init();

    let mut app = App::new();
    app.add_plugins((
        MinimalPlugins,
        TransformPlugin,
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
        SpritePlugin,
        StatesPlugin,
    ))
    .add_plugins((PbAssetsPlugin, PbEnginePlugin, PbRenderPlugin));

    app.add_systems(Startup, setup).add_systems(Update, update);

    app.init_resource::<TestState>()
        .insert_resource(TestConfig {
            dir: format!("tests/data/{name}").into(),
        });

    let exit_code = app.run();

    assert!(exit_code.is_success());
}

fn update(
    mut commands: Commands,
    registry: Res<AppTypeRegistry>,
    config: Res<TestConfig>,
    mut state: ResMut<TestState>,
    timer: Res<Time>,
    assets: Res<AssetHandles>,
    asset_server: Res<AssetServer>,
    mut images: ResMut<Assets<Image>>,
    engine_state: Res<State<EngineState>>,
    mut exit_e: EventWriter<AppExit>,
    mut ticks: Local<u32>,
) {
    if timer.elapsed_secs() > 5. {
        error!("Test execution timed out");
        exit_e.write(AppExit::error());
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
                    target: RenderTarget::Image(image.clone().into()),
                    clear_color: ClearColorConfig::Custom(Srgba::hex("192a28").unwrap().into()),
                    ..Default::default()
                },
                projection(),
                Msaa::Off,
            ));

            let save_json = fs::read_to_string(config.dir.join("scene.json")).unwrap();
            let save = from_json(LoadSeed::new(registry.0.clone()), &save_json).unwrap();
            commands.queue(move |world: &mut World| {
                let mut param: SystemState<pb_engine::save::LoadParam<'_, '_>> =
                    SystemState::new(world);
                let root = load(world, &mut param, &save).unwrap();
                world
                    .resource_mut::<NextState<EngineState>>()
                    .set(EngineState::Running(root));
            });

            *state = TestState::Prepare { image };
        }
        TestState::Prepare { image } => {
            match assets.load_state(&asset_server) {
                LoadState::NotLoaded | LoadState::Loading => return,
                LoadState::Loaded => (),
                LoadState::Failed(error) => {
                    error!("Failed to load all assets, exiting: {error}");
                    exit_e.write(AppExit::error());
                }
            }

            if matches!(engine_state.get(), EngineState::Disabled) {
                return;
            }

            // Wait for all render resources to be created...
            *ticks += 1;
            if *ticks < 100 {
                return;
            }

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
                exit_e.write(AppExit::Success);
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

                exit_e.write(AppExit::error());
            } else {
                exit_e.write(AppExit::Success);
            }
        }
    }
}

fn from_json<S, T>(seed: S, json: &str) -> Result<T, serde_json::Error>
where
    S: for<'de> DeserializeSeed<'de, Value = T>,
    T: 'static,
{
    let mut de = serde_json::Deserializer::from_str(json);
    let value = seed.deserialize(&mut de)?;
    de.end()?;
    Ok(value)
}
