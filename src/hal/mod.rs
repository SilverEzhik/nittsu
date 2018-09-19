/// # Hardware Abstraction Layer
///
/// This is the part of the OS where differences between various platforms are
/// dealt with. 
///
/// The idea is to try and contain all code that might not be cross-platform in
/// this module. Of course, in practice, things do get more complicated as things
/// like optimization and tightly integrated hardware support come into play.
/// However, for the scope of this small OS, it should be possible to contain 
/// everything in here.
///
/// So far, only x86_64 support is in the works, but an attempt will be made to
/// use traits and such for handling abstractions for working with devices, 
/// memory, switching to kernel mode, and etc, so that an eventual port to something
/// like ARM could happen.

#[cfg(target_arch = "x86_64")]
mod x86_64;
#[cfg(target_arch = "x86_64")]
pub use self::x86_64::*;

pub mod ascii_text_display;
