#![feature(panic_implementation)]
#![no_std]
#![no_main]

extern crate bootloader_precompiled;

#[macro_use]
extern crate lazy_static;

extern crate spin;

use core::panic::PanicInfo;

mod hal;

#[panic_handler]
#[no_mangle]
pub fn panic(_info: &PanicInfo) -> ! {
    loop {}
}


static HELLO: &[u8] = b"Here comes a new adventure.";

#[no_mangle]
pub extern "C" fn _start() -> ! {
    use hal::ascii_text_display::ASCIITextDisplay;

    let mut display = hal::vga::VGA_DISPLAY.lock();
    for (i, &byte) in HELLO.iter().enumerate() {
        display.set(byte, 
                hal::ascii_text_display::Color::White, 
                hal::ascii_text_display::Color::Black,
                i, 0);
    }
    // try copy
    for (i, &byte) in HELLO.iter().enumerate() {
        display.copy(i, 0, i, 1);
    }
    // try get
    for (i, &byte) in HELLO.iter().enumerate() {
        let c = display.get(i, 0).unwrap();
        display.set(c.0, 
                    hal::ascii_text_display::Color::Cyan,
                    c.2,
                    i, 2);
    }
    loop {}
}


