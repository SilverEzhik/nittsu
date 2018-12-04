extern crate x86;
extern crate x86_64;

#[macro_use]
use kprint;

mod ktty;
pub use self::ktty::kernel_print;

mod tasks;

mod devices;
mod gdt;
mod interrupts;
pub use self::interrupts::do_not_interrupt;

fn test_evil() {
    kprintln!("awoo");
    loop {}
}

pub fn init() {
    gdt::init();
    interrupts::init();

    x86_64::instructions::interrupts::enable();

    // let mut stack: [u64; 128] = [0; 128];
    // unsafe {
    //     tasks::stack_jmp(stack[127] as *mut (), kmain as *const ());
    // }
}
