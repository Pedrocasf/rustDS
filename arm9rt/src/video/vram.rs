use voladdress::{VolAddress, VolBlock, Safe};
use crate::macros::*;
use crate::video::e3d::Disp3DCntOpts;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct VramCnt(u8);
impl VramCnt {
    pub_const_fn_new_zeroed!();
    u8_bool_field!(7,get_vram_enable,with_vram_enable);
    u8_int_field!(0 - 2,get_vram_mst,with_vram_mst);
    u8_int_field!(3 - 4,get_vram_offset,with_vram_offset);
}
pub const VRAMCNT_A: VolAddress<VramCnt, Safe, Safe> = unsafe { VolAddress::new(0x4000240) };
pub const VRAMCNT_B: VolAddress<VramCnt, Safe, Safe> = unsafe { VolAddress::new(0x4000241) };
pub const VRAMCNT_C: VolAddress<VramCnt, Safe, Safe> = unsafe { VolAddress::new(0x4000242) };
pub const VRAMCNT_D: VolAddress<VramCnt, Safe, Safe> = unsafe { VolAddress::new(0x4000243) };
pub const VRAMCNT_E: VolAddress<VramCnt, Safe, Safe> = unsafe { VolAddress::new(0x4000244) };
pub const VRAMCNT_F: VolAddress<VramCnt, Safe, Safe> = unsafe { VolAddress::new(0x4000245) };
pub const VRAMCNT_G: VolAddress<VramCnt, Safe, Safe> = unsafe { VolAddress::new(0x4000246) };
pub const VRAMCNT_H: VolAddress<VramCnt, Safe, Safe> = unsafe { VolAddress::new(0x4000248) };
pub const VRAMCNT_I: VolAddress<VramCnt, Safe, Safe> = unsafe { VolAddress::new(0x4000249) };

pub const BG_PALLETES: VolBlock<u16, Safe, Safe, 0x100> = unsafe { VolBlock::new(0x05000000) };
pub const FG_PALLETES: VolBlock<u16, Safe, Safe, 0x100> = unsafe { VolBlock::new(0x05000200) };
pub const BG_PALLETES_B: VolBlock<u16, Safe, Safe, 0x100> = unsafe { VolBlock::new(0x05000400) };
pub const ENGINE_A_BG0: VolBlock<u16, Safe, Safe, 0x1000> = unsafe { VolBlock::new(0x06002000) };
pub const ENGINE_A_BG1: VolBlock<u16, Safe, Safe, 0x1000> = unsafe { VolBlock::new(0x06004000) };
pub const ENGINE_A_BG2: VolBlock<u16, Safe, Safe, 0x1000> = unsafe { VolBlock::new(0x06006000) };
pub const ENGINE_A_BG3: VolBlock<u16, Safe, Safe, 0x1000> = unsafe { VolBlock::new(0x06008000) };
pub const ENGINE_A_TILESET: VolBlock<u32, Safe, Safe,0x20000> = unsafe { VolBlock::new(0x06000000) };
pub const ENGINE_B_TILESET: VolBlock<u32, Safe, Safe,0x20000> = unsafe { VolBlock::new(0x06200000) };
pub const ENGINE_B_BG0: VolBlock<u16, Safe, Safe, 0x1000> = unsafe { VolBlock::new(0x06202000) };
pub const ENGINE_A_OBJ: VolBlock<u8, Safe, Safe, 0x18000> = unsafe { VolBlock::new(0x06418000) };
pub const OBJ_ATTR: VolBlock<u8, Safe, Safe, 0x800> = unsafe { VolBlock::new(0x07000000) };
