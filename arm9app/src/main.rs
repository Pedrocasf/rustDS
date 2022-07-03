#![no_std]
#![no_main]
#![feature(asm)]
#![feature(default_alloc_error_handler)]
#![feature(const_float_bits_conv)]
extern crate arm9rt;
extern crate alloc;
extern crate tinybmp;
extern crate fixed;
use embedded_graphics::prelude::*;
use arm9rt::*;
use arm9rt::dma::*;
pub mod vertices;
pub mod world;
use vertices::*;
use world::*;
use fixed::types::I8F8;
use arm9rt::{a::*, regs::*,displays::E3D::*};
entry!(main);
fn main() -> ! {
    start_fnt();
    IME.write(0);
    IE.write(0);
    let mut dispcnt = DispCnt::new().with_BG0_3D(true).with_bg_mode(6).with_display_mode(1);
    DISPCNT.write(dispcnt);
    let mut e3d = glInit(None, None);
    let mut r = 31;
    let mut g = 31;
    let mut b = 31;
    let viewport = ViewPort::new().with_x1(0).with_y1(0).with_x2(255).with_y2(191);
    glViewPort(viewport);
    glMatrixMode(GlMatrixModeEnum::GlProjection);
    glLoadIdentity();
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
        glClearColor(r, g, b, 31);
        glFlush(SwapBuffers::new());
        //particle.draw_sprites(&mut bg3, &TEXTURES);
        unsafe{
            //asm!("swi 0x50000")
        }
    }
}
