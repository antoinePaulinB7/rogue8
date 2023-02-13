mod crossterm_plug;
mod terminal_engine;

use crossterm_plug::CrosstermEngine;
use terminal_engine::{TerminalPrintable, ColorTile, TerminalColor, Color};

fn main() {
    let mut term = CrosstermEngine {output: std::io::stdout()};
    term.clear();
    term.print(
        'a', 
        (10,10), 
        ColorTile {foreground: TerminalColor::Light(Color::Red), background: TerminalColor::Dark(Color::Green)});
}
