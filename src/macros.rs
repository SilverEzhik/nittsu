#[macro_export]
macro_rules! do_not_interrupt {
    ($($arg:tt)*) => ($crate::hal::do_not_interrupt(|| {$($arg)*}));
}

/// Like the `print!` macro in the standard library, but prints to the VGA text buffer.
#[macro_export]
macro_rules! kprint {
    ($($arg:tt)*) => ($crate::hal::kernel_print(format_args!($($arg)*)));
}

/// Like the `print!` macro in the standard library, but prints to the VGA text buffer.
#[macro_export]
macro_rules! kprintln {
    () => (kprint!("\n"));
    ($fmt:expr) => (kprint!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (kprint!(concat!($fmt, "\n"), $($arg)*));
}

/// Prints to the host through the serial interface.
macro_rules! serial_print {
    ($($arg:tt)*) => {
        $crate::hal::serial_print(format_args!($($arg)*));
    };
}

/// Prints to the host through the serial interface, appending a newline.
macro_rules! serial_println {
    () => (serial_print!("\n"));
    ($fmt:expr) => (serial_print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (serial_print!(concat!($fmt, "\n"), $($arg)*));
}
