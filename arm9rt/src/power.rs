pub mod POWCNT{
    use voladdress::*;
    use crate::macros::*;

    #[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct PowCnt1Opts(u16);
    
    impl PowCnt1Opts {
        const_new!();
        bitfield_bool!(u16;0,get_LCDs,with_LCDs,set_LCDs);
        bitfield_bool!(u16;1,get_A,with_A,set_A);
        bitfield_bool!(u16;2,get_Render3D,with_Render3D,set_Render3D);
        bitfield_bool!(u16;3,get_Geometry3D,with_Geometry3D,set_Geometry3D);
        bitfield_bool!(u16;4,get_B,with_B,set_B);
        bitfield_bool!(u16;15,get_DisplaySwap,with_DisplaySwap,set_DisplaySwap);
    }
    pub const POWCNT1: VolAddress<PowCnt1Opts, Safe, Safe> = unsafe { VolAddress::new(0x04000304) };
    impl_bitwise_ops!(PowCnt1Opts);
}
