/// # Kernel TTY
///
/// Basic display implementation for the time being.
///
use core::fmt;
use hal::ascii_text_display::*;
use hal::do_not_interrupt;
use spin::Mutex;

mod vga;

lazy_static! {
    static ref KTTY_STATIC: Mutex<KTTY> = Mutex::new(KTTY::get().unwrap());
}

/// Prints the given formatted string to the VGA text buffer through the global `WRITER` instance.
pub fn kernel_print(args: fmt::Arguments) {
    use core::fmt::Write;
    do_not_interrupt! {
        KTTY_STATIC.lock().write_fmt(args).unwrap();
    };
}

struct KTTY {
    width: usize,
    height: usize,

    row: usize,
    column: usize,

    fg: Color,
    bg: Color,

    display: vga::VGADisplay,
}

impl fmt::Write for KTTY {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

impl KTTY {
    #[allow(unused_mut)]
    pub fn get() -> Option<KTTY> {
        let mut display = vga::get_display();
        let dimensions = display.dimensions();
        let mut ktty = KTTY {
            width: dimensions.0,
            height: dimensions.1,
            row: 0,
            column: 0,

            fg: Color::White,
            bg: Color::Black,

            display: display,
        };
        ktty.blank();
        return Some(ktty);
    }

    fn write_byte(&mut self, byte: u8) {
        match byte {
            // TODO: backspace?
            b'\n' => self.new_line(),
            _ => {
                if self.column >= self.width {
                    self.new_line();
                }

                self.display
                    .set(byte, self.fg, self.bg, self.column, self.row);
                self.column += 1;
            }
        }
    }

    pub fn blank(&mut self) {
        for x in 0..self.width {
            for y in 0..self.width {
                self.display.set(0x00, Color::Black, Color::Black, x, y);
            }
        }
        self.row = 0;
        self.column = 0;
    }

    fn new_line(&mut self) {
        self.column = 0;

        // if we haven't hit the last row yet, just move to the next row
        if self.row < self.height - 1 {
            self.row += 1;
        } else {
            // if we did, scroll.
            for row in 1..self.height {
                for col in 0..self.width {
                    self.display.copy(col, row, col, row - 1);
                }
            }

            // clear last row
            for i in 0..self.width {
                self.display.set(b' ', self.fg, self.bg, i, self.height - 1);
            }
        }
    }

    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                // printable ASCII byte or newline
                0x20...0x7e | b'\n' => self.write_byte(byte),
                // not part of printable ASCII range
                _ => self.write_byte(0xfe),
            }
        }
    }
}
