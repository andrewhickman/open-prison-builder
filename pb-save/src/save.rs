use std::iter::once;

use anyhow::{bail, Context, Result};
use bevy::{
    ecs::{
        entity::EntityHashMap,
        system::{SystemParam, SystemState},
    },
    prelude::*,
};
use bevy_rapier2d::dynamics::Velocity;
use chrono::{DateTime, Local, Utc};
use serde::{Deserialize, Serialize};
use smol_str::SmolStr;

use pb_engine::{pawn::Pawn, Root};

#[derive(Component)]
pub struct Save {
    pub name: SmolStr,
}

#[derive(Serialize, Deserialize, Component)]
pub struct SaveMetadata {
    pub name: SmolStr,
    pub modified: DateTime<Utc>,
}

#[derive(SystemParam)]
pub struct SaveParam<'w, 's> {
    children: Query<'w, 's, &'static Children>,
}

#[derive(SystemParam)]
pub struct LoadParam<'w, 's> {
    commands: Commands<'w, 's>,
    roots: Query<'w, 's, &'static Root>,
}

pub fn save(world: &World, param: &SaveParam, root: Entity) -> DynamicScene {
    DynamicSceneBuilder::from_world(world)
        .allow::<Pawn>()
        .allow::<Root>()
        .allow::<Transform>()
        .allow::<Velocity>()
        .allow::<GlobalTransform>()
        .allow::<Parent>()
        .allow::<Children>()
        .extract_entities(once(root).chain(param.children.iter_descendants(root)))
        .remove_empty_entities()
        .build()
}

pub fn load(
    world: &mut World,
    param: &mut SystemState<LoadParam>,
    scene: DynamicScene,
) -> Result<Entity> {
    let mut entities = EntityHashMap::default();
    scene
        .write_to_world(world, &mut entities)
        .context("failed to load save")?;

    let mut param = param.get(world);
    let root = entities
        .values()
        .copied()
        .find(|&entity| param.roots.contains(entity));
    let Some(root) = root else {
        for &entity in entities.values() {
            param.commands.entity(entity).despawn();
        }
        bail!("no root entity found in save")
    };

    Ok(root)
}

impl SaveMetadata {
    pub fn new(name: impl Into<SmolStr>) -> Self {
        SaveMetadata {
            name: name.into(),
            modified: Utc::now(),
        }
    }

    pub fn modified_local(&self) -> DateTime<Local> {
        self.modified.into()
    }
}
