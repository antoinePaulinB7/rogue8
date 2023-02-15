mod crossterm_plug;
mod terminal_engine;
mod world;

use crossterm_plug::CrosstermEngine;
use terminal_engine::{Color, ColorTile, TerminalColor, TerminalPrintable};

use world::world_state::WorldState;

fn main() {
    let world = WorldState::default();

    println!("Hello, {}!", &world.get_name());
    let mut term = CrosstermEngine {
        output: std::io::stdout(),
    };
    term.clear();
    term.print(
        'a',
        (10, 10),
        ColorTile {
            foreground: TerminalColor::Light(Color::Red),
            background: TerminalColor::Dark(Color::Green),
        },
    );

    for (position, tile) in world.get_map().get_tiles() {
        term.print(
            'X',
            position.into(),
            ColorTile {
                foreground: TerminalColor::Light(Color::Red),
                background: TerminalColor::Dark(Color::Green),
            },
        );
    }
}
