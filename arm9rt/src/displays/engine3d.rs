pub mod E3D{
    use crate::macros::*;
    use alloc::boxed::Box;
    use core::{borrow::BorrowMut, u32, usize};
    use crate::dynamic_array::DynamicArray;
    use crate::power::POWCNT::{POWCNT1, PowCnt1Opts};
    use voladdress::*;
    use crate::*;
    use linked_list_allocator::{RefCell, RefMut};
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
        bitfield_bool!(u32;0,get_AutoSort,with_AutoSort,set_AutoSort);
        bitfield_bool!(u32;1,get_DepthBuffering,with_DepthBuffering,set_DepthBuffering);
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
        bitfield_int !(i32;0..=5:i32,get_MatrixPop,with_MatrixPop,set_MatrixPop);
    }
    pub const MTXPOP :VolAddress<MatrixPop,(),Safe> = unsafe { VolAddress::new(0x04000448) };
    #[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct MatrixMode(u32);
    impl MatrixMode{
        const_new!();
        bitfield_enum!(u32;0..=1:GlMatrixModeEnum,get_MatrixMode,with_MatrixMode,set_MatrixMode);
    }
    pub const MTXMODE :VolAddress<MatrixMode,(),Safe> = unsafe { VolAddress::new(0x04000440) };
    #[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct GxStatOpts(u32);
    impl GxStatOpts{
        const_new!();
        bitfield_int! (u32;8..=12:u32,get_PositionVectorMatrixStack,with_PositionVectorMatrixStack,set_PositionVectorMatrixStack);
        bitfield_bool!(u32;13,get_ProjectionMatrixStack,with_ProjectionMatrixStack,set_ProjectionMatrixStack);
        bitfield_bool!(u32;14,get_MatrixStackBusy,with_MatrixStackBusy,set_MatrixStackBusy);
        bitfield_bool!(u32;15,get_MatrixStackError,with_MatrixStackError,set_MatrixStackError);
        bitfield_bool!(u32;27,get_GeometryEngineBusy,with_GeometryEngineBusy,set_GeometryEngineBusy);
        bitfield_bool!(u32;29,get_ClearFifo,with_ClearFifo,set_ClearFifo);        
    }
    pub const GXSTAT: VolAddress<GxStatOpts, Safe, Safe> = unsafe { VolAddress::new(0x04000600) };
    impl_bitwise_ops!(GxStatOpts);
    #[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct Disp3DCntOpts(u16);

    impl Disp3DCntOpts {
        const_new!();
        bitfield_bool!(u16;0,get_EnableTextureMapping,with_EnableTextureMapping,set_EnableTextureMapping);
        bitfield_bool!(u16;1,get_ToonHighlightShading,with_ToonHighlightShading,set_ToonHighlightShading);
        bitfield_bool!(u16;2,get_EnableAlphaTest ,with_EnableAlphaTest ,set_EnableAlphaTest);
        bitfield_bool!(u16;3,get_EnableAlphaBlending,with_EnableAlphaBlending,set_EnableAlphaBlending);
        bitfield_bool!(u16;4,get_EnableAntiAliasing,with_EnableAntiAliasing,set_EnableAntiAliasing);
        bitfield_bool!(u16;5,get_EnableEdgeMarking,with_EnableEdgeMarking,set_EnableEdgeMarking);
        bitfield_bool!(u16;6,get_FogColorAlpha,with_FogColorAlpha ,set_FogColorAlpha );
        bitfield_bool!(u16;7,get_EenableFog,with_EenableFog,set_EenableFog);
        bitfield_int !(u16;8..=11:u16,get_FogDepthShift,with_FogDepthShift,set_FogDepthShift);
        bitfield_bool!(u16;12,get_UndeflowAck,with_UndeflowAck,set_UndeflowAck);
        bitfield_bool!(u16;13,get_OverflowAck,with_OverflowAck,set_OverflowAck);
        bitfield_bool!(u16;14,get_EnableRearBmp,with_EnableRearBmp,set_EnableRearBmp);
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
    pub fn glInit(disp3dcnt:Option<Disp3DCntOpts>, powcnt1:Option<PowCnt1Opts>){
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
        while GXSTAT.read().get_GeometryEngineBusy(){};
        GXSTAT.write(GxStatOpts::new().with_ClearFifo(true));
        glResetMatrixStack();
        glFlush(SwapBuffers::new());
        if let Some(c) = disp3dcnt{
            DISP3DCNT.write(c);
        }else{
            DISP3DCNT.write(Disp3DCntOpts::new());
        }
        glClearColor(0, 0, 0, 31);
        glClearPolyId(63);
        glClearDepth(I13F3::MAX);
        TEX_IMAGE_PARAM.write(TexImageParam::new());
        POLYGON_ATTR.write(PolygonAttr::new());
        glMatrixMode(GlMatrixModeEnum::GlProjection);
        glLoadIdentity();
        glMatrixMode(GlMatrixModeEnum::GlModelView);
        glLoadIdentity();
        glMatrixMode(GlMatrixModeEnum::GlTexture);
        glLoadIdentity();
    }
    pub fn glViewPort(viewport: ViewPort){
        VIEWPORT.write(viewport);
    }
    pub fn glClearDepth(depth:I13F3){
        CLEAR_DEPTH.write(depth);
    }
    pub fn glClearPolyId(id:u8){
        CLEAR_COLOR.write(ClearColor::new().with_poly_id(id));
    }
    pub fn glClearColor(r:u8,g:u8,b:u8,a:u8){
        GL_GLOBALS.borrow_mut().clear_color = ClearColor::new().with_a(a).with_r(r).with_g(g).with_b(b);
        CLEAR_COLOR.write(ClearColor::new().with_a(a).with_r(r).with_g(g).with_b(b));
    }
    pub fn glFlush(sb:SwapBuffers){
        SWAPBUFFRES.write(sb);
    }
    pub fn glResetMatrixStack(){
        while GXSTAT.read().get_MatrixStackBusy(){
            let mut stat = GXSTAT.read();
            stat.set_MatrixStackError(true);
            GXSTAT.write(stat);
        }
        if GXSTAT.read().get_ProjectionMatrixStack(){
            glMatrixMode(GlMatrixModeEnum::GlProjection);
            glMatrixPop(1);
        }
        glMatrixMode(GlMatrixModeEnum::GlModelView);
        glMatrixPop((GXSTAT.read().get_PositionVectorMatrixStack() & 0x1F)as i32);
        glMatrixMode(GlMatrixModeEnum::GlModelView);
        glLoadIdentity();
        glMatrixMode(GlMatrixModeEnum::GlProjection);
        glLoadIdentity();
        glMatrixMode(GlMatrixModeEnum::GlTexture);
        glLoadIdentity();
    }
    pub fn glMatrixMode(mode:GlMatrixModeEnum){
        let mtx_mode = MatrixMode::new().with_MatrixMode(mode);
        MTXMODE.write(mtx_mode);
    }
    pub fn glMatrixPop(count:i32){
        let pop_count = MatrixPop::new().with_MatrixPop(count);
        MTXPOP.write(pop_count);
    }
    pub fn glLoadIdentity(){
        MTXIDENTITY.write(MatrixIdentity::new());
    }
} 