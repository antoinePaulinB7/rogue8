use crate::world::item::Item;
use crate::world::map::Position;

pub struct Actor {
    position: Position,
    inventory: Vec<(Item, usize)>,
}
