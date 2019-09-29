#[repr(C)]
struct VramControl{
    p: &'static mut VramControlRegs
}

#[repr(C)]
struct VramControlRegs{
    VRAMCNT_A:Volatile<u8>,
    VRAMCNT_B:Volatile<u8>,
    VRAMCNT_C:Volatile<u8>,
    VRAMCNT_D:Volatile<u8>,
    VRAMCNT_E:Volatile<u8>,
    VRAMCNT_F:Volatile<u8>,
    VRAMCNT_G:Volatile<u8>,
    WRAMCNT:Volatile<u8>,
    VRAMCNT_H:Volatile<u8>,
    VRAMCNT_I:Volatile<u8>,
}
impl VramControl{
    pub fn new() -> VramControl{
        VramControl{
            p: unsafe{ &mut *(0x4000240 as *mut VramControlRegs)}
        }
    }

}

pub struct VRAM{
    standard_palletes: &'static mut [u8:0x800],
    engine_a_background: &'static mut [u8:0x80000],
    engine_b_background: &'static mut [u8:0x20000],
    engine_a_object: &'static mut [u8:0x40000],
    engine_b_object: &'static mut [u8:0x20000],
    lcdc: &'static mut [u8:0xA4000],
    object_attribute: &'static mut [u8:0x800],
    vram_control:VramControl
}
impl VRAM{
    pub fn new() -> VRAM{
        VRAM{
            standard_palletes: unsafe{ &mut *(0x05000000 as *mut [u8:0x800])},
            engine_a_background: unsafe{ &mut *(0x06000000 as *mut [u8:0x80000])},
            engine_b_background: unsafe{ &mut *(0x06200000 as *mut [u8:0x20000])},
            engine_a_object: unsafe{ &mut *(0x06400000 as *mut [u8:0x40000])},
            engine_b_object: unsafe{ &mut *(0x06600000 as *mut [u8:0x20000])},
            lcdc: unsafe{ &mut *(0x06800000 as *mut [u8:0xA4000])},
            object_attribute: unsafe{ &mut *(0x07000000 as *mut [u8:0x800])},
            vram_control:VramControl::new(),
        }
    }
}