#![no_std]
extern crate voladdress;

pub mod consts;
pub mod regs;

use consts::*;
use core::panic::PanicInfo;
use core::arch::asm;
use core::ptr;
pub use regs::*;

#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    IME.write(0x00000000);
    loop{}
}

#[link_section = ".crt0"]
#[no_mangle]
pub unsafe extern "C" fn Reset() -> ! {
    IME.write(0x00000000);
    extern "C" {
        static mut __iwram_start: u8;
        static mut __iwram_top: u8;

        static mut __irq_vector: u8;
        static mut __irq_flags: u8;
        static mut __irq_flagsaux: u8;

        static mut __sp_irq: u8;
        static mut __sp_svc: u8;
        static mut __sp_usr: u8;

        static mut _siirq: u8;
    }
    ptr::copy_nonoverlapping(&_siirq as *const u8, &mut __irq_vector as *mut u8, 4);

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
    "
    );
    extern "C" {
        static mut _sbss: u8;
        static mut _ebss: u8;

        static mut _sdata: u8;
        static mut _edata: u8;
        static _sidata: u8;
    }

    let count = &_ebss as *const u8 as usize - &_sbss as *const u8 as usize;
    ptr::write_bytes(&mut _sbss as *mut u8, 0, count);

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
pub fn sync() {
    IPCSYNC.write(0x7 << 8);
    while IPCSYNC.read() & 0xF != 0x9 {}
}
pub fn spi_fw_init() {
    SPICNT.write(SPI_BUS_ENABLE | SPI_CHIPSELECT_HOLD | FIRMWARE_SELECT);
    SPIDATA.write(FW_WRITE_DISABLE);
    wait_spi_busy();
}
pub fn read_fw_dw(addr: u32, buff: &mut [u8]) {
    SPIDATA.write(FW_READ);
    SPIDATA.write((addr >> 16) as u8);
    SPIDATA.write((addr >> 8) as u8);
    SPIDATA.write(addr as u8);
    wait_spi_busy();
    for i in 0..buff.len() {
        buff[i] = SPIDATA.read();
        wait_spi_busy();
    }
}
pub fn wait_spi_busy() {
    while (SPICNT.read() & SPI_BUSY_FLAG) == SPI_BUSY_FLAG {}
}
pub fn wifi_init() {
    POWCNT2.write(0x00000003);
    let mut buff = [0; 8];
    read_fw_dw(0x36, &mut buff[2..8]);
    let h = u8_to_u32(&buff[0..4]);
    let l = u8_to_u32(&buff[4..8]);
    IPCFIFOSEND.write(l);
    IPCFIFOSEND.write(h);
    IPCFIFOCNT.write(8);
}
pub fn u8_to_u32(val: &[u8]) -> u32 {
    let b0 = val[0] as u32;
    let b1 = val[1] as u32;
    let b2 = val[2] as u32;
    let b3 = val[3] as u32;
    (b3 << 24) | (b2 << 16) | (b1 << 8) | (b0 << 0)
}

#[link_section = ".irq"]
pub static IRQ_HANDLE: unsafe extern "C" fn() = irq_handler;

unsafe extern "C" fn irq_handler() {
    IPCSYNC.write(1 << 13);
}
