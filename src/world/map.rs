use std::collections::BTreeMap;

use crate::world::constants;

#[derive(Eq, Ord, PartialEq, PartialOrd)]
pub struct Position {
    x: usize,
    y: usize,
}
pub struct Tile {
    info: String,
    block_movement: bool,
    block_sight: bool,
}

impl Tile {
    pub fn wall() -> Self {
        Tile {
            info: constants::wall.to_string(),
            block_movement: true,
            block_sight: true,
        }
    }
}

pub struct WorldMap {
    tiles: BTreeMap<Position, Tile>,
}

impl Default for WorldMap {
    fn default() -> Self {
        let mut tiles: BTreeMap<Position, Tile> = BTreeMap::new();

        for x in 0..80 {
            for y in 0..50 {
                if x == 0 || x == 79 || y == 0 || y == 49 {
                    tiles.insert(Position { x, y }, Tile::wall());
                }
            }
        }

        Self { tiles }
    }
}
