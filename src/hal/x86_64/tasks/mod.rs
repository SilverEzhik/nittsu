pub unsafe fn stack_jmp(stack: *mut (), ip: *const ()) -> ! {
    asm!("mov rsp, $0; jmp $1" 
         :
         : "rg"(stack), "r"(ip) 
         :
         : "volatile", "intel");
    loop {}
}
