use bevy::{
    asset::{LoadState, UntypedAssetId},
    prelude::*,
};

#[derive(Resource)]
pub struct Assets {
    pub font_graduate: Handle<Font>,
    pub font_tomorrow: Handle<Font>,
    pub tomorrow_italic_font: Handle<Font>,
    pub button_image: Handle<Image>,
    pub ribbon_button_image: Handle<Image>,
    pub ribbon_button_wall_image: Handle<Image>,
    pub bevy_icon: Handle<Image>,
    pub github_icon: Handle<Image>,
    pub pawn_image: Handle<Image>,
    pub close_icon: Handle<Image>,
    pub error_icon: Handle<Image>,
}

pub struct AssetsPlugin;

impl Plugin for AssetsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, load);
    }
}

pub fn load(mut commands: Commands, server: Res<AssetServer>) {
    commands.insert_resource(Assets {
        font_graduate: server.load("fonts/Graduate-Regular.ttf"),
        font_tomorrow: server.load("fonts/Tomorrow-Medium.ttf"),
        tomorrow_italic_font: server.load("fonts/Tomorrow-MediumItalic.ttf"),
        button_image: server.load("image/button.png"),
        ribbon_button_image: server.load("image/ribbon_button.png"),
        ribbon_button_wall_image: server.load("image/ribbon_button_wall.png"),
        bevy_icon: server.load("image/bevy.png"),
        github_icon: server.load("image/github.png"),
        pawn_image: server.load("image/pawn.png"),
        close_icon: server.load("image/close.png"),
        error_icon: server.load("image/error.png"),
    });
}

impl Assets {
    pub fn load_state(&self, server: &AssetServer) -> LoadState {
        self.asset_ids()
            .map(|id| server.get_load_state(id).unwrap_or(LoadState::NotLoaded))
            .fold(LoadState::Loaded, |l, r| match (l, r) {
                (LoadState::Failed(error), _) | (_, LoadState::Failed(error)) => {
                    LoadState::Failed(error)
                }
                (LoadState::NotLoaded | LoadState::Loading, _)
                | (_, LoadState::NotLoaded | LoadState::Loading) => LoadState::Loading,
                (LoadState::Loaded, LoadState::Loaded) => LoadState::Loaded,
            })
    }

    fn asset_ids(&self) -> impl Iterator<Item = UntypedAssetId> {
        let Assets {
            font_graduate,
            font_tomorrow,
            tomorrow_italic_font,
            button_image,
            ribbon_button_image,
            ribbon_button_wall_image,
            bevy_icon,
            github_icon,
            pawn_image,
            close_icon,
            error_icon,
        } = self;

        [
            font_graduate.into(),
            font_tomorrow.into(),
            tomorrow_italic_font.into(),
            button_image.into(),
            ribbon_button_image.into(),
            ribbon_button_wall_image.into(),
            bevy_icon.into(),
            github_icon.into(),
            pawn_image.into(),
            close_icon.into(),
            error_icon.into(),
        ]
        .into_iter()
    }
}
