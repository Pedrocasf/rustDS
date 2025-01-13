use core::ffi::c_void;
use crate::dma::*;
use voladdress::*;
use fixed::types::I9F23;
use embedded_graphics_core::{
  draw_target::DrawTarget,
  geometry::{
    OriginDimensions,
    Size,
  },
  pixelcolor::{
    PixelColor,
    raw::RawU32
  },
  Pixel,
};
impl PixelColor for U32{
    type Raw = RawU32;
  }
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct U32(pub u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Page {
  Zero,
  One,
}

#[derive(Default)]
pub struct BG3;

impl BG3{
    pub const WIDTH:usize = 256;
    pub const WORD_WIDTH:usize = 64;
    pub const HEIGHT:usize = 192;
    
    const PIXELS: VolBlock<u8, Safe, Safe, 0x80000> = unsafe { VolBlock::new(0x0600_0000) };
    pub const WORDS: VolBlock<u32, Safe, Safe, 0x4000> = unsafe { VolBlock::new(0x0600_0000) };
    pub const HWORDS: VolBlock<u16, Safe, Safe, 0x8000> = unsafe { VolBlock::new(0x0600_0000) };


    pub fn read(col: usize, row: usize) -> Option<u8> {
      Self::PIXELS.get(col + row * Self::WIDTH).map(VolAddress::<u8, Safe, Safe>::read)
    }
    pub fn write_word(col: usize, row: usize, colors: u32) -> Option<()> {
        Self::WORDS
            .get(col + row * Self::WORD_WIDTH)
            .map(|va| va.write(colors))
    }
}

impl DrawTarget for BG3{
    type Color = U32;
    type Error = core::convert::Infallible;
    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        for Pixel(coord, color) in pixels.into_iter() {
            BG3::write_word(coord.x as usize, coord.y as usize, color.0);
        }
        Ok(())
    }
}
impl OriginDimensions for BG3{
  fn size(&self) -> Size {
      Size::new(BG3::WIDTH as u32, BG3::HEIGHT as u32)
  }
}
#[derive(Default)]
pub struct BG2;

impl BG2{
    pub const WIDTH:usize = 256;
    pub const WORD_WIDTH:usize = 64;
    pub const HEIGHT:usize = 192;
    pub const FIX_HEIGHT:I9F23 = I9F23::from_bits(192<<23);
    pub const FIX_WIDTH:I9F23 = I9F23::from_bits(256<<23);
    const PIXELS: VolBlock<u8, Safe, Safe, 0xC000> = unsafe { VolBlock::new(0x0603_C000) };
    pub const WORDS: VolBlock<u32, Safe, Safe, 0x3000> = unsafe { VolBlock::new(0x0603_C000) };
    pub const HWORDS: VolBlock<u16, Safe, Safe, 0x6000> = unsafe { VolBlock::new(0x0600_0000) };


    pub fn read(col: usize, row: usize) -> Option<u8> {
      Self::PIXELS.get(col + row * Self::WIDTH).map(VolAddress::<u8,Safe,Safe>::read)
    }
    pub fn write_word(col: usize, row: usize, colors: u32) -> Option<()> {
        Self::WORDS
            .get(col + (row * Self::WORD_WIDTH))
            .map(|va| va.write(colors))
    }
    pub fn write_px(col: usize, row: usize, colors: u8) -> Option<()> {
      Self::PIXELS
          .get(col + row * Self::WIDTH)
          .map(|va| va.write(colors))
  }
    pub fn dma_clear(){
      use crate::dma::*;
      const FILL_CONTROL: DmaControl = DmaControl::new()
      .with_src_addr_control(SrcAddrControl::Fixed)
      .with_dest_addr_control(DestAddrControl::Increment)
      .with_transfer_32bit(true)
      .with_enabled(true);
    unsafe{
      DMA3_SRC.write(0 as *const c_void);
      DMA3_DEST.write(((Self::WORDS.index(0).as_usize() as *mut u32) as usize) as *mut c_void);
      DMA3_COUNT.write(0x3000);
      DMA3_CONTROL.write(FILL_CONTROL);
    }
    }
}

impl DrawTarget for BG2{
    type Color = U32;
    type Error = core::convert::Infallible;
    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        for Pixel(coord, color) in pixels.into_iter() {
            BG2::write_word(coord.x as usize, coord.y as usize, color.0);
        }
        Ok(())
    }
}
impl OriginDimensions for BG2{
  fn size(&self) -> Size {
      Size::new(BG2::WIDTH as u32, BG2::HEIGHT as u32)
  }
}