// basic interface for interacting with the VGA buffer

use hal::ascii_text_display::*; 

const BUFFER_WIDTH: usize = 80;
const BUFFER_HEIGHT: usize = 25;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct ColorCode(u8);

impl ColorCode {
    fn new(foreground: Color, background: Color) -> ColorCode {
        ColorCode((background as u8) << 4 | (foreground as u8))
    }
    fn recover(&self) -> (Color, Color) {
        ( color_from_u8((self.0) & 0b00001111).unwrap_or(Color::Black), color_from_u8((self.0) >> 4).unwrap_or(Color::White) )
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct ScreenChar {
    ascii_character: u8,
    color_code: ColorCode,
}
impl ScreenChar {
    fn recover(&self) -> (u8, Color, Color) {
        let c = self.color_code.recover();
        (self.ascii_character, c.0, c.1)
    }
}

extern crate volatile;
use self::volatile::Volatile;
struct Buffer {
    chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

pub struct VGADisplay {
    buffer: &'static mut Buffer
}

// TODO: handle this with the object manager
pub fn get_display() -> VGADisplay {
    VGADisplay {
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    }
}

impl ASCIITextDisplay for VGADisplay {
    fn dimensions(&self) -> (usize, usize) {
        (BUFFER_WIDTH, BUFFER_HEIGHT)
    }
    fn get(&self, x: usize, y: usize) -> Option<(u8, Color, Color)> {
        if x >= BUFFER_WIDTH || y >= BUFFER_HEIGHT {
            None
        } else {
            Some(self.buffer.chars[y][x].read().recover())
        }
    }


    fn set(&mut self, ascii_character: u8, fg: Color, bg: Color, x: usize, y: usize) {
        if x >= BUFFER_WIDTH || y >= BUFFER_HEIGHT {
            return
        } 
        
        self.buffer.chars[y][x].write(ScreenChar {
            ascii_character: ascii_character,
            color_code: ColorCode::new(fg, bg)
        });
    }

    fn copy(&mut self, source_x: usize, source_y: usize, 
            destination_x: usize, destination_y: usize) {
        if source_x >= BUFFER_WIDTH || source_y >= BUFFER_HEIGHT ||
           destination_x >= BUFFER_WIDTH || destination_y >= BUFFER_HEIGHT {
           return;
        }
       
        let c = self.buffer.chars[source_y][source_x].read();
        self.buffer.chars[destination_y][destination_x].write(c);
    }
}
