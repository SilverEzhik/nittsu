use super::KTTY;
use spin::Mutex;
use core::fmt;

lazy_static! {
    static ref KTTY_STATIC: Mutex<KTTY> = Mutex::new(KTTY::get().unwrap());
}

/// Like the `print!` macro in the standard library, but prints to the VGA text buffer.
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::ktty::hardcoded::print(format_args!($($arg)*)));
}

/// Like the `print!` macro in the standard library, but prints to the VGA text buffer.
#[macro_export]
macro_rules! println {
    () => (print!("\n"));
    ($fmt:expr) => (print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (print!(concat!($fmt, "\n"), $($arg)*));
}

/// Prints the given formatted string to the VGA text buffer through the global `WRITER` instance.
pub fn print(args: fmt::Arguments) {
    use core::fmt::Write;
    KTTY_STATIC.lock().write_fmt(args).unwrap();
}
