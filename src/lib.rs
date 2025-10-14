#![no_std]

/// Boot Information required for successful boot of the HyperText Markup Operation System.
/// 
/// This struct should change size between 32- and 64-bit architecture.
#[repr(C)]
pub struct HTMOSBootInformation {
    /// Boot mode indicator
    /// 
    /// - **0** - BIOS
    /// - **1** - UEFI
    pub boot_mode: usize,

    /// Address to the Memory Map
    pub memory_map_addr: usize,
    /// Size of the Memory Map
    pub memory_map_size: usize,
    /// Size of a Memory Description
    pub memory_desc_size: usize,

    /// Base address for the Framebuffer
    /// 
    /// To use the VGA buffer, **don't use VGA text** as HTMOS does not support that yet.
    pub framebuffer_addr: usize,
    /// Size of the Framebuffer
    pub framebuffer_size: usize,
    /// Number of pixels on a horizontal line
    pub framebuffer_width: u32,
    /// Number of pixels on a vertical line
    pub framebuffer_height: u32,
    /// Number of pixels per scan line
    pub framebuffer_pitch: u32,
    /// Pixel format of the Framebuffer
    /// 
    /// - **0x0** - RGBA
    /// - **0x1** - BGRA
    /// - **0x2** - Bit Mask
    /// - **0x3** - BLT Only
    pub framebuffer_format: u32,

    /// **BIOS** - Address to raw config tables
    /// 
    /// **UEFI** - Address to the System Table given at the UEFI entry point
    pub more_info: usize,
}