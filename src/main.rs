mod crossterm_plug;
mod terminal_engine;

use crossterm_plug::CrosstermEngine;
use terminal_engine::{TerminalPrintable, ColorTile, TerminalColor, Color, TerminalCharacter};

fn main() {
    let mut term = CrosstermEngine {output: std::io::stdout()};
    term.clear();
    term.print(
        TerminalCharacter {
            character: 'a',
            color: ColorTile {foreground: TerminalColor::Light(Color::Green), background: TerminalColor::Dark(Color::Green)}
        },
        (20,20), 
        ).unwrap();
}
