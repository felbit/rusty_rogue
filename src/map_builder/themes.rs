use crate::prelude::*;

// *** DUNGEON THEME ***
pub struct DungeonTheme {}

impl DungeonTheme {
    pub fn new() -> Box<dyn MapTheme> {
        Box::new(Self {})
    }
}

impl MapTheme for DungeonTheme {
    fn tile_to_render(&self, tile_type: TileType) -> FontCharType {
        match tile_type {
            TileType::Exit => to_cp437('>'),
            TileType::Floor => to_cp437('.'),
            TileType::Wall => to_cp437('#'),
        }
    }
}

// *** FOREST THEME ***
pub struct ForestTheme {}

impl ForestTheme {
    pub fn new() -> Box<dyn MapTheme> {
        Box::new(Self {})
    }
}

impl MapTheme for ForestTheme {
    fn tile_to_render(&self, tile_type: TileType) -> FontCharType {
        match tile_type {
            TileType::Exit => to_cp437('>'),
            TileType::Floor => to_cp437(';'),
            TileType::Wall => to_cp437('"'),
        }
    }
}
