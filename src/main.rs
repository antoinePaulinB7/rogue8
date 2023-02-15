mod world;

use world::world_state::WorldState;

fn main() {
    let mut world = WorldState::default();

    println!("Hello, {}!", &world.get_name());
}
