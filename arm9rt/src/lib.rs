#![no_std]
#![feature(trait_alias)]
#![feature(allocator_api)]
pub(crate) use voladdress::{VolAddress, VolBlock};
extern crate alloc;
extern crate buddy_alloc;
extern crate derive_more;
extern crate simba;
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
use buddy_alloc::*;
pub use consts::*;
use core::arch::asm;
use core::panic::PanicInfo;
use core::ptr;
pub use irq::*;
pub use regs::*;
pub use sync::*;
pub use video::{a, vram::*};
pub use write::*;

#[link_section = ".secure"]
#[no_mangle]
pub static SECURE_AREA: [u8; 0x800] = [0; 0x800];

extern "C" {
    static mut _sheap: u8;
    static mut _heap_size: u8;
}
const FAST_HEAP_SIZE: usize = 8 * 1024 * 1024; // 8 MB
const HEAP_SIZE: usize = 4 * 1024 * 1024; // 4M
static mut FAST_HEAP: Heap<FAST_HEAP_SIZE> = Heap([0u8; FAST_HEAP_SIZE]);
static mut HEAP: Heap<HEAP_SIZE> = Heap([0u8; HEAP_SIZE]);
const LEAF_SIZE: usize = 16;
#[repr(align(64))]
struct Heap<const S: usize>([u8; S]);
#[cfg_attr(not(test), global_allocator)]
static ALLOC: NonThreadsafeAlloc = unsafe {
    let fast_param = FastAllocParam::new(FAST_HEAP.0.as_ptr(), FAST_HEAP_SIZE);
    let buddy_param = BuddyAllocParam::new(HEAP.0.as_ptr(), HEAP_SIZE, LEAF_SIZE);
    NonThreadsafeAlloc::new(fast_param, buddy_param)
};

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
