#![no_std]
#![feature(asm)]
extern crate volatile;

pub mod consts;
pub mod regs;

use consts::*;
pub use regs::*;
use core::panic::PanicInfo;
use core::ptr;

#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    loop {}
}

#[no_mangle]
pub unsafe extern "C" fn Reset() -> ! {
    IME.write_volatile(0x00000000);
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
    ");

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
pub fn sync(){
    unsafe{
        IPCSYNC.write_volatile(0x7<<8);
        while IPCSYNC.read_volatile() & 0xF != 0x9 {}
    }
}
pub fn spi_fw_init(){
    unsafe{
        SPICNT.write_volatile(SPI_BUS_ENABLE|SPI_CHIPSELECT_HOLD|FIRMWARE_SELECT);
        SPIDATA.write_volatile(FW_WRITE_DISABLE);
        wait_spi_busy();
    }
}
pub fn read_fw_dw(addr:u32,buff:&mut [u8]){
    unsafe{
        SPIDATA.write_volatile(FW_READ);
        SPIDATA.write_volatile((addr >> 16) as u8);
        SPIDATA.write_volatile((addr >> 8) as u8);
        SPIDATA.write_volatile(addr as u8);
        wait_spi_busy();
        for i in 0..buff.len(){
            buff[i] = SPIDATA.read_volatile();
            wait_spi_busy();
        }  
    }
}
pub fn wait_spi_busy(){
    unsafe{
        while (SPICNT.read_volatile() & SPI_BUSY_FLAG) == SPI_BUSY_FLAG {}
    }
}
pub fn wifi_init() {
    unsafe{
        POWCNT2.write_volatile(0x00000003);
        let mut buff = [0;8];
        read_fw_dw(0x36,&mut buff[2..8]);
        let h = u8_to_u32(&buff[0..4]);
        let l = u8_to_u32(&buff[4..8]);
        IPCFIFOSEND.write_volatile(l);
        IPCFIFOSEND.write_volatile(h);
        IPCFIFOCNT.write_volatile(8);
    }
}
pub fn u8_to_u32(val:&[u8])->u32{
        let b0 = val[0] as u32;
        let b1 = val[1] as u32;
        let b2 = val[2] as u32;
        let b3 = val[3] as u32;
        (b3 << 24) | (b2 << 16) | (b1 << 8) | (b0 << 0) 
    }
pub fn halt() -> !{
    loop {
        unsafe{
            asm!("swi 0x60000");
        }
    }
}

#[link_section = ".irq"]
pub static IRQ_HANDLE:unsafe extern "C" fn()= irq_handler;

unsafe extern "C" fn irq_handler(){
    IPCSYNC.write_volatile(1<<13);
    halt();
}
