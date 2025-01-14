use voladdress::Safe;
use super::*;
pub mod vram;
pub use vram::*;
pub mod engine3d;
pub use engine3d::*;
pub mod a;
mod vram_alloc;

pub const DISPCNT: VolAddress<VideoMode, Safe, Safe> = unsafe { VolAddress::new(0x04000000) };
pub const DISPCNT_SUB: VolAddress<VideoMode, Safe, Safe> = unsafe { VolAddress::new(0x04001000) };
const ENABLE_3D:u32 = 1<<3;
const DISPLAY_BG0_ACTIVE:u32 = 1<<8;
#[repr(u32)]
#[derive(Copy, Clone, Debug)]
pub enum VideoMode{
    MODE_0_2D = 0x10000, // /< Video mode 0
    MODE_1_2D = 0x10001, // /< Video mode 1
    MODE_2_2D = 0x10002, // /< Video mode 2
    MODE_3_2D = 0x10003, // /< Video mode 3
    MODE_4_2D = 0x10004, // /< Video mode 4
    MODE_5_2D = 0x10005, // /< Video mode 5
    MODE_6_2D = 0x10006, // /< Video mode 6 (main engine)
    MODE_0_3D = 0x10000 | DISPLAY_BG0_ACTIVE | ENABLE_3D, // /< Video mode 0 with 3D (main engine)
    MODE_1_3D = 0x10001 | DISPLAY_BG0_ACTIVE | ENABLE_3D, // /< Video mode 1 with 3D (main engine)
    MODE_2_3D = 0x10002 | DISPLAY_BG0_ACTIVE | ENABLE_3D, // /< Video mode 2 with 3D (main engine)
    MODE_3_3D = 0x10003 | DISPLAY_BG0_ACTIVE | ENABLE_3D, // /< Video mode 3 with 3D (main engine)
    MODE_4_3D = 0x10004 | DISPLAY_BG0_ACTIVE | ENABLE_3D, // /< Video mode 4 with 3D (main engine)
    MODE_5_3D = 0x10005 | DISPLAY_BG0_ACTIVE | ENABLE_3D, // /< Video mode 5 with 3D (main engine)
    MODE_6_3D = 0x10006 | DISPLAY_BG0_ACTIVE | ENABLE_3D, // /< Video mode 6 with 3D (main engine)

    MODE_FIFO = 3 << 16,    // /< Video display from main memory

    MODE_FB0  = 0x00020000, // /< Video display directly from VRAM_A in LCD mode
    MODE_FB1  = 0x00060000, // /< Video display directly from VRAM_B in LCD mode
    MODE_FB2  = 0x000A0000, // /< Video display directly from VRAM_C in LCD mode
    MODE_FB3  = 0x000E0000, // /< Video display directly from VRAM_D in LCD mode
}
pub fn video_set_mode(v:VideoMode){
    DISPCNT.write(v);
}
pub use a::*;