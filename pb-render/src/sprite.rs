use bevy::prelude::*;

use pb_assets::AssetHandles;
use pb_engine::{
    pawn::{self, Pawn},
    root::Root,
};
use rand::{Rng, SeedableRng, rngs::SmallRng};

pub fn root_added(trigger: Trigger<OnAdd, Root>, mut commands: Commands) {
    commands
        .entity(trigger.target())
        .insert(Visibility::default());
}

pub fn pawn_added(
    trigger: Trigger<OnAdd, Pawn>,
    mut commands: Commands,
    assets: Res<AssetHandles>,
) -> Result {
    let mut rng = SmallRng::from_os_rng();
    let head = rng.random_range(0..7);
    let body = rng.random_range(0..4);

    commands
        .entity(trigger.target())
        .insert(Visibility::Visible);
    commands.spawn((
        Sprite {
            image: assets.pawn_heads_image.clone(),
            custom_size: Some(Vec2::splat(pawn::RADIUS * 4.)),
            texture_atlas: Some(TextureAtlas {
                layout: assets.pawn_heads_layout.clone(),
                index: head,
            }),
            color: Color::Srgba(Srgba::hex("d7bd96")?),
            ..default()
        },
        ChildOf(trigger.target()),
    ));
    commands.spawn((
        Sprite {
            image: assets.pawn_bodies_image.clone(),
            custom_size: Some(Vec2::splat(pawn::RADIUS * 4.)),
            texture_atlas: Some(TextureAtlas {
                layout: assets.pawn_bodies_layout.clone(),
                index: body,
            }),
            color: Color::Srgba(Srgba::hex("f33a02")?),
            ..default()
        },
        ChildOf(trigger.target()),
    ));
    Ok(())
}
