use crate::world::item::Item;
use crate::world::world_map::Position;

pub struct Actor {
    position: Position,
    inventory: Vec<(Item, usize)>,
}
