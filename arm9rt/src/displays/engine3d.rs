pub mod e3d{
    #[repr(u8)]
    pub enum GXFIFO{
        NOP = 0x00,
        MTX_MODE = 0x10,
        MTX_PUSH = 0x11,
        MTX_POP = 0x12,
        MTX_STORE = 0x13,
        MTX_RESTORE = 0x14,
        MTX_IDENTITY = 0x15,
        MTX_LOAD_4X4 = 0x16,
        MTX_LOAD_4X3 = 0x17,
        MTX_MULT_4X4 = 0x18,
        MTX_MULT_4X3 = 0x19,
        MTX_MULT_3X3 = 0x1A,
        MTX_SCALE = 0x1B,
        MTX_TRANSFORM = 0x1C,
        COLOR = 0x20,
        NORMAL = 0x21,
        TEXCOORD = 0x22,
        VTX_16 = 0x23,
        VTX_10 = 0x24,
        VTX_XY = 0x25,
        VTX_XZ = 0x26,
        VTX_YZ = 0x27,
        VTX_DIFF = 0x28,
        POLYGON_ATTR = 0x29,
        TEXIMAGE_PARAMS = 0x2A,
        PLTT_BASE = 0x2B,
        DIF_AMB = 0x30,
        SPEC_EMI = 0x31,
        LIGHT_VECTOR = 0x32,
        LIGHT_COLOR = 0x33,
        SHININESS = 0x34,
        BEGIN_VTXS = 0x40,
        END_VTXS = 0x41,
        SWAP_BUFFERS = 0x50,
        VIEWPORT = 0x60,
        BOX_TEST = 0x70,
        POS_TEST = 0x71,
        VEC_TEST = 0x72
    }
    pub const fn fifo_command_pack(cmd0:GXFIFO,cmd1:GXFIFO,cmd2:GXFIFO, cmd3:GXFIFO) -> u32{
        ((cmd3 as u32) << 24) | ((cmd2 as u32) <<16) | ((cmd1 as u32) << 8) | (cmd0 as u32)
    }
}