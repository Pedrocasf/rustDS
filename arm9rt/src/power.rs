pub mod POWCNT{
    use voladdress::*;
    use crate::macros::*;

    #[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct PowCnt1Opts(u16);
    
    impl PowCnt1Opts {
        pub_const_fn_new_zeroed!();
        u16_bool_field!(0,get_LCDs,with_LCDs);
        u16_bool_field!(1,get_A,with_A);
        u16_bool_field!(2,get_Render3D,with_Render3D);
        u16_bool_field!(3,get_Geometry3D,with_Geometry3D);
        u16_bool_field!(4,get_B,with_B);
        u16_bool_field!(15,get_DisplaySwap,with_DisplaySwap);
    }
    pub const POWCNT1: VolAddress<PowCnt1Opts, Safe, Safe> = unsafe { VolAddress::new(0x04000304) };
}
