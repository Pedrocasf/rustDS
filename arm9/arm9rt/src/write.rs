use crate::regs::*;

use core::fmt;
pub const COLOR:u16 = (5<<8);
#[macro_export]
macro_rules! rgb8{
    ($r:expr, $g:expr, $b:expr) => {
        (($b >> 3) << 10) | (($g >> 3) << 5) | ($r >>3) 
    }
}
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

pub fn cls(){
    POWCNT1.write(0x0003);
    DISPCNT.write(0x00020000);
    VRAMCNT_A.write(0x80);
    VRAMCNT_B.write(0x80);
    for i in 0..=0xFFFF{
        VRAMBASE[i].write(0x0020);
    }
}
const FNT:&[u8] = include_bytes!("amiga.fnt");
pub fn start_fnt(){
    DISPCNT.write(0x00010100);
    BG0CNT.write(0x0400);
    VRAMCNT_A.write(0x81);
    VRAMCNT_B.write(0x89);
    BGPRAM[0x0].write(rgb8!(0,0,0));
    BGPRAM[0x1].write(rgb8!(0,0,171));
    BGPRAM[0x2].write(rgb8!(0,171,0));
    BGPRAM[0x3].write(rgb8!(0,171,171));
    BGPRAM[0x4].write(rgb8!(171,0,0));
    BGPRAM[0x5].write(rgb8!(171,0,171));
    BGPRAM[0x6].write(rgb8!(171,171,0));
    BGPRAM[0x7].write(rgb8!(211,211,211));
    BGPRAM[0x8].write(rgb8!(171,171,171));
    BGPRAM[0x9].write(rgb8!(0,0,255));
    BGPRAM[0xA].write(rgb8!(0,255,0));
    BGPRAM[0xB].write(rgb8!(0,255,255));
    BGPRAM[0xC].write(rgb8!(255,0,0));
    BGPRAM[0xD].write(rgb8!(255,0,255));
    BGPRAM[0xE].write(rgb8!(255,255,0));
    BGPRAM[0xF].write(rgb8!(255,255,255));
    for i in 0..FNT.len(){
        let mut dst:u32 = 0;
        let mut src:u32 = FNT[i] as u32;
        for _ in 0..8 {
            dst = dst << 4;
            dst |= ((src & 1) << 4).saturating_sub(1);
            src = src >> 1;
        }
        VRAMABG[i as usize].write(dst);
    }
}
pub struct Writer {
    pub console_x:u8,
    pub console_y:u8,
}
impl Writer{
    pub fn write_byte(&mut self,c:u8){
        if c == 0xA{
            self.console_x = 0;
        }
        if c == 0xD || c == 0xA{
            self.console_y = self.console_y.wrapping_add(1);
            return;
        }
        let off = self.console_x as u32 | ((self.console_y as u32) << 5);
        let (x,o) = self.console_x.overflowing_add(1);
        self.console_x = x;
        self.console_y = self.console_y.wrapping_add(o as u8);
        VRAMABG[off as usize + 0x1000].write(c as u32)
    }
}
impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for byte in s.bytes() {
          self.write_byte(byte)
        }
        Ok(())
    }
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    unsafe{
        extern "Rust" {
            static mut WRITER: &'static mut dyn Write;
        }
        WRITER.write_fmt(args).unwrap();
    }
}
#[macro_export]
macro_rules! global{
    ($writer:expr, $name:ident, $type:ident) => { 
        #[no_mangle]
        pub static mut $name: &dyn $type = &$writer;
    };
}
pub trait Writee = core::fmt::Write + Sync;
#[macro_export]
macro_rules! global_writer{
    ($name:ident) => {
        global!(Writer{console_x:0,console_y:0},$name,Writee);
    };
}