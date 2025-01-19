use crate::video::vram::*;
use crate::regs::*;
use core::fmt;
pub const COLOR: u16 = 5 << 8;
#[macro_export]
macro_rules! rgb8 {
    ($r:expr, $g:expr, $b:expr) => {
        (($b >> 3) << 10) | (($g >> 3) << 5) | ($r >> 3)
    };
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

const FNT: &[u8] = include_bytes!("amiga.fnt");
pub fn start_fnt() {
    let vram_cnt = VramCnt::new().with_vram_enable(true).with_vram_mst(1);
    DISPCNT_B.write(0x00010100);
    BG0CNT_B.write(0x0400);
    VRAMCNT_H.write(vram_cnt);
    VRAMCNT_I.write(vram_cnt);
    BG_PALLETES_B.index(0x0).write(rgb8!(0, 0, 0));
    BG_PALLETES_B.index(0x1).write(rgb8!(0, 0, 171));
    BG_PALLETES_B.index(0x2).write(rgb8!(0, 171, 0));
    BG_PALLETES_B.index(0x3).write(rgb8!(0, 171, 171));
    BG_PALLETES_B.index(0x4).write(rgb8!(171, 0, 0));
    BG_PALLETES_B.index(0x5).write(rgb8!(171, 0, 171));
    BG_PALLETES_B.index(0x6).write(rgb8!(171, 171, 0));
    BG_PALLETES_B.index(0x7).write(rgb8!(211, 211, 211));
    BG_PALLETES_B.index(0x8).write(rgb8!(171, 171, 171));
    BG_PALLETES_B.index(0x9).write(rgb8!(0, 0, 255));
    BG_PALLETES_B.index(0xA).write(rgb8!(0, 255, 0));
    BG_PALLETES_B.index(0xB).write(rgb8!(0, 255, 255));
    BG_PALLETES_B.index(0xC).write(rgb8!(255, 0, 0));
    BG_PALLETES_B.index(0xD).write(rgb8!(255, 0, 255));
    BG_PALLETES_B.index(0xE).write(rgb8!(255, 255, 0));
    BG_PALLETES_B.index(0xF).write(rgb8!(255, 255, 255));
    for i in 0..FNT.len() {
        let mut dst: u32 = 0;
        let mut src: u32 = FNT[i] as u32;
        for _ in 0..8 {
            dst <<= 4;
            dst |= ((src & 1) << 4).saturating_sub(1);
            src = src >> 1;
        }
        ENGINE_B_TILESET.index(i).write(dst);
    }
}
pub struct Writer {
    pub console_x: u8,
    pub console_y: u8,
}
impl Writer {
    pub fn write_byte(&mut self, c: u8) {
        if c == 0xA {
            self.console_x = 0;
        }
        if c == 0xD || c == 0xA {
            self.console_y = self.console_y.wrapping_add(1);
            return;
        }
        let off = self.console_x as u32 | ((self.console_y as u32) << 5);
        let (x, o) = self.console_x.overflowing_add(1);
        self.console_x = x;
        self.console_y = self.console_y.wrapping_add(o as u8);
        ENGINE_B_BG0.index(off as usize).write(c as u16)
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
    unsafe {
        extern "Rust" {
            static mut WRITER: &'static mut dyn Write;
        }
        WRITER.write_fmt(args).unwrap();
    }
}
#[macro_export]
macro_rules! global {
    ($writer:expr, $name:ident, $type:ident) => {
        #[no_mangle]
        pub static mut $name: &dyn $type = &$writer;
    };
}
pub trait Writee = core::fmt::Write + Sync;
#[macro_export]
macro_rules! global_writer {
    ($name:ident) => {
        global!(
            Writer {
                console_x: 0,
                console_y: 0
            },
            $name,
            Writee
        );
    };
}
