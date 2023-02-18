use std::collections::BTreeMap;

use crate::world::constants;

pub type Position = (usize, usize);

#[derive(Clone)]
pub struct Tile {
    info: String,
    block_movement: bool,
    block_sight: bool,
}

impl Tile {
    pub fn wall() -> Self {
        Tile {
            info: constants::WALL.to_string(),
            block_movement: true,
            block_sight: true,
        }
    }
}

#[derive(Clone)]
pub struct WorldMap {
    tiles: BTreeMap<Position, Tile>,
}

impl WorldMap {
    pub fn get_tiles(&self) -> &BTreeMap<Position, Tile> {
        &self.tiles
    }
}

impl Default for WorldMap {
    fn default() -> Self {
        let mut tiles: BTreeMap<Position, Tile> = BTreeMap::new();

        for x in 0..80 {
            for y in 0..50 {
                if x == 0 || x == 79 || y == 0 || y == 49 {
                    tiles.insert((x, y), Tile::wall());
                }
            }
        }

        Self { tiles }
    }
}
