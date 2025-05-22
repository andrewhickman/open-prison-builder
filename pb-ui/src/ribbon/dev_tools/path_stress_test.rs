use std::f32::consts::PI;

use bevy::prelude::*;
use pb_engine::pawn::{Pawn, PawnBundle, ai::path::PathTaskBundle};
use pb_util::rng::LocalRng;
use rand::distr::{Distribution, Uniform};

pub fn spawn_1000_pawns(
    _: Trigger<Pointer<Click>>,
    mut commands: Commands,
    mut rng: LocalRng,
) -> Result {
    let position_distr = Uniform::new(-100., 100.).unwrap();
    let rotation_distr = Uniform::new(-PI, PI).unwrap();

    let pawns: Vec<PawnBundle> = (0..1000)
        .map(|_| {
            let position = Vec2::new(
                position_distr.sample(&mut rng),
                position_distr.sample(&mut rng),
            );
            let rotation = rotation_distr.sample(&mut rng);

            PawnBundle::new(position, rotation)
        })
        .collect();

    commands.spawn_batch(pawns);
    Ok(())
}

pub fn create_path_tasks(
    _: Trigger<Pointer<Click>>,
    mut commands: Commands,
    query: Query<Entity, With<Pawn>>,
    mut rng: LocalRng,
) -> Result {
    let position_distr = Uniform::new(-100., 100.).unwrap();

    let tasks: Vec<PathTaskBundle> = query
        .iter()
        .map(|entity| {
            let position = Vec2::new(
                position_distr.sample(&mut rng),
                position_distr.sample(&mut rng),
            );

            PathTaskBundle::move_to(entity, position)
        })
        .collect();

    info!("created {} path tasks", tasks.len());

    commands.spawn_batch(tasks);
    Ok(())
}
