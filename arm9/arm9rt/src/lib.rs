#![no_std]
#![feature(asm)]
#![feature(trait_alias)]
#![feature(associated_type_bounds)]
#![feature(const_raw_ptr_deref)]
#![feature(const_fn)]

extern crate volatile;

pub mod irq;
pub mod regs;
pub mod sync;
pub mod write;
pub mod consts;

use core::ptr;
pub use irq::*;
pub use regs::*;
pub use sync::*;
pub use write::*;
pub use consts::*;
use core::panic::PanicInfo;

global_writer!(WRITER);

#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    println!("{}",_panic);
    IME.write(0x00000000);
    halt();
}
 
#[no_mangle]
pub unsafe extern "C" fn Reset() -> ! {
    
    extern "C"{
        static mut __iwram_start: u8;
        static mut __iwram_top: u8;

        static mut  __irq_vector: u8;
        static mut __irq_flags: u8;
        static mut __irq_flagsaux: u8;

        static mut __sp_irq: u8;
        static mut __sp_svc: u8;
        static mut __sp_usr: u8;

        static mut _siirq:u8;
    }    
    
    asm!("
	mov	r5, #0x12
	msr	cpsr, r5
	ldr	sp, =__sp_irq

	mov	r5, #0x13
	msr	cpsr, r5
	ldr	sp, =__sp_svc

	mov	r5, #0x1F
	msr	cpsr, r5
	ldr	sp, =__sp_usr
    ");

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

    ptr::copy_nonoverlapping(&_siirq as *const u8, &mut __irq_vector as *mut u8, 4);

    sync();
    extern "Rust" {
        fn main() -> !;
    }

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
    }
}

pub fn halt()->!{
    unsafe{
        loop{
            asm!("swi 0x60000");
        }
    }
}
