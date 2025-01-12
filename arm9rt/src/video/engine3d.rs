use voladdress::{Safe, VolAddress};
use crate::macros::{bitfield_bool, bitfield_int, const_new};

pub mod e3d{
    use simba::scalar::FixedI13F3;
    use crate::macros::*;
    use crate::power::POWCNT::{POWCNT1, PowCnt1Opts};
    use voladdress::*;
    use crate::video::e3d::GlMatrixModeEnum::GlTexture;
    //use crate::video::old_engine3d::e3d::{gl_load_identity, gl_matrix_mode, gl_matrix_pop, MatrixIdentity, MatrixMode, MatrixPop, SwapBuffers, GXSTAT, MTXIDENTITY, MTXMODE, MTXPOP, SWAPBUFFRES};
    pub const fn FIFO_COMMAND_PACK(c1:u32,c2:u32,c3:u32,c4:u32)->u32{
        (c4<<24) | (c3<<16) | (c2<<8) | c1
    }
    pub const fn RGB15(r:u16,g:u16,b:u16)->u16{
        r | (g << 5) | (b << 10)
    }
    pub const fn VERTEX_PACK(x:i16,y:i16)->i32{
        (x as u32 | ((y as u32)<<16)) as i32
    }
    pub const fn INT_TO_V16(n:i16)->i16{
        n<<12
    }
    #[repr(u32)]
    pub enum Begin{
        Triangles = 0,
        Quads = 1,
        TriStrip = 2,
        QuadStrip = 3
    }
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
    #[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct ViewPort(u32);
    impl ViewPort {
        const_new!();
        bitfield_int!(u32;0..=7:u32,x1,with_x1,set_x1);
        bitfield_int!(u32;8..=15:u32,y1,with_y1,set_y1);
        bitfield_int!(u32;16..=23:u32,x2,with_x2,set_x2);
        bitfield_int!(u32;24..=31:u32,y2,with_y2,set_y2);
    }


    #[derive(Clone, Copy)]
    #[repr(u32)]
    pub enum GlMatrixModeEnum{
        GlProjection = 0,
        GlPosition = 1,
        GlModelView = 2,
        GlTexture = 3,
    }
    #[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct Disp3DCntOpts(u16);

    impl Disp3DCntOpts {
        const_new!();
        bitfield_bool!(u16;0,get_enable_texture_mapping,with_enable_texture_mapping,set_enable_texture_mapping);
        bitfield_bool!(u16;1,get_toon_highlight_shading,with_toon_highlight_shading,set_toon_highlight_shading);
        bitfield_bool!(u16;2,get_enable_alpha_test ,with_enable_alpha_test ,set_enable_alpha_test);
        bitfield_bool!(u16;3,get_enable_alpha_blending,with_enable_alpha_blending,set_enable_alpha_blending);
        bitfield_bool!(u16;4,get_enable_anti_aliasing,with_enable_anti_aliasing,set_enable_anti_aliasing);
        bitfield_bool!(u16;5,get_enable_edge_marking,with_enable_edge_marking,set_enable_edge_marking);
        bitfield_bool!(u16;6,get_fog_color_alpha,with_fog_color_alpha ,set_fog_color_alpha );
        bitfield_bool!(u16;7,get_eenable_fog,with_eenable_fog,set_eenable_fog);
        bitfield_int !(u16;8..=11:u16,get_fog_depth_shift,with_fog_depth_shift,set_fog_depth_shift);
        bitfield_bool!(u16;12,get_undeflow_ack,with_undeflow_ack,set_undeflow_ack);
        bitfield_bool!(u16;13,get_overflow_ack,with_overflow_ack,set_overflow_ack);
        bitfield_bool!(u16;14,get_enable_rear_bmp,with_enable_rear_bmp,set_enable_rear_bmp);
    }
    impl_bitwise_ops!(Disp3DCntOpts);
    #[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct MatrixMode(u32);
    impl MatrixMode{
        const_new!();
        bitfield_enum!(u32;0..=1:GlMatrixModeEnum,get_matrix_mode,with_matrix_mode,set_matrix_mode);
    }
    #[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct ClearColor(u32);
    impl ClearColor {
        const_new!();
        bitfield_int!(u32;0..=4:u8,get_r,with_r,set_r);
        bitfield_int!(u32;5..=9:u8,get_g,with_g,set_g);
        bitfield_int!(u32;10..=14:u8,get_b,with_b,set_b);
        bitfield_bool!(u32;15,get_enable_fog,with_enable_fog,set_enable_fog);
        bitfield_int!(u32;16..=20:u8,get_a,with_a,set_a);
        bitfield_int!(u32;24..=29:u8,get_poly_id,with_poly_id,set_poly_id);
    }
    pub const CLEAR_COLOR :VolAddress<ClearColor,(),Safe> = unsafe { VolAddress::new(0x04000350) };
    pub const CLEAR_DEPTH:VolAddress<FixedI13F3,(),Safe> = unsafe{VolAddress::new(0x04000354)};

    #[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct SwapBuffers(u32);
    impl SwapBuffers{
        const_new!();
        bitfield_bool!(u32;0,get_auto_sort,with_auto_sort,set_auto_sort);
        bitfield_bool!(u32;1,get_depth_buffering,with_depth_buffering,set_depth_buffering);
    }
    #[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct MatrixIdentity(i32);
    impl MatrixIdentity{
        const_new!();
    }

    pub struct GL{

    }
    #[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct GxStatOpts(u32);
    impl GxStatOpts{
        const_new!();
        bitfield_int! (u32;8..=12:u32,get_position_vector_matrix_stack,with_position_vector_matrix_stack,set_position_vector_matrix_stack);
        bitfield_bool!(u32;13,get_projection_matrix_stack,with_projection_matrix_stack,set_projection_matrix_stack);
        bitfield_bool!(u32;14,get_matrix_stack_busy,with_matrix_stack_busy,set_matrix_stack_busy);
        bitfield_bool!(u32;15,get_matrix_stack_error,with_matrix_stack_error,set_matrix_stack_error);
        bitfield_bool!(u32;27,get_geometry_engine_busy,with_geometry_engine_busy,set_geometry_engine_busy);
        bitfield_bool!(u32;29,get_clear_fifo,with_clear_fifo,set_clear_fifo);
    }
    impl_bitwise_ops!(GxStatOpts);
    #[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct MatrixPop(i32);
    impl MatrixPop{
        const_new!();
        bitfield_int !(i32;0..=5:i32,get_matrix_pop,with_matrix_pop,set_matrix_pop);
    }
    #[repr(u32)]
    pub enum TexFormat {
        NoTexture = 0,
        A3I5 = 1,
        Palette4Color = 2,
        Palette16Color = 3,
        Palette256Color = 4,
        Texel4x4Compression = 5,
        A4I3 = 6,
        Direct = 7
    }
    #[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct TexImageParam(u32);
    impl TexImageParam{
        const_new!();
        bitfield_enum!(u32;26..=28:TexFormat,tex_format,with_tex_format,set_tex_format);
    }
    #[repr(u32)]
    pub enum PolygonMode {
        Modulation = 0,
        Decal = 1,
        Toon = 2,
        Shadow = 3
    }
    #[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct PolygonAttr(u32);
    impl PolygonAttr{
        const_new!();
        bitfield_enum!(u32;4..=5:PolygonMode,polygon_mode,with_polygon_mode,set_polygon_mode);
    }
    impl GL{
        const VIEWPORT:VolAddress<ViewPort,(),Safe> = unsafe { VolAddress::new(0x04000580) };
        const DISP3DCNT: VolAddress<Disp3DCntOpts, Safe, Safe> = unsafe { VolAddress::new(0x04000060) };
        const MTXIDENTITY :VolAddress<MatrixIdentity,(),Safe> = unsafe { VolAddress::new(0x04000454) };
        const SWAPBUFFRES :VolAddress<SwapBuffers,(),Safe> = unsafe { VolAddress::new(0x04000540) };
        const CLEAR_COLOR :VolAddress<ClearColor,(),Safe> = unsafe { VolAddress::new(0x04000350) };
        const CLEAR_DEPTH:VolAddress<FixedI13F3,(),Safe> = unsafe{VolAddress::new(0x04000354)};
        const GXSTAT: VolAddress<GxStatOpts, Safe, Safe> = unsafe { VolAddress::new(0x04000600) };
        const MTXPOP :VolAddress<MatrixPop,(),Safe> = unsafe { VolAddress::new(0x04000448) };
        const TEX_IMAGE_PARAM:VolAddress<TexImageParam,(),Safe> = unsafe { VolAddress::new(0x040004A8) };
        const POLYGON_ATTR:VolAddress<PolygonAttr,(),Safe> = unsafe { VolAddress::new(0x040004A4) };
        const MTXMODE :VolAddress<MatrixMode,(),Safe> = unsafe { VolAddress::new(0x04000440) };
        pub fn new(disp3dcnt:Option<Disp3DCntOpts>, powcnt1:Option<PowCnt1Opts>)->GL{
            if let Some(p) = powcnt1{
                POWCNT1.write(p);
            }else{
                let powcnt1_default = PowCnt1Opts::new()
                .with_LCDs(true)
                .with_A(true)
                .with_B(true)
                .with_Render3D(true)
                .with_Geometry3D(true)
                .with_DisplaySwap(true);
                POWCNT1.write(powcnt1_default);
            }
            if let Some(c) = disp3dcnt{
                Self::DISP3DCNT.write(c);
            }else{
                Self::DISP3DCNT.write(Disp3DCntOpts::new());
            }
            while Self::GXSTAT.read().get_geometry_engine_busy(){};
            Self::GXSTAT.write(GxStatOpts::default().with_clear_fifo(true));
            Self::reset_matrix_stack();
            Self::flush(SwapBuffers::new());
            Self::clear_color(0,0,0,31);
            Self::clear_poly_id(0);
            Self::clear_depth(FixedI13F3::from_bits(0x7FFF));
            Self::TEX_IMAGE_PARAM.write(TexImageParam(0));
            Self::POLYGON_ATTR.write(PolygonAttr(0));
            Self::matrix_mode(GlMatrixModeEnum::GlProjection);
            Self::load_identity();
            Self::matrix_mode(GlMatrixModeEnum::GlModelView);
            Self::load_identity();
            Self::matrix_mode(GlTexture);
            Self::load_identity();
            GL{

            }
        }
        pub fn reset_matrix_stack(){
            while Self::GXSTAT.read().get_matrix_stack_busy(){
                let mut stat = Self::GXSTAT.read();
                stat.set_matrix_stack_error(false);
                Self::GXSTAT.write(stat);
            }
            if Self::GXSTAT.read().get_projection_matrix_stack(){
                Self::matrix_mode(GlMatrixModeEnum::GlProjection);
                Self::matrix_pop(1);
            }
            Self::matrix_mode(GlMatrixModeEnum::GlModelView);
            Self::matrix_pop((Self::GXSTAT.read().get_position_vector_matrix_stack() & 0x1F)as i32);
            Self::matrix_mode(GlMatrixModeEnum::GlModelView);
            Self::load_identity();
            Self::matrix_mode(GlMatrixModeEnum::GlProjection);
            Self::load_identity();
            Self::matrix_mode(GlTexture);
            Self::load_identity();
        }
        pub fn view_port(viewport: ViewPort){
            Self::VIEWPORT.write(viewport);
        }
        pub const fn fifo_command_pack(cmd0:GXFIFO,cmd1:GXFIFO,cmd2:GXFIFO, cmd3:GXFIFO) -> u32{
            ((cmd3 as u32) << 24) | ((cmd2 as u32) <<16) | ((cmd1 as u32) << 8) | (cmd0 as u32)
        }
        pub fn matrix_mode(mode: GlMatrixModeEnum){
            let mtx_mode = MatrixMode::new().with_matrix_mode(mode);
            Self::MTXMODE.write(mtx_mode);
        }
        pub fn flush(sb:SwapBuffers){
            Self::SWAPBUFFRES.write(sb);
        }

        pub fn matrix_pop(count:i32){
            let pop_count = MatrixPop::new().with_matrix_pop(count);
            Self::MTXPOP.write(pop_count);
        }
        pub fn load_identity(){
            Self::MTXIDENTITY.write(MatrixIdentity::new());
        }
        pub fn clear_color(r:u8, g:u8, b:u8, a:u8){
            CLEAR_COLOR.write(ClearColor::new().with_r(r).with_g(g).with_b(b).with_a(a));
        }
        pub fn clear_depth(d:FixedI13F3){
            CLEAR_DEPTH.write(d);
        }
        pub fn clear_poly_id(id:u8){
            CLEAR_COLOR.write(ClearColor::new().with_poly_id(id));
        }
    }

}