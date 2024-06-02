use bevy::{app::AppExit, asset, asset::UntypedAssetId, prelude::*};

#[derive(Resource)]
pub struct Assets {
    pub font_graduate: Handle<Font>,
    pub font_tomorrow: Handle<Font>,
    pub font_tomorrow_bold: Handle<Font>,
    pub font_tomorrow_italic: Handle<Font>,
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
        font_tomorrow: server.load("fonts/Tomorrow-Bold.ttf"),
        font_tomorrow_bold: server.load("fonts/Tomorrow-Medium.ttf"),
        font_tomorrow_italic: server.load("fonts/Tomorrow-MediumItalic.ttf"),
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
        state.set(LoadState::Ready);
    }

    let failed = assets
        .asset_ids()
        .filter_map(|id| server.get_load_state(id))
        .any(|state| state == asset::LoadState::Failed);
    if failed {
        error_once!("Failed to load assets, exiting application");
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
            font_tomorrow_bold,
            font_tomorrow_italic,
        } = self;

        [
            font_graduate.into(),
            font_tomorrow.into(),
            font_tomorrow_bold.into(),
            font_tomorrow_italic.into(),
        ]
        .into_iter()
    }
}
