#![no_std]
#![feature(trait_alias)]
#![feature(allocator_api)]
#[allow(static_mut_refs)]
pub(crate) use voladdress::{VolAddress, VolBlock};
extern crate alloc;
extern crate derive_more;
mod critical_section;
pub mod consts;
pub mod dma;
pub mod dynamic_array;
pub mod irq;
pub mod macros;
pub mod power;
pub mod regs;
pub mod sync;
pub mod video;
pub mod write;
mod new_wram;

pub use consts::*;
use core::arch::asm;
use core::panic::PanicInfo;
use core::ptr;
pub use irq::*;
pub use regs::*;
pub use sync::*;
pub use video::{a, vram::*};
pub use write::*;

#[unsafe(link_section = ".secure")]
#[unsafe(no_mangle)]
pub static SECURE_AREA: [u8; 0x800] = [0; 0x800];

unsafe extern "C" {
    static mut _sheap: u8;
    static mut ___sheap_end: u8;
}
use embedded_alloc::LlffHeap as Heap;

#[global_allocator]
static HEAP: Heap = Heap::empty();

use write::Writer;
global_writer!(Writer { console_x: 0, console_y: 0 });

#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    start_fnt();
    println!("{}", _panic);
    halt()
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Reset() -> ! {
    unsafe extern "C" {
        static mut __iwram_start: u8;
        static mut __iwram_top: u8;

        static mut __irq_vector: u8;
        static mut __irq_flags: u8;
        static mut __irq_flagsaux: u8;

        static mut __sp_irq: u8;
        static mut __sp_svc: u8;
        static mut __sp_usr: u8;
    }
    /*
    extern "C"{
        fn set_sp();
    }
        set_sp();
    */
    asm!(
        "mov	r5, #0x12",
        "msr	cpsr, r5",
        "ldr	sp, =__sp_irq",
        "mov	r5, #0x13",
        "msr	cpsr, r5",
        "ldr	sp, =__sp_svc",
        "mov	r5, #0x1F",
        "msr	cpsr, r5",
        "ldr	sp, =__sp_usr",
    );
    unsafe extern "C" {
        fn __libnds_mpu_setup();
    }
    __libnds_mpu_setup();

    unsafe extern "C" {
        static mut _sbss: u8;
        static mut _ebss: u8;

        static mut _sdata: u8;
        static mut _edata: u8;
        static _sidata: u8;

    }

    let count = &raw const _ebss as *const u8 as usize - &raw const _sbss as *const u8 as usize;
    ptr::write_bytes(&raw mut _sbss as *mut u8, 0, count);

    let count = &raw const _edata as *const u8 as usize - &raw const _sdata as *const u8 as usize;
    ptr::copy_nonoverlapping(&_sidata as *const u8, &raw mut _sdata as *mut u8, count);

    ptr::write_volatile(0x03FFFFFC as *mut extern "C" fn(), IRQ_HANDLE);

    unsafe {
        embedded_alloc::init!(HEAP, 8*1024*1024);
    }

    unsafe extern "Rust" {
        fn main() -> !;
    }
    main()
}

#[macro_export]
macro_rules! entry {
    ($path:path) => {
        #[unsafe(export_name = "main")]
        pub unsafe fn __main() -> ! {
            // type check the given path
            let f: fn() -> ! = $path;

            f()
        }
    };
}

pub fn halt() -> ! {
    IME.write(false);
    unsafe {
        asm!("swi 0x60000");
    };
    loop {}
}
