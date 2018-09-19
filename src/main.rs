#![feature(panic_implementation)]
#![no_std]
#![no_main]

extern crate bootloader_precompiled;

#[macro_use]
extern crate lazy_static;

extern crate spin;

use core::panic::PanicInfo;

mod hal;
#[macro_use]
mod ktty;

#[panic_handler]
#[no_mangle]
pub fn panic(_info: &PanicInfo) -> ! {
    println!("{}", _info);
    loop {}
}

use core::fmt::Write;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    panic!("test");
    loop {}
}
