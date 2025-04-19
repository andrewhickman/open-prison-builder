use std::f32::consts::PI;

use bevy::prelude::*;
use pb_engine::pawn::{ai::path::PathTaskBundle, Pawn, PawnBundle};
use rand::{
    distr::{Distribution, Uniform},
    rngs::SmallRng,
    SeedableRng,
};

pub fn spawn_1000_pawns(_: Trigger<Pointer<Click>>, mut commands: Commands) {
    let mut rng = SmallRng::from_os_rng();
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
}

pub fn create_path_tasks(
    _: Trigger<Pointer<Click>>,
    mut commands: Commands,
    query: Query<Entity, With<Pawn>>,
) {
    let mut rng = SmallRng::from_os_rng();
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
}
