use bevy::{app::AppExit, asset, asset::UntypedAssetId, prelude::*};

#[derive(Resource)]
pub struct Assets {
    pub font_graduate: Handle<Font>,
    pub font_tomorrow: Handle<Font>,
    pub tomorrow_italic_font: Handle<Font>,
    pub button_image: Handle<Image>,
    pub bevy_icon_image: Handle<Image>,
    pub github_icon_image: Handle<Image>,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default, States)]
pub enum LoadState {
    #[default]
    Pending,
    Ready,
}

pub struct AssetsPlugin;

impl Plugin for AssetsPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<LoadState>();

        app.add_systems(Startup, load);

        app.add_systems(
            PreUpdate,
            update_load_state.run_if(update_load_state_condition),
        );
    }
}

pub fn load(mut commands: Commands, server: Res<AssetServer>) {
    commands.insert_resource(Assets {
        font_graduate: server.load("fonts/Graduate-Regular.ttf"),
        font_tomorrow: server.load("fonts/Tomorrow-Medium.ttf"),
        tomorrow_italic_font: server.load("fonts/Tomorrow-MediumItalic.ttf"),
        button_image: server.load("image/button.png"),
        bevy_icon_image: server.load("image/bevy.png"),
        github_icon_image: server.load("image/github.png"),
    });
}

pub fn update_load_state(
    mut state: ResMut<NextState<LoadState>>,
    assets: Res<Assets>,
    server: Res<AssetServer>,
    mut exit_e: EventWriter<AppExit>,
) {
    let succeeded = assets
        .asset_ids()
        .filter_map(|id| server.get_load_state(id))
        .all(|state| state == asset::LoadState::Loaded);
    if succeeded {
        info!("Loaded all assets successfully");
        state.set(LoadState::Ready);
    }

    let failed = assets
        .asset_ids()
        .filter_map(|id| server.get_load_state(id))
        .any(|state| state == asset::LoadState::Failed);
    if failed {
        error!("Failed to load assets, exiting application");
        exit_e.send(AppExit);
    }
}

pub fn update_load_state_condition(
    load_state: Res<State<LoadState>>,
    mut font_e: EventReader<AssetEvent<Font>>,
    mut image_e: EventReader<AssetEvent<Image>>,
) -> bool {
    *load_state == LoadState::Pending && (font_e.read().count() > 0 || image_e.read().count() > 0)
}

impl Assets {
    fn asset_ids(&self) -> impl Iterator<Item = UntypedAssetId> {
        let Assets {
            font_graduate,
            font_tomorrow,
            tomorrow_italic_font,
            button_image,
            bevy_icon_image,
            github_icon_image,
        } = self;

        [
            font_graduate.into(),
            font_tomorrow.into(),
            tomorrow_italic_font.into(),
            button_image.into(),
            bevy_icon_image.into(),
            github_icon_image.into(),
        ]
        .into_iter()
    }
}
