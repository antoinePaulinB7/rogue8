use std::ops::Index;

use super::actor::Actor;
use crate::world::action::Action;
use crate::world::world_map::WorldMap;

#[derive(Clone)]
pub struct WorldState {
    name: String,
    seed: usize,
    actors: Vec<Actor>,
    map: WorldMap,
    history: Vec<Action>,
}

impl WorldState {
    pub fn create(
        name: String,
        seed: usize,
        actors: Vec<Actor>,
        map: WorldMap,
        history: Vec<Action>,
    ) -> Self {
        WorldState {
            name,
            seed,
            actors,
            map,
            history,
        }
    }

    pub fn add_actor(&mut self, actor: Actor) {
        self.actors.push(actor);
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_map(&self) -> &WorldMap {
        &self.map
    }

    pub fn get_actors(&self) -> &Vec<Actor> {
        &self.actors
    }

    pub fn process(&self, actor: Actor, action: Action) -> Self {
        let mut new_state = self.clone();

        let mut new_actor = new_state
            .actors
            .iter_mut()
            .find(|some_actor| **some_actor == actor)
            .unwrap();

        match action {
            Action::Move(_position) => new_actor.position = _position,
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

        new_state.history.push(action);

        new_state
    }
}

impl Default for WorldState {
    fn default() -> Self {
        Self {
            name: String::from("World 1"),
            seed: usize::default(),
            actors: vec![],
            map: WorldMap::default(),
            history: vec![],
        }
    }
}
