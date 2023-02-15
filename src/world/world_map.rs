use std::collections::BTreeMap;

use crate::world::constants;

#[derive(Eq, Ord, PartialEq, PartialOrd)]
pub struct Position {
    x: usize,
    y: usize,
}

impl From<(usize, usize)> for Position {
    fn from((x, y): (usize, usize)) -> Self {
        Self { x, y }
    }
}

impl Into<(usize, usize)> for &Position {
    fn into(self) -> (usize, usize) {
        (self.x, self.y)
    }
}

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
                    tiles.insert(Position { x, y }, Tile::wall());
                }
            }
        }

        Self { tiles }
    }
}
