use bevy_ecs_tilemap::tiles::{TilePos, TileTextureIndex};

use crate::map::{LARGE_TILE_OFFSET, LARGE_TILE_SIZE, TILE_SIZE};

pub enum Material {
    Dirt,
    Grass,
}

impl Material {
    pub fn index(&self, pos: TilePos) -> TileTextureIndex {
        let base = match self {
            Material::Dirt => TileTextureIndex(0),
            Material::Grass => TileTextureIndex(LARGE_TILE_OFFSET),
        };

        if self.is_large() {
            large_texture_id(base, pos)
        } else {
            base
        }
    }

    fn is_large(&self) -> bool {
        matches!(self, Material::Dirt | Material::Grass)
    }
}

fn large_texture_id(base: TileTextureIndex, pos: TilePos) -> TileTextureIndex {
    let len = LARGE_TILE_SIZE / TILE_SIZE;
    TileTextureIndex(base.0 + (len - 1 - (pos.y % len)) * len + (pos.x % len))
}
