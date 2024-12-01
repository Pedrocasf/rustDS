use super::*;

#[derive(Default)]
pub struct LCDC;

impl LCDC{
    pub const WIDTH:usize = 256;
    pub const WORD_WIDTH:usize = 128;
    pub const HEIGHT:usize = 192;
    
    const PIXELS: VolBlock<Bgr555, 0xC000> = unsafe { VolBlock::new(0x06800000) };
    pub const WORDS: VolBlock<u32, 0x20000> = unsafe { VolBlock::new(0x06800000) };

    pub fn read(col: usize, row: usize) -> Option<Bgr555> {
      Self::PIXELS.get(col + row * Self::WIDTH).map(VolAddress::read)
    }
    pub fn write(col: usize, row: usize, color: Bgr555) -> Option<()> {
      Self::PIXELS.get(col + row * Self::WIDTH).map(|va| va.write(color))
    }
    pub fn write_word(col: usize, row: usize, colors: u32) -> Option<()> {
      Self::WORDS.get(col + row * Self::WORD_WIDTH)
      .map(|va| va.write(colors))
    }
}
use embedded_graphics_core::{
  draw_target::DrawTarget,
  geometry::{
    OriginDimensions,
    Size,
  },
  pixelcolor::{
    Bgr555,
  },
  Pixel
};
impl DrawTarget for LCDC{
    type Color = Bgr555;
    type Error = core::convert::Infallible;
    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        for Pixel(coord, color) in pixels.into_iter() {
            LCDC::write(coord.x as usize, coord.y as usize, color);
        }
        Ok(())
    }
}
impl OriginDimensions for LCDC {
  fn size(&self) -> Size {
      Size::new(LCDC::WIDTH as u32, LCDC::HEIGHT as u32)
  }
}