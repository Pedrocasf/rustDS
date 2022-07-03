#![no_std]
#![feature(asm)]
#![feature(trait_alias)]
#![feature(associated_type_bounds)]
#![feature(const_raw_ptr_deref)]
#![feature(default_alloc_error_handler)]

pub(crate) use voladdress::{VolAddress, VolBlock};
extern crate linked_list_allocator;
extern crate derive_more;
extern crate alloc;
pub mod consts;
pub mod displays;
pub mod irq;
pub mod regs;
pub mod sync;
pub mod write;
pub mod dma;
pub mod macros;
pub mod power;
pub mod dynamic_array;
pub use consts::*;
use core::panic::PanicInfo;
use core::ptr;
use core::arch::asm;
pub use displays::{
    vram::*,
    a,
};
pub use irq::*;
pub use regs::*;
pub use sync::*;
pub use write::*;

#[link_section = ".secure"]
#[no_mangle]
pub static SECURE_AREA: [u8; 0x800] = [0; 0x800];

extern "C" {
    static mut _sheap :u8;
    static mut _heap_size: u8;
}

#[global_allocator]
static mut ALLOCATOR: linked_list_allocator::CellHeap = linked_list_allocator::CellHeap::ALLOC;

pub fn init_heap() {
    unsafe {
        ALLOCATOR.borrow_mut().init(&_sheap as *const u8 as usize ,&_heap_size as *const u8 as usize);
    }
}

global_writer!(WRITER);

#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    start_fnt();
    println!("{}", _panic);
    halt()
}

#[no_mangle]
pub unsafe extern "C" fn Reset() -> ! {
    extern "C" {
        static mut __iwram_start: u8;
        static mut __iwram_top: u8;

        static mut __irq_vector: u8;
        static mut __irq_flags: u8;
        static mut __irq_flagsaux: u8;

        static mut __sp_irq: u8;
        static mut __sp_svc: u8;
        static mut __sp_usr: u8;
    }

    asm!(
        "
	mov	r5, #0x12
	msr	cpsr, r5
	ldr	sp, =__sp_irq

	mov	r5, #0x13
	msr	cpsr, r5
	ldr	sp, =__sp_svc

	mov	r5, #0x1F
	msr	cpsr, r5
	ldr	sp, =__sp_usr
    "
    );

    extern "C" {
        fn __libnds_mpu_setup();
    }
    __libnds_mpu_setup();

    extern "C" {
        static mut _sbss: u8;
        static mut _ebss: u8;

        static mut _sdata: u8;
        static mut _edata: u8;
        static _sidata: u8;

    }

    let count = &_ebss as *const u8 as usize - &_sbss as *const u8 as usize;
    ptr::write_bytes(&mut _sbss as *mut u8, 0, count);

    let count = &_edata as *const u8 as usize - &_sdata as *const u8 as usize;
    ptr::copy_nonoverlapping(&_sidata as *const u8, &mut _sdata as *mut u8, count);

    ptr::write_volatile(0x03FFFFFC as *mut extern "C" fn(), IRQ_HANDLE);

    extern "Rust" {
        fn main() -> !;
    }
    init_heap();
    main()
}

#[macro_export]
macro_rules! entry {
    ($path:path) => {
        #[export_name = "main"]
        pub unsafe fn __main() -> ! {
            let f: fn() -> ! = $path;
            f()
        }
    };
}

pub fn halt() -> ! {
    IME.write(0x00000000);
    unsafe {
        asm!("swi 0x60000");
    };
    loop {}
}
