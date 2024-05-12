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

    pub fn iter() -> Vec<Material> {
        vec![Material::Dirt, Material::Grass]
    }

    fn is_large(&self) -> bool {
        matches!(self, Material::Dirt | Material::Grass)
    }
}

fn large_texture_id(base: TileTextureIndex, pos: TilePos) -> TileTextureIndex {
    let len = LARGE_TILE_SIZE / TILE_SIZE;
    TileTextureIndex(base.0 + (len - 1 - (pos.y % len)) * len + (pos.x % len))
}

#[allow(clippy::too_many_arguments)]
fn wall_index(tl: bool, t: bool, tr: bool, r: bool, br: bool, b: bool, bl: bool, l: bool) -> u32 {
    match (tl, t, tr, r, br, b, bl, l) {
        // Isolated wall
        (_, false, _, false, _, false, _, false) => 0,
        // End walls
        (_, true, _, false, _, false, _, false) => 1,
        (_, false, _, true, _, false, _, false) => 2,
        (_, false, _, false, _, true, _, false) => 3,
        (_, false, _, false, _, false, _, true) => 4,
        // Top-right corner
        (_, true, false, true, _, false, _, false) => 5,
        (_, true, true, true, _, false, _, false) => 6,
        // Bottom-right corner
        (_, false, _, true, false, true, _, false) => 7,
        (_, false, _, true, true, true, _, false) => 8,
        // Bottom-left corner
        (_, false, _, false, _, true, false, true) => 9,
        (_, false, _, false, _, true, true, true) => 10,
        // Top-left corner
        (false, true, _, false, _, false, _, true) => 11,
        (true, true, _, false, _, false, _, true) => 12,
        // Vertical straight
        (_, true, _, false, _, true, _, false) => 13,
        // Horizontal straight
        (_, false, _, true, _, false, _, true) => 14,
        // Top t-junction
        (false, true, false, true, _, false, _, true) => 15,
        (true, true, false, true, _, false, _, true) => 16,
        (false, true, true, true, _, false, _, true) => 17,
        (true, true, true, true, _, false, _, true) => 18,
        // Right t-junction
        (_, true, false, true, false, true, _, false) => 19,
        (_, true, true, true, false, true, _, false) => 20,
        (_, true, false, true, true, true, _, false) => 21,
        (_, true, true, true, true, true, _, false) => 22,
        // Bottom t-junction
        (_, false, _, true, false, true, false, true) => 23,
        (_, false, _, true, true, true, false, true) => 24,
        (_, false, _, true, false, true, true, true) => 25,
        (_, false, _, true, true, true, true, true) => 26,
        // Left t-junction
        (false, true, _, false, _, true, false, true) => 27,
        (true, true, _, false, _, true, false, true) => 28,
        (false, true, _, false, _, true, true, true) => 29,
        (true, true, _, false, _, true, true, true) => 30,
        // 4-way intersection
        (false, true, false, true, false, true, false, true) => 31,
        (true, true, false, true, false, true, false, true) => 32,
        (false, true, true, true, false, true, false, true) => 33,
        (true, true, true, true, false, true, false, true) => 34,
        (false, true, false, true, true, true, false, true) => 35,
        (true, true, false, true, true, true, false, true) => 36,
        (false, true, true, true, true, true, false, true) => 37,
        (true, true, true, true, true, true, false, true) => 38,
        (false, true, false, true, false, true, true, true) => 39,
        (true, true, false, true, false, true, true, true) => 40,
        (false, true, true, true, false, true, true, true) => 41,
        (true, true, true, true, false, true, true, true) => 42,
        (false, true, false, true, true, true, true, true) => 43,
        (true, true, false, true, true, true, true, true) => 44,
        (false, true, true, true, true, true, true, true) => 45,
        (true, true, true, true, true, true, true, true) => 46,
    }
}
