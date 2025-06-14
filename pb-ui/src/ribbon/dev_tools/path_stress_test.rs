use std::f32::consts::PI;

use bevy::prelude::*;
use pb_engine::pawn::Pawn;
use pb_util::rng::LocalRng;
use rand::distr::{Distribution, Uniform};

pub fn spawn_1000_pawns(
    _: Trigger<Pointer<Click>>,
    mut commands: Commands,
    mut rng: LocalRng,
) -> Result {
    let position_distr = Uniform::new(-100., 100.).unwrap();
    let rotation_distr = Uniform::new(-PI, PI).unwrap();

    let pawns: Vec<_> = (0..1000)
        .map(|_| {
            let position = Vec2::new(
                position_distr.sample(&mut rng),
                position_distr.sample(&mut rng),
            );
            let rotation = rotation_distr.sample(&mut rng);

            Pawn::bundle(position, rotation)
        })
        .collect();

    commands.spawn_batch(pawns);
    Ok(())
}

pub fn create_path_tasks(_: Trigger<Pointer<Click>>) -> Result {
    Ok(())
}
