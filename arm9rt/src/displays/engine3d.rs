pub mod e3d {
    use crate::macros::*;
    use alloc::boxed::Box;
    use core::{borrow::BorrowMut, u32, usize};
    use crate::dynamic_array::DynamicArray;
    use crate::power::POWCNT::{POWCNT1, PowCnt1Opts};
    use voladdress::*;
    use crate::*;
    use crate::buddy_alloc;
    use core::cell::RefCell;
    use fixed::types::I13F3;
    #[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct ViewPort(u32);
    impl ViewPort{
        const_new!();
        bitfield_int!(u32;0..=7:u32,x1,with_x1,set_x1);
        bitfield_int!(u32;8..=15:u32,y1,with_y1,set_y1);
        bitfield_int!(u32;16..=23:u32,x2,with_x2,set_x2);
        bitfield_int!(u32;24..=31:u32,y2,with_y2,set_y2);
    }
    pub const VIEWPORT:VolAddress<ViewPort,(),Safe> = unsafe { VolAddress::new(0x04000580) };
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
    pub const POLYGON_ATTR:VolAddress<PolygonAttr,(),Safe> = unsafe { VolAddress::new(0x040004A4) };
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
    pub const TEX_IMAGE_PARAM:VolAddress<TexImageParam,(),Safe> = unsafe { VolAddress::new(0x040004A8) };
    
    pub const CLEAR_DEPTH:VolAddress<I13F3,(),Safe> = unsafe{VolAddress::new(0x04000354)};
    #[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct ClearColor(u32);
    impl ClearColor{
        const_new!();
        bitfield_int!(u32;0..=4:u8,get_r,with_r,set_r);
        bitfield_int!(u32;5..=9:u8,get_g,with_g,set_g);
        bitfield_int!(u32;10..=14:u8,get_b,with_b,set_b);
        bitfield_bool!(u32;15,get_enable_fog,with_enable_fog,set_enable_fog);
        bitfield_int!(u32;16..=20:u8,get_a,with_a,set_a);
        bitfield_int!(u32;24..=29:u8,get_poly_id,with_poly_id,set_poly_id);
    }
    pub const CLEAR_COLOR :VolAddress<ClearColor,(),Safe> = unsafe { VolAddress::new(0x04000350) };

    #[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct SwapBuffers(u32);
    impl SwapBuffers{
        const_new!();
        bitfield_bool!(u32;0,get_auto_sort,with_auto_sort,set_auto_sort);
        bitfield_bool!(u32;1,get_depth_buffering,with_depth_buffering,set_depth_buffering);
    }
    pub const SWAPBUFFRES :VolAddress<SwapBuffers,(),Safe> = unsafe { VolAddress::new(0x04000540) };
    #[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct MatrixIdentity(i32);
    impl MatrixIdentity{
        const_new!();
    }
    pub const MTXIDENTITY :VolAddress<MatrixIdentity,(),Safe> = unsafe { VolAddress::new(0x04000454) };
    #[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct MatrixPop(i32);
    impl MatrixPop{
        const_new!();
        bitfield_int !(i32;0..=5:i32,get_matrix_pop,with_matrix_pop,set_matrix_pop);
    }
    pub const MTXPOP :VolAddress<MatrixPop,(),Safe> = unsafe { VolAddress::new(0x04000448) };
    #[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct MatrixMode(u32);


    impl MatrixMode{
        const_new!();
        bitfield_enum!(u32;0..=1:GlMatrixModeEnum,get_matrix_mode,with_matrix_mode,set_matrix_mode);
    }
    pub const MTXMODE :VolAddress<MatrixMode,(),Safe> = unsafe { VolAddress::new(0x04000440) };
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
    pub const GXSTAT: VolAddress<GxStatOpts, Safe, Safe> = unsafe { VolAddress::new(0x04000600) };
    impl_bitwise_ops!(GxStatOpts);
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
    pub const DISP3DCNT: VolAddress<Disp3DCntOpts, Safe, Safe> = unsafe { VolAddress::new(0x04000060) };
    impl_bitwise_ops!(Disp3DCntOpts);
    /// The screen's width in this mode.
    pub const WIDTH: usize = 256;

    /// The screen's height in this mode.
    pub const HEIGHT: usize = 192;

    const GL_GLOBALS:RefCell<GlGlobals> = RefCell::new(GlGlobals::new());
    #[derive(Clone, Default)]
    struct GlGlobals{
        pub matrix_mode:Option<GlMatrixModeEnum>,
        pub vram_blocks:[Option<Box<VramBlock>>;2],
        pub vram_lock:[Option<bool>;2],
        pub texture_ptrs:Option<DynamicArray<u8>>,
        pub palette_ptrs:Option<DynamicArray<u8>>,
        pub dealloc_tex:Option<DynamicArray<u8>>,
        pub dealloc_pal:Option<DynamicArray<u8>>,
        pub dealloc_tex_size:usize,
        pub dealloc_pal_size:usize,
        pub active_texture:usize,
        pub active_palette:usize,
        pub tex_count:usize,
        pub pal_count:usize,
        pub clear_color:ClearColor,
    }
    impl GlGlobals{
        pub const fn new() -> GlGlobals{
            GlGlobals{
                matrix_mode:None,
                vram_blocks:[None,None],
                vram_lock:[None;2],
                texture_ptrs:None,
                palette_ptrs:None,
                dealloc_tex:None,
                dealloc_pal:None,
                dealloc_tex_size:0,
                dealloc_pal_size:0,
                active_texture:0,
                active_palette:0,
                tex_count:0,
                pal_count:0,
                clear_color:ClearColor::new(),
            }
        }
    }
    #[derive(Clone, Copy)]
    #[repr(u32)]
    pub enum GlMatrixModeEnum{
        GlProjection = 0,
        GlPosition = 1,
        GlModelView = 2,
        GlTexture = 3,
    }
    #[derive(Clone)]
    struct VramBlock{
        pub start_addr:usize,
        pub end_addr:usize,
        pub first_block:Option<Box<SingleBlock>>,
        pub first_empty:Option<Box<SingleBlock>>,
        pub first_alloc:Option<Box<SingleBlock>>,
        pub last_examined:Option<Box<SingleBlock>>,
        pub last_examined_addr:Option<*mut u8>,
        pub last_examined_size:u32,
        pub block_ptrs:Option<DynamicArray<u8>>,
        pub dealloc_blocks:Option<DynamicArray<u8>>,
        pub block_count:usize,
        pub dealloc_count:usize
    }
    impl VramBlock{
        pub fn construct(start:usize,end:usize)->Box<VramBlock>{
            let mut mb = Box::new(VramBlock{
                start_addr:start,
                end_addr:end,
                first_block:None,
                first_empty:None,
                first_alloc:None,
                last_examined:None,
                last_examined_addr:None,
                last_examined_size:0,
                block_ptrs:None,
                dealloc_blocks:None,
                block_count:0,
                dealloc_count:0

            });
            if start > end{
                core::mem::swap(&mut mb.start_addr, &mut mb.end_addr);
            };
            VramBlock::init(&mut mb);
            return mb;
        }
        pub fn init(mb:&mut Box<VramBlock>){
            let mut new_block = SingleBlock::new(mb.start_addr, mb.end_addr - mb.start_addr);
            new_block.block_size = mb.end_addr - mb.start_addr;
            mb.first_empty = Some(new_block.clone());
            mb.first_block = Some(new_block.clone());
            mb.block_count = 1;
            mb.block_ptrs = Some(DynamicArray::new(16));
            mb.dealloc_blocks = Some(DynamicArray::new(16));
            if let Some(blocks) = &mut mb.block_ptrs{
                for i in 0..16{
                    blocks[i] = 0;
                }
            }
            if let Some(blocks) = &mut mb.dealloc_blocks{
                for i in 0..16{
                    blocks[i] = 0;
                }
            }
        }
    }
    #[derive(Clone)]
    struct SingleBlock{
        pub index_out:u32,
        pub addr_set:usize,
        pub node:[Box<Option<SingleBlock>>;4],
        pub block_size:usize,
    }
    impl SingleBlock{
        pub fn new(addr:usize, sz:usize)-> Box<SingleBlock>{
            Box::new(SingleBlock{
                index_out:0,
                addr_set:addr,
                node:[Box::new(None).clone(),Box::new(None).clone(),Box::new(None).clone(),Box::new(None).clone()],
                block_size:sz
            }.clone())
        }
    }
    pub fn gl_init(disp3dcnt:Option<Disp3DCntOpts>, powcnt1:Option<PowCnt1Opts>){
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
        if let Ok(mut glob) = GL_GLOBALS.try_borrow_mut(){

            glob.vram_blocks[0] = Some(VramBlock::construct(0x6800000, 0x6880000));              
            glob.vram_blocks[1] = Some(VramBlock::construct(0x6880000, 0x68A4000));
            glob.vram_lock[0] = Some(false);
            glob.vram_lock[1] = Some(false);
            glob.clear_color = ClearColor::new();
            glob.active_texture = 0;
            glob.active_palette = 0;
            glob.pal_count = 1;
            glob.tex_count = 1;
            glob.dealloc_tex_size = 0;
            glob.dealloc_pal_size = 0;
            glob.texture_ptrs = Some(DynamicArray::new(16));
            glob.palette_ptrs = Some(DynamicArray::new(16));
            glob.dealloc_tex = Some(DynamicArray::new(16));
            glob.dealloc_pal = Some(DynamicArray::new(16));
            for i in 0..16{
                glob.texture_ptrs.as_mut().unwrap()[i] = 0;
                glob.palette_ptrs.as_mut().unwrap()[i] = 0;
                glob.dealloc_tex.as_mut().unwrap()[i] = 0;
                glob.dealloc_pal.as_mut().unwrap()[i] = 0;
            }
        }
        while GXSTAT.read().get_geometry_engine_busy(){};
        GXSTAT.write(GxStatOpts::new().with_clear_fifo(true));
        gl_reset_matrix_stack();
        gl_flush(SwapBuffers::new());
        if let Some(c) = disp3dcnt{
            DISP3DCNT.write(c);
        }else{
            DISP3DCNT.write(Disp3DCntOpts::new());
        }
        gl_clear_color(0, 0, 0, 31);
        gl_clear_poly_id(63);
        gl_clear_depth(I13F3::MAX);
        TEX_IMAGE_PARAM.write(TexImageParam::new());
        POLYGON_ATTR.write(PolygonAttr::new());
        gl_matrix_mode(GlMatrixModeEnum::GlProjection);
        gl_load_identity();
        gl_matrix_mode(GlMatrixModeEnum::GlModelView);
        gl_load_identity();
        gl_matrix_mode(GlMatrixModeEnum::GlTexture);
        gl_load_identity();
    }
    pub fn gl_view_port(viewport: ViewPort){
        VIEWPORT.write(viewport);
    }
    pub fn gl_clear_depth(depth:I13F3){
        CLEAR_DEPTH.write(depth);
    }
    pub fn gl_clear_poly_id(id:u8){
        CLEAR_COLOR.write(ClearColor::new().with_poly_id(id));
    }
    pub fn gl_clear_color(r:u8, g:u8, b:u8, a:u8){
        GL_GLOBALS.borrow_mut().clear_color = ClearColor::new().with_a(a).with_r(r).with_g(g).with_b(b);
        CLEAR_COLOR.write(ClearColor::new().with_a(a).with_r(r).with_g(g).with_b(b));
    }
    pub fn gl_flush(sb:SwapBuffers){
        SWAPBUFFRES.write(sb);
    }
    pub fn gl_reset_matrix_stack(){
        while GXSTAT.read().get_matrix_stack_busy(){
            let mut stat = GXSTAT.read();
            stat.set_matrix_stack_error(true);
            GXSTAT.write(stat);
        }
        if GXSTAT.read().get_projection_matrix_stack(){
            gl_matrix_mode(GlMatrixModeEnum::GlProjection);
            gl_matrix_pop(1);
        }
        gl_matrix_mode(GlMatrixModeEnum::GlModelView);
        gl_matrix_pop((GXSTAT.read().get_position_vector_matrix_stack() & 0x1F)as i32);
        gl_matrix_mode(GlMatrixModeEnum::GlModelView);
        gl_load_identity();
        gl_matrix_mode(GlMatrixModeEnum::GlProjection);
        gl_load_identity();
        gl_matrix_mode(GlMatrixModeEnum::GlTexture);
        gl_load_identity();
    }
    pub fn gl_matrix_mode(mode:GlMatrixModeEnum){
        let mtx_mode = MatrixMode::new().with_matrix_mode(mode);
        MTXMODE.write(mtx_mode);
    }
    pub fn gl_matrix_pop(count:i32){
        let pop_count = MatrixPop::new().with_matrix_pop(count);
        MTXPOP.write(pop_count);
    }
    pub fn gl_load_identity(){
        MTXIDENTITY.write(MatrixIdentity::new());
    }
} 