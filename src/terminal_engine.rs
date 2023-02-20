
#[derive(Debug)]
pub enum TerminalError {
    PositionOutOfBounds,
    Io(std::io::Error)
}

impl From<std::io::Error> for TerminalError {
    fn from(error: std::io::Error) -> Self {
        TerminalError::Io(error)
    }
}

#[derive(Clone)]
pub enum Color {
    Black, Red, Green, Yellow, Blue, Magenta, Cyan, White
}

#[derive(Clone)]
pub enum TerminalColor {
    Light(Color),
    Dark(Color)
}

pub struct ColorTile {
    pub background: TerminalColor,
    pub foreground: TerminalColor
}

pub struct TerminalCharacter {
    pub character: char,
    pub color: ColorTile,
}

pub trait TerminalPrintable {
    fn print(&mut self, ch: TerminalCharacter, position: (usize, usize)) -> Result<(), TerminalError>;
    fn clear(&mut self) -> Result<(), TerminalError>;
    fn refresh(&mut self) -> Result<(), TerminalError>;
}
