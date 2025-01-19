#![no_std]
#![no_main]
#![feature(asm)]
#![feature(default_alloc_error_handler)]
#![feature(const_float_bits_conv)]
extern crate arm9rt;
extern crate alloc;
extern crate tinybmp;
extern crate simba;
use core::arch::asm;
use embedded_graphics::prelude::*;
use arm9rt::*;
use arm9rt::dma::*;
pub mod vertices;
pub mod world;
use vertices::*;
use world::*;
use simba::scalar::FixedI13F3;
use arm9rt::{a::*, regs::*, video::e3d::*};
use arm9rt::dma::SrcAddrControl::Fixed;
use fixed::types::I13F3;
use arm9rt::video::e3d::GXFIFO;
entry!(main);
const Triangle:[u32;12] = [
    FIFO_COMMAND_PACK(GXFIFO::BEGIN_VTXS as u32, GXFIFO::COLOR as u32, GXFIFO::VTX_16 as u32, GXFIFO::COLOR as u32),
    Begin::Triangles as u32,
    RGB15(31,0,0) as u32,
    VERTEX_PACK(INT_TO_V16(-1), INT_TO_V16(-1)) as u32, VERTEX_PACK(0,0) as u32,
    RGB15(0,31,0) as u32,
    FIFO_COMMAND_PACK(GXFIFO::VTX_16 as u32, GXFIFO::COLOR as u32, GXFIFO::VTX_16 as u32, GXFIFO::END_VTXS as u32),
    VERTEX_PACK(INT_TO_V16(1), INT_TO_V16(-1)) as u32, VERTEX_PACK(0,0) as u32,
    RGB15(0,0,31) as u32,
    VERTEX_PACK(INT_TO_V16(0), INT_TO_V16(1)) as u32, VERTEX_PACK(0,0) as u32,
];
fn main() -> ! {
    start_fnt();
    IME.write(0);
    IE.write(0);
    let dispcnt = DispCnt::new().with_BG0_en(true).with_BG0_3D(true).with_bg_mode(0).with_display_mode(1);
    DISPCNT.write(dispcnt);
    let vram_cnt = VramCnt::new().with_vram_enable(true).with_vram_mst(3);
    VRAMCNT_A.write(vram_cnt.with_vram_offset(0));
    VRAMCNT_B.write(vram_cnt.with_vram_offset(1));
    VRAMCNT_C.write(vram_cnt.with_vram_offset(2));
    VRAMCNT_D.write(vram_cnt.with_vram_offset(3));
    let e3d = GL::new(None, None);
    let mut r = 0;
    let mut g = 0;
    let mut b = 0;
    GL::clear_color(r,g,b,31);
    GL::clear_poly_id(63);
    GL::clear_depth(FixedI13F3::from_bits(0x7FFF));
    let viewport = ViewPort::new().with_x1(0).with_y1(0).with_x2(255).with_y2(191);
    GL::view_port(viewport);
    GL::matrix_mode(GlMatrixModeEnum::GlProjection);
    GL::load_identity();
    loop {
        if KEYINPUT.read() & 0x0010 == 0 {
            r +=1;
        }
        if KEYINPUT.read() & 0x0020 == 0 {
            g += 1;
        }
        if KEYINPUT.read() & 0x0040 == 0 {
            b += 1;
        }
        if KEYINPUT.read() & 0x0080 == 0 {
            
        }

        GL::clear_poly_id(63);
        GL::poly_format(PolygonAttr::new().with_show_front(true).with_alpha(0x1F));
        GL::call_list(&Triangle);
        GL::matrix_pop(1);
        //GL::clear_color(r, g, b, 31);
        GL::flush(SwapBuffers::new());//SwapBuffers::new().with_auto_sort(true).with_depth_buffering(true));
        //particle.draw_sprites(&mut bg3, &TEXTURES);
        unsafe{
            asm!("swi 0x50000")
        }
    }
}
