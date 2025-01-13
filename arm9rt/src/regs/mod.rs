use voladdress::*;
use fixed::types::I8F8;
use crate::macros::*;
pub const IME: VolAddress<u32, Safe, Safe> = unsafe { VolAddress::new(0x04000208) };
pub const IE: VolAddress<u32, Safe, Safe> = unsafe { VolAddress::new(0x04000210) };
pub const IF: VolAddress<u32, Safe, Safe> = unsafe { VolAddress::new(0x04000214) };
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct DispCnt(u32);
impl DispCnt{
    pub_const_fn_new_zeroed!();
    u32_int_field!(16 - 17,get_display_mode,with_display_mode);
    u32_int_field!(0 - 2,get_bg_mode,with_bg_mode);
    u32_bool_field!(8,get_BG0_en,with_BG0_en);
    u32_bool_field!(3,get_BG0_3D,with_BG0_3D);
}
pub const DISPCNT: VolAddress<DispCnt, Safe, Safe> = unsafe { VolAddress::new(0x04000000) };
pub const DISPCNT_B: VolAddress<u32, Safe, Safe> = unsafe { VolAddress::new(0x04001000) };
pub const DISPSTAT: VolAddress<u32, Safe, Safe> = unsafe { VolAddress::new(0x04000004) };
pub const IPCFIFORECV: VolAddress<u32, Safe, ()> = unsafe { VolAddress::new(0x04100000) };
pub const BG0CNT: VolAddress<u16, Safe, Safe> = unsafe { VolAddress::new(0x04000008) };
pub const BG0CNT_B: VolAddress<u16, Safe, Safe> = unsafe { VolAddress::new(0x04001008) };
pub const BG1CNT: VolAddress<u16, Safe, Safe> = unsafe { VolAddress::new(0x0400000A) };
pub const BG2CNT: VolAddress<u16, Safe, Safe> = unsafe { VolAddress::new(0x0400000C) };
pub const BG3CNT: VolAddress<u16, Safe, Safe> = unsafe { VolAddress::new(0x0400000E) };
pub const IPCFIFOCNT: VolAddress<u32, Safe, Safe> = unsafe { VolAddress::new(0x04000184) };
pub const IPCSYNC: VolAddress<u32, Safe, Safe> = unsafe { VolAddress::new(0x04000180) };
pub const WRAMCNT: VolAddress<u8, Safe, Safe> = unsafe { VolAddress::new(0x4000247) };
pub const BG3PA: VolAddress<I8F8, Safe, Safe> = unsafe { VolAddress::new(0x4000030) };
pub const BG3PB: VolAddress<I8F8, Safe, Safe> = unsafe { VolAddress::new(0x4000032) };
pub const BG3PC: VolAddress<I8F8, Safe, Safe> = unsafe { VolAddress::new(0x4000034) };
pub const BG3PD: VolAddress<I8F8, Safe, Safe> = unsafe { VolAddress::new(0x4000036) };
pub const BG2PA: VolAddress<I8F8, Safe, Safe> = unsafe { VolAddress::new(0x4000020) };
pub const BG2PB: VolAddress<I8F8, Safe, Safe> = unsafe { VolAddress::new(0x4000022) };
pub const BG2PC: VolAddress<I8F8, Safe, Safe> = unsafe { VolAddress::new(0x4000024) };
pub const BG2PD: VolAddress<I8F8, Safe, Safe> = unsafe { VolAddress::new(0x4000026) };
pub const KEYINPUT: VolAddress<u16, Safe, ()> = unsafe { VolAddress::new(0x400_0130) };