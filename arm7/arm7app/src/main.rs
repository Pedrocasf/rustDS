#![no_std]
#![no_main]
#![feature(asm)]

use arm7rt::*;

entry!(main);

fn main() -> ! {
    halt()
}

