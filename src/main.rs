mod crossterm_plug;
mod terminal_engine;
mod world;

use crossterm_plug::CrosstermEngine;
use terminal_engine::{Color, ColorTile, TerminalColor, TerminalPrintable};

use world::world_state::WorldState;

use crate::world::{action::Action, actor::Actor, world_map::Position};

fn print_world(term: &mut CrosstermEngine, world: &WorldState) {
    term.clear();
    // term.print(
    //     'a',
    //     (10, 10),
    //     ColorTile {
    //         foreground: TerminalColor::Light(Color::Red),
    //         background: TerminalColor::Dark(Color::Green),
    //     },
    // );

    for (&position, tile) in world.get_map().get_tiles() {
        term.print(
            'X',
            position,
            ColorTile {
                foreground: TerminalColor::Light(Color::Red),
                background: TerminalColor::Dark(Color::Green),
            },
        );
    }

    for actor in world.get_actors() {
        let a = Actor {
            position: actor.position,
            inventory: actor.inventory.clone(),
        };

        let p_tuple: (usize, usize) = a.position.into();

        term.print(
            '@',
            p_tuple,
            ColorTile {
                foreground: TerminalColor::Light(Color::Blue),
                background: TerminalColor::Dark(Color::Yellow),
            },
        );
    }

    term.refresh();
}

fn main() {
    let player = Actor {
        position: (10, 11),
        inventory: vec![],
    };

    let mut world = WorldState::default();
    world.add_actor(player.clone());

    println!("Hello, {}!", &world.get_name());
    let mut term = CrosstermEngine {
        output: std::io::stdout(),
    };

    print_world(&mut term, &world);
    let new_world = world.process(player, Action::Move((10, 12)));
    print_world(&mut term, &new_world);
    let b = 1;
}
