use super::actor::Actor;
use crate::world::action::Action;
use crate::world::world_map::WorldMap;

pub struct WorldState {
    name: String,
    seed: usize,
    actors: Vec<Actor>,
    map: WorldMap,
}

impl WorldState {
    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_map(&self) -> &WorldMap {
        &self.map
    }

    pub fn process(&self, action: Action) -> Self {
        match action {
            Action::Move(_position) => todo!(),
            Action::Look(_positions) => todo!(),
            Action::Hit(_position) => todo!(),
            Action::Use(_item) => todo!(),
            Action::Throw(_item, _position) => todo!(),
            Action::Shoot(_ammo, _position) => todo!(),
            Action::Activate(_position) => todo!(),
            Action::Pickup(_item, _quantity) => todo!(),
            Action::Drop(_item, _quantity) => todo!(),
            Action::Wait(_turns) => todo!(),
        }
    }
}

impl Default for WorldState {
    fn default() -> Self {
        Self {
            name: String::from("World 1"),
            seed: Default::default(),
            actors: Default::default(),
            map: Default::default(),
        }
    }
}
