/// # ASCIITextDisplay
///
/// ASCIITextDisplay is used to provide simple text-based displays
/// (equivalent to IBM PC VGA)
///
/// The set of functionality provided here is quite minimal, as the actual TTY
/// or console code should reside in user mode, but it is useful actually having
/// output in the kernel.
///
pub trait ASCIITextDisplay {
    /// Returns the dimensions of the display
    fn dimensions(&self) -> (usize, usize);

    /// Returns a tuple of the ASCII character, foreground color,
    /// and background color)
    /// Will return `None` if x or y is out of bounds.
    fn get(&self, x: usize, y: usize) -> Option<(u8, Color, Color)>;

    /// Sets the character at the specified coordinates.
    /// Fails silently if coordinates are out of bounds.
    fn set(&mut self, ascii_character: u8, fg: Color, bg: Color, x: usize, y: usize);
    /// Copies a character. This is useful for implementing scrolling,
    /// and also allows for optimizing that implementation.
    fn copy(
        &mut self,
        source_x: usize,
        source_y: usize,
        destination_x: usize,
        destination_y: usize,
    );
}

/// VGA colors
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Color {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}

// TODO: Just transmute here?
pub fn color_from_u8(n: u8) -> Option<Color> {
    match n {
        0 => Some(Color::Black),
        1 => Some(Color::Blue),
        2 => Some(Color::Green),
        3 => Some(Color::Cyan),
        4 => Some(Color::Red),
        5 => Some(Color::Magenta),
        6 => Some(Color::Brown),
        7 => Some(Color::LightGray),
        8 => Some(Color::DarkGray),
        9 => Some(Color::LightBlue),
        10 => Some(Color::LightGreen),
        11 => Some(Color::LightCyan),
        12 => Some(Color::LightRed),
        13 => Some(Color::Pink),
        14 => Some(Color::Yellow),
        15 => Some(Color::White),
        _ => None,
    }
}
