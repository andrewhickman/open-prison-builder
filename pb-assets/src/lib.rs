use bevy::{
    asset::{LoadState, UntypedAssetId},
    image::{
        ImageAddressMode, ImageFilterMode, ImageLoaderSettings, ImageSampler,
        ImageSamplerDescriptor,
    },
    prelude::*,
};

#[derive(Clone, Resource)]
pub struct AssetHandles {
    pub font_graduate: Handle<Font>,
    pub font_tomorrow: Handle<Font>,
    pub tomorrow_italic_font: Handle<Font>,
    pub button_image: Handle<Image>,
    pub ribbon_button_image: Handle<Image>,
    pub ribbon_button_wall_image: Handle<Image>,
    pub ribbon_button_delete_wall_image: Handle<Image>,
    pub ribbon_button_door_image: Handle<Image>,
    pub bevy_icon: Handle<Image>,
    pub github_icon: Handle<Image>,
    pub pawn_image: Handle<Image>,
    pub pawn_highlight_image: Handle<Image>,
    pub pawn_heads_image: Handle<Image>,
    pub pawn_heads_layout: Handle<TextureAtlasLayout>,
    pub pawn_bodies_image: Handle<Image>,
    pub pawn_bodies_layout: Handle<TextureAtlasLayout>,
    pub close_icon: Handle<Image>,
    pub error_icon: Handle<Image>,
    pub brick_image: Handle<Image>,
}

pub struct PbAssetsPlugin;

impl Plugin for PbAssetsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, load);
    }
}

pub fn load(mut commands: Commands, server: Res<AssetServer>) {
    commands.insert_resource(AssetHandles {
        font_graduate: server.load("fonts/Graduate-Regular.ttf"),
        font_tomorrow: server.load("fonts/Tomorrow-Medium.ttf"),
        tomorrow_italic_font: server.load("fonts/Tomorrow-MediumItalic.ttf"),
        button_image: server.load("image/button.png"),
        ribbon_button_image: server.load("image/ribbon_button.png"),
        ribbon_button_delete_wall_image: server.load("image/ribbon_button_delete_wall.png"),
        ribbon_button_wall_image: server.load("image/ribbon_button_wall.png"),
        ribbon_button_door_image: server.load("image/ribbon_button_door.png"),
        bevy_icon: server.load("image/bevy.png"),
        github_icon: server.load("image/github.png"),
        pawn_image: server.load("image/pawn.png"),
        pawn_highlight_image: server.load("image/pawn_highlight.png"),
        pawn_heads_image: server.load("image/pawn_heads.png"),
        pawn_heads_layout: server.add(TextureAtlasLayout::from_grid(
            UVec2::splat(128),
            7,
            1,
            None,
            None,
        )),
        pawn_bodies_layout: server.add(TextureAtlasLayout::from_grid(
            UVec2::splat(128),
            4,
            1,
            None,
            None,
        )),
        pawn_bodies_image: server.load("image/pawn_bodies.png"),
        close_icon: server.load("image/close.png"),
        error_icon: server.load("image/error.png"),
        brick_image: server.load_with_settings(
            "image/brick.png",
            |settings: &mut ImageLoaderSettings| {
                settings.sampler = ImageSampler::Descriptor(ImageSamplerDescriptor {
                    address_mode_u: ImageAddressMode::Repeat,
                    address_mode_v: ImageAddressMode::ClampToEdge,
                    min_filter: ImageFilterMode::Linear,
                    mag_filter: ImageFilterMode::Linear,
                    ..default()
                });
            },
        ),
    });
}

impl AssetHandles {
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

    pub async fn wait_for_load(&self, server: &AssetServer) -> Result {
        for id in self.asset_ids() {
            server.wait_for_asset_id(id).await?;
        }

        Ok(())
    }

    fn asset_ids(&self) -> impl Iterator<Item = UntypedAssetId> + '_ {
        let AssetHandles {
            font_graduate,
            font_tomorrow,
            tomorrow_italic_font,
            button_image,
            ribbon_button_image,
            ribbon_button_wall_image,
            ribbon_button_delete_wall_image,
            ribbon_button_door_image,
            bevy_icon,
            github_icon,
            pawn_image,
            pawn_highlight_image,
            pawn_heads_image,
            pawn_heads_layout,
            pawn_bodies_image,
            pawn_bodies_layout,
            close_icon,
            error_icon,
            brick_image,
        } = self;

        [
            font_graduate.into(),
            font_tomorrow.into(),
            tomorrow_italic_font.into(),
            button_image.into(),
            ribbon_button_image.into(),
            ribbon_button_wall_image.into(),
            ribbon_button_delete_wall_image.into(),
            ribbon_button_door_image.into(),
            bevy_icon.into(),
            github_icon.into(),
            pawn_image.into(),
            pawn_highlight_image.into(),
            pawn_heads_image.into(),
            pawn_heads_layout.into(),
            pawn_bodies_image.into(),
            pawn_bodies_layout.into(),
            close_icon.into(),
            error_icon.into(),
            brick_image.into(),
        ]
        .into_iter()
    }
}
