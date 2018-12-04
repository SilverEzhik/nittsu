/// Protection ring levels
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(i8)]
pub enum PrivilegeLevel {
    /// Ring 0 - used by the kernel
    Kernel = 0,
    /// Ring 3 - user mode
    User = 3,
    /// Ring "-1" - used by hypervisor software
    Hypervisor = -1,
}
