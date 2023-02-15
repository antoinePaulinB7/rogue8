use crate::world::item::Equipment;
use crate::world::item::Item;
use crate::world::map::Position;

pub enum Action {
    Move(Position),
    Look(Vec<Position>),
    Hit(Position),
    Use(Item),
    Throw(Item, Position),
    Shoot(Equipment, Position),
    Activate(Position),
    Pickup(Item, usize),
    Drop(Item, usize),
    Wait(usize),
}
