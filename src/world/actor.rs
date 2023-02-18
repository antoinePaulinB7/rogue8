use crate::world::item::Item;
use crate::world::world_map::Position;

#[derive(Clone, PartialEq, Eq)]
pub struct Actor {
    pub position: Position,
    pub inventory: Vec<(Item, usize)>,
}
