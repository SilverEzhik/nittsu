use super::gdt;
use super::x86_64::structures::idt::{ExceptionStackFrame, InterruptDescriptorTable};
use super::x86_64::VirtAddr;
use lazy_static::lazy_static;

pub const TIMER_INTERRUPT_ID: u8 = super::pic::PIC_1_OFFSET;

pub fn init() {
    IDT.load();
}

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        unsafe {
            idt.double_fault
                .set_handler_fn(double_fault_handler)
                .set_stack_index(gdt::DOUBLE_FAULT_IST_INDEX);
        }
        idt[usize::from(TIMER_INTERRUPT_ID)].set_handler_fn(timer_interrupt_handler);

        idt
    };
}

pub struct Thread {
    pub stack: [u64; 16384],

    // segmentation
    // pub ss: u16,
    // pub cs: u16,
    // pub ds: u16,
    // pub es: u16,
    // pub fs: u16,

    // execution
    pub rip: u64,
    pub rsp: u64,

    // registers
    pub rax: u64,
    pub rbx: u64,
    pub rcx: u64,
    pub rdx: u64,
    pub rsi: u64,
    pub rdi: u64,
    pub rbp: u64,
    // TODO: r8-r15
}

use core::fmt;
impl fmt::Debug for Thread {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Thread: RIP: {:X}, RSP: {:X}, RAX: {:X}, RBX: {:X}, RCX: {:X}, RDX: {:X}, RSI: {:X}, RDI: {:X}, RBP: {:X}",
            self.rip,
            self.rsp,
            self.rax,
            self.rbx,
            self.rcx,
            self.rdx,
            self.rsi,
            self.rdi,
            self.rbp
        )
    }
}
impl Default for Thread {
    fn default() -> Thread {
        Thread {
            stack: [0; 16384],
            // ss: 0,
            // cs: 0,
            // ds: 0,
            // es: 0,
            // fs: 0,
            rip: 0,
            rsp: 0,
            rax: 0,
            rbx: 0,
            rcx: 0,
            rdx: 0,
            rsi: 0,
            rdi: 0,
            rbp: 0,
        }
    }
}

use spin::Mutex;
lazy_static! {
    static ref THREADS: Mutex<[Thread; 2]> =
        unsafe {
            Mutex::new([//make_thread(a), make_thread(b),
         make_thread(|| {loop { for _ in 1..10000 {} kprint!("c");}}),
         make_thread(|| {loop { for _ in 1..10000 {} kprint!("d");}}),
        // make_thread(|| {loop { for _ in 1..10000 {} kprint!("e");}}),
        // make_thread(|| {loop { for _ in 1..10000 {} kprint!("f");}}),
        // make_thread(|| {loop { for _ in 1..10000 {} kprint!("g");}}),
        // make_thread(|| {loop { for _ in 1..10000 {} kprint!("h");}}),
        // make_thread(|| {loop { for _ in 1..10000 {} kprint!("i");}}),
        // make_thread(|| {loop { for _ in 1..10000 {} kprint!("j");}}),

        ]) };
}

lazy_static! {
    static ref TICK: Mutex<Tick> = Mutex::new(Tick::new());
}

struct Tick {
    t: usize,
}
impl Tick {
    fn new() -> Tick {
        Tick { t: 0 }
    }
    fn get(&self) -> usize {
        self.t
    }
    fn inc(&mut self) {
        self.t += 1;
    }
}

pub unsafe fn make_thread(f: fn()) -> Thread {
    let mut thread: Thread = Default::default();
    thread.rip = f as *const u64 as u64;
    thread.rsp = &thread.stack[thread.stack.len() - 10] as *const u64 as u64;
    thread.rsi = thread.rsp;
    thread.rdi = thread.rsi;

    thread
}

fn a() {
    loop {
        for _ in 1..10000 {}
        kprint!("a");
    }
}
fn b() {
    loop {
        for _ in 1..10000 {}
        kprint!("b");
    }
}

extern "x86-interrupt" fn timer_interrupt_handler(_stack_frame: &mut ExceptionStackFrame) {
    let mut threads = THREADS.lock();
    let mut tick = TICK.lock();
    tick.inc();
    let tick = tick.get();
    unsafe {
        let current = tick % threads.len();
        let new = (tick + 1) % threads.len();

        if tick > 1 {
            let mut current_thread = &mut threads[current];
            // record interrupted thread's state
            // current_thread.rip = _stack_frame.instruction_pointer.as_u64();
            current_thread.rsp = _stack_frame.stack_pointer.as_u64();

            asm!("mov %rax, $0" : "=r" (current_thread.rax));
            asm!("mov %rbx, $0" : "=r" (current_thread.rbx));
            asm!("mov %rcx, $0" : "=r" (current_thread.rcx));
            asm!("mov %rdx, $0" : "=r" (current_thread.rdx));
            asm!("mov %rsi, $0" : "=r" (current_thread.rsi));
            asm!("mov %rdi, $0" : "=r" (current_thread.rdi));
            asm!("mov %rbp, $0" : "=r" (current_thread.rbp));
            // kprint!("!!{} {:?}!!", TICK, current_thread);
        }

        let new_thread = &mut threads[new];
        // load new state
        _stack_frame.instruction_pointer = VirtAddr::from_ptr(new_thread.rip as *const ());
        _stack_frame.stack_pointer = VirtAddr::from_ptr(new_thread.rsp as *const ());

        asm!("movq $0, %rax " :: "r" (new_thread.rax) : "memory" : "volatile");
        asm!("movq $0, %rbx " :: "r" (new_thread.rbx) : "memory" : "volatile");
        asm!("movq $0, %rcx " :: "r" (new_thread.rcx) : "memory" : "volatile");
        asm!("movq $0, %rdx " :: "r" (new_thread.rdx) : "memory" : "volatile");
        asm!("movq $0, %rsi " :: "r" (new_thread.rsi) : "memory" : "volatile");
        asm!("movq $0, %rdi " :: "r" (new_thread.rdi) : "memory" : "volatile");
        asm!("movq $0, %rbp " :: "r" (new_thread.rbp) : "memory" : "volatile");
    }

    unsafe {
        super::pic::notify_end_of_interrupt(TIMER_INTERRUPT_ID);
    };
}

extern "x86-interrupt" fn breakpoint_handler(stack_frame: &mut ExceptionStackFrame) {
    kprintln!("Breakpoint exception:\n{:#?}", stack_frame);
}
extern "x86-interrupt" fn double_fault_handler(
    stack_frame: &mut ExceptionStackFrame,
    _error_code: u64,
) {
    kprintln!("Double fault exception:\n{:#?}", stack_frame);
    loop {}
}
