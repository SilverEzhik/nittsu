use super::gdt;
use super::x86_64;

pub mod idt;
pub mod pic;

pub fn init() {
    idt::init();
    pic::init();
}

pub fn do_not_interrupt<F, R>(f: F) -> R
where
    F: FnOnce() -> R,
{
    use super::x86_64::instructions::interrupts::without_interrupts;
    without_interrupts(f)
}
