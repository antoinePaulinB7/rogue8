use crate::terminal_engine::{Color, ColorTile, TerminalColor, TerminalError, TerminalPrintable};
use std::io::Write;

pub struct CrosstermEngine {
    pub output: std::io::Stdout,
}

impl Into<crossterm::style::Color> for TerminalColor {
    fn into(self) -> crossterm::style::Color {
        match self {
            Self::Light(col) => match col {
                Color::Black => crossterm::style::Color::DarkGrey,
                Color::Red => crossterm::style::Color::Red,
                Color::Green => crossterm::style::Color::Green,
                Color::Yellow => crossterm::style::Color::Yellow,
                Color::Blue => crossterm::style::Color::Blue,
                Color::Magenta => crossterm::style::Color::Magenta,
                Color::Cyan => crossterm::style::Color::Cyan,
                Color::White => crossterm::style::Color::White,
            },
            Self::Dark(col) => match col {
                Color::Black => crossterm::style::Color::Black,
                Color::Red => crossterm::style::Color::DarkRed,
                Color::Green => crossterm::style::Color::DarkGreen,
                Color::Yellow => crossterm::style::Color::DarkYellow,
                Color::Blue => crossterm::style::Color::DarkBlue,
                Color::Magenta => crossterm::style::Color::DarkMagenta,
                Color::Cyan => crossterm::style::Color::DarkCyan,
                Color::White => crossterm::style::Color::Grey,
            },
        }
    }
}

impl ColorTile {
    fn to_crossterm_colors(&self) -> crossterm::style::Colors {
        crossterm::style::Colors::new(
            self.foreground.clone().into(),
            self.background.clone().into(),
        )
    }
}

impl TerminalPrintable for CrosstermEngine {
    fn print(
        &mut self,
        character: char,
        position: (usize, usize),
        color: crate::terminal_engine::ColorTile,
    ) -> Result<(), crate::terminal_engine::TerminalError> {
        crossterm::queue!(
            self.output,
            crossterm::cursor::MoveTo(position.0 as u16, position.1 as u16),
            crossterm::style::SetColors(color.to_crossterm_colors())
        )?;
        let buffer = [character as u8];
        self.output.write(&buffer[..]).unwrap();
        crossterm::queue!(self.output, crossterm::style::ResetColor)?;
        Ok(())
    }
    fn clear(&mut self) -> Result<(), crate::terminal_engine::TerminalError> {
        crossterm::queue!(
            self.output,
            crossterm::terminal::Clear(crossterm::terminal::ClearType::All),
        )?;
        self.output.flush()?;
        Ok(())
    }
    fn refresh(&mut self) -> Result<(), crate::terminal_engine::TerminalError> {
        self.output.flush()?;
        Ok(())
    }
}
