#![no_std]
#![no_main]
#![feature(asm)]

use arm9rt::*;

#[link_section = ".secure"]
#[no_mangle]
pub static SECURE_AREA:[u8;0x800] = [0;0x800];

entry!(main);

fn main() -> !{
    cls();
    start_fnt();
    
    panic!("END");
}
