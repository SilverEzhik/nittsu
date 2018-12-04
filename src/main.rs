#![feature(abi_x86_interrupt)]
#![feature(asm)]
#![no_std]
#![no_main]

#[macro_use]
extern crate lazy_static;
extern crate multiboot2;
extern crate spin;

use core::panic::PanicInfo;

#[macro_use]
mod macros;

#[macro_use]
mod hal;

#[panic_handler]
#[no_mangle]
pub fn panic(_info: &PanicInfo) -> ! {
    kprintln!("{}", _info);
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    hal::init();
    kprintln!("Rust world");

    loop {
        for _ in 0..10000 {}
        kprint!("-");
    }
}
