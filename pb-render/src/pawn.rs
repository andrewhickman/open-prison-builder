use bevy::prelude::*;

use pb_assets::AssetHandles;
use pb_engine::pawn::Pawn;
use pb_util::rng::LocalRng;
use rand::{Rng, seq::IndexedRandom};

use crate::layer;

const PAWN_SPRITE_SIZE: Vec2 = Vec2::splat(Pawn::RADIUS * 4.);

#[derive(Default, Copy, Clone, Component)]
pub struct PawnSprite;

#[derive(Default, Copy, Clone, Component)]
pub struct PawnHighlight;

pub fn pawn_added(
    trigger: Trigger<OnAdd, Pawn>,
    mut commands: Commands,
    assets: Res<AssetHandles>,
    mut rng: LocalRng,
) -> Result {
    let head = rng.random_range(0..7);
    let body = rng.random_range(0..4);
    let skin = random_skin_tone(&mut rng);
    let uniform = random_uniform(&mut rng);

    commands
        .entity(trigger.target())
        .insert(Visibility::Visible);
    commands.spawn((
        PawnSprite,
        Transform::from_xyz(0., 0., layer::PAWN_HEAD),
        Visibility::Visible,
        Sprite {
            image: assets.pawn_heads_image.clone(),
            custom_size: Some(PAWN_SPRITE_SIZE),
            texture_atlas: Some(TextureAtlas {
                layout: assets.pawn_heads_layout.clone(),
                index: head,
            }),
            color: skin,
            ..default()
        },
        ChildOf(trigger.target()),
    ));
    commands.spawn((
        PawnSprite,
        Transform::from_xyz(0., 0., layer::PAWN_BODY),
        Visibility::Visible,
        Sprite {
            image: assets.pawn_bodies_image.clone(),
            custom_size: Some(PAWN_SPRITE_SIZE),
            texture_atlas: Some(TextureAtlas {
                layout: assets.pawn_bodies_layout.clone(),
                index: body,
            }),
            color: uniform,
            ..default()
        },
        ChildOf(trigger.target()),
    ));
    Ok(())
}

pub fn clear_rotation(
    mut sprite_q: Query<(&mut Transform, &ChildOf), With<PawnSprite>>,
    parent_q: Query<&Transform, Without<PawnSprite>>,
) {
    sprite_q.par_iter_mut().for_each(|(mut transform, parent)| {
        if let Ok(parent_transform) = parent_q.get(parent.parent()) {
            transform.rotation = parent_transform.rotation.inverse();
        }
    });
}

impl PawnHighlight {
    pub fn bundle(assets: &AssetHandles, pawn: Entity, color: Color) -> impl Bundle {
        (
            Transform::from_xyz(0., 0., layer::PAWN_HIGHLIGHT),
            Visibility::Inherited,
            Sprite {
                image: assets.pawn_highlight_image.clone(),
                custom_size: Some(PAWN_SPRITE_SIZE),
                color,
                ..default()
            },
            ChildOf(pawn),
        )
    }
}

fn random_skin_tone(rng: &mut impl Rng) -> Color {
    static SKIN_TONES: [Hsla; 10] = [
        Hsla::hsl(30.0, 0.500, 0.929),
        Hsla::hsl(30.0, 0.500, 0.906),
        Hsla::hsl(40.0, 0.709, 0.892),
        Hsla::hsl(40.0, 0.533, 0.824),
        Hsla::hsl(36.0, 0.448, 0.716),
        Hsla::hsl(32.4, 0.301, 0.482),
        Hsla::hsl(23.8, 0.320, 0.386),
        Hsla::hsl(17.7, 0.297, 0.290),
        Hsla::hsl(26.3, 0.160, 0.196),
        Hsla::hsl(26.7, 0.123, 0.143),
    ];

    SKIN_TONES.choose(rng).copied().unwrap().into()
}

fn random_uniform(rng: &mut impl Rng) -> Color {
    static UNIFORMS: [Hsla; 5] = [
        Hsla::hsl(194., 0.71, 0.52),
        Hsla::hsl(39., 0.98, 0.53),
        Hsla::hsl(14., 0.98, 0.48),
        Hsla::hsl(71., 0.88, 0.49),
        Hsla::hsl(291., 0.51, 0.92),
    ];

    UNIFORMS.choose(rng).copied().unwrap().into()
}
