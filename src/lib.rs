#![feature(panic_implementation)]
#![no_std]

#[macro_use]
extern crate lazy_static;
extern crate spin;

use core::panic::PanicInfo;

#[macro_use]
mod kprint;

mod hal;

#[panic_handler]
#[no_mangle]
pub fn panic(_info: &PanicInfo) -> ! {
    kprintln!("{}", _info);
    loop {}
}

#[no_mangle]
pub extern "C" fn kmain() -> ! {
    kprintln!("test");
    panic!("reached end");
}
