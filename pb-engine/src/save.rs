use std::iter::once;

use anyhow::{Context, Result, bail};
use avian2d::prelude::LinearVelocity;
use bevy::{
    ecs::{
        entity::EntityHashMap,
        system::{SystemParam, SystemState},
    },
    prelude::*,
    reflect::TypeRegistryArc,
    scene::serde::{SceneDeserializer, SceneSerializer},
};
use serde::{Deserializer, Serialize, Serializer, de::DeserializeSeed};

use crate::{
    map::{Map, MapLayer},
    pawn::Pawn,
    root::Root,
    wall::{Vertex, Wall},
};

#[derive(TypePath)]
pub struct Save {
    scene: DynamicScene,
    registry: TypeRegistryArc,
}

pub struct LoadSeed {
    registry: TypeRegistryArc,
}

// TODO: necessary?
#[derive(SystemParam)]
pub struct SaveParam<'w, 's> {
    children: Query<'w, 's, &'static Children>,
    registry: Res<'w, AppTypeRegistry>,
}

#[derive(SystemParam)]
pub struct LoadParam<'w, 's> {
    commands: Commands<'w, 's>,
    roots: Query<'w, 's, &'static Root>,
}

pub fn save(world: &World, param: &SaveParam, root: Entity) -> Save {
    let scene = DynamicSceneBuilder::from_world(world)
        .allow_component::<Pawn>()
        .allow_component::<Wall>()
        .allow_component::<Vertex>()
        .allow_component::<Root>()
        .allow_component::<Name>()
        .allow_component::<Map>()
        .allow_component::<MapLayer>()
        .allow_component::<Transform>()
        .allow_component::<LinearVelocity>()
        .allow_component::<ChildOf>()
        .allow_component::<Children>()
        .extract_entities(once(root).chain(param.children.iter_descendants(root)))
        .remove_empty_entities()
        .build();

    Save {
        scene,
        registry: param.registry.0.clone(),
    }
}

pub fn load(world: &mut World, param: &mut SystemState<LoadParam>, save: &Save) -> Result<Entity> {
    let mut entities = EntityHashMap::default();
    save.scene
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

impl Serialize for Save {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        SceneSerializer {
            scene: &self.scene,
            registry: &self.registry.read(),
        }
        .serialize(serializer)
    }
}

impl LoadSeed {
    pub fn new(registry: TypeRegistryArc) -> Self {
        LoadSeed { registry }
    }
}

impl<'de> DeserializeSeed<'de> for LoadSeed {
    type Value = Save;

    fn deserialize<D>(self, deserializer: D) -> std::result::Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        let scene = SceneDeserializer {
            type_registry: &self.registry.read(),
        }
        .deserialize(deserializer)?;

        Ok(Save {
            scene,
            registry: self.registry.clone(),
        })
    }
}
