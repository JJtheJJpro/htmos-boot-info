//! # Boot Information for the HyperText Markup Operating System
//! 
//! This also includes the entry calling convention and type for the kernel.

#![no_std]

#[cfg(target_arch = "x86")]
pub type HTMOSEntry = extern "cdecl" fn(*const HTMOSBootInformation) -> !;
#[cfg(target_arch = "x86_64")]
pub type HTMOSEntry = extern "sysv64" fn(*const HTMOSBootInformation) -> !;
// As of right now, I haven't expanded at all on the other archs.  Defaulting to C.
#[cfg(not(any(target_arch = "x86_64", target_arch = "x86")))]
pub type HTMOSEntry = extern "C" fn(*const HTMOSBootInformation) -> !;

/// Boot Information required for successful boot of the HyperText Markup Operation System.
///
/// This struct should change size between 32- and 64-bit architecture.
#[repr(C)]
pub struct HTMOSBootInformation {
    /// "HTMLBOOT" - 0x48544D4C424F4F54
    /// 
    /// HTMOS is forced to panic at the very beginning if this value is not presented correctly.
    pub magic: u64,

    /// Boot mode indicator
    ///
    /// - **0** - BIOS
    /// - **1** - UEFI
    pub boot_mode: usize,

    /// Address to the Memory Map.
    /// - **BIOS** - HTMOS only has support for the E820 Memory Map entries.
    /// - **UEFI** - The memory map will be given to you close to exiting boot services.
    pub memory_map_addr: usize,
    /// Size of the Memory Map
    pub memory_map_size: usize,
    /// Size of a Memory Description
    pub memory_desc_size: usize,

    /// Base address for the Framebuffer
    ///
    /// HTMOS does not have support for VGA text (0xB8000) or the VGA buffer (0xA0000).
    //To use the VGA buffer, **don't use VGA text** as HTMOS does not support that yet.
    pub framebuffer_addr: usize,
    /// Size of the Framebuffer (can be 0 for now, I will change it in the future to require a number > 0)
    pub framebuffer_size: usize,
    /// Number of pixels on a horizontal line
    pub framebuffer_width: u32,
    /// Number of pixels on a vertical line
    pub framebuffer_height: u32,
    /// Number of pixels per scan line
    pub framebuffer_pitch: u32,
    /// Pixel format (UEFI) or Bits Per Pixel (BIOS) of the Framebuffer
    ///
    /// - **BIOS** - This is bbp (**BITS** per pixel); it should come straight from the information given when converting to the framebuffer.
    /// - **UEFI** - As of right now, I'm only aware of UEFI values 0x0 and 0x1.
    pub framebuffer_format: u32,

    /// **BIOS** - Address to raw config tables (if 0, HTMOS will find it manually)
    ///
    /// **UEFI** - Address to the System Table given at the UEFI entry point
    pub more_info: usize,
}
