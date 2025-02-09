use embedded_graphics::pixelcolor::{PixelColor, raw::RawU16};
#[derive(Copy,Clone,PartialEq)]
pub struct ABGR1555(pub u16);
impl ABGR1555{
    pub const fn new(r:u16,g:u16,b:u16)->ABGR1555{
        ABGR1555(
            (1<<15)|(b<<10)|(g<<5)|r
        )
    }
}
impl PixelColor for ABGR1555{
    type Raw = RawU16;
}
pub const COLORS:[ABGR1555;125] = [
    ABGR1555::new(0,0,0),
    ABGR1555::new(8,0,0),
    ABGR1555::new(9,0,0),
    ABGR1555::new(11,0,0),
    ABGR1555::new(12,0,0),
    ABGR1555::new(14,0,0),
    ABGR1555::new(15,0,0),
    ABGR1555::new(17,0,0),
    ABGR1555::new(18,0,0),
    ABGR1555::new(20,0,0),
    ABGR1555::new(21,0,0),
    ABGR1555::new(23,0,0),
    ABGR1555::new(24,0,0),
    ABGR1555::new(26,0,0),
    ABGR1555::new(27,0,0),
    ABGR1555::new(29,0,0),
    ABGR1555::new(31,0,0),
    ABGR1555::new(8,8,0),
    ABGR1555::new(0,9,0),
    ABGR1555::new(11,10,0),
    ABGR1555::new(22,10,0),
    ABGR1555::new(0,11,0),
    ABGR1555::new(0,12,0),
    ABGR1555::new(25,12,0),
    ABGR1555::new(14,13,0),
    ABGR1555::new(0,14,0),
    ABGR1555::new(0,15,0),
    ABGR1555::new(31,15,0),
    ABGR1555::new(16,16,0),
    ABGR1555::new(0,17,0),
    ABGR1555::new(0,18,0),
    ABGR1555::new(19,19,0),
    ABGR1555::new(0,20,0),
    ABGR1555::new(0,21,0),
    ABGR1555::new(22,21,0),
    ABGR1555::new(0,23,0),
    ABGR1555::new(0,24,0),
    ABGR1555::new(25,24,0),
    ABGR1555::new(28,26,0),
    ABGR1555::new(0,27,0),
    ABGR1555::new(0,29,0),
    ABGR1555::new(31,30,0),
    ABGR1555::new(0,31,0),
    ABGR1555::new(4,3,1),
    ABGR1555::new(5,4,1),
    ABGR1555::new(6,5,2),
    ABGR1555::new(7,5,2),
    ABGR1555::new(3,3,3),
    ABGR1555::new(7,5,3),
    ABGR1555::new(8,6,3),
    ABGR1555::new(9,6,3),
    ABGR1555::new(9,7,3),
    ABGR1555::new(10,7,3),
    ABGR1555::new(4,4,4),
    ABGR1555::new(31,4,4),
    ABGR1555::new(11,8,4),
    ABGR1555::new(12,8,4),
    ABGR1555::new(13,9,4),
    ABGR1555::new(31,17,4),
    ABGR1555::new(31,30,4),
    ABGR1555::new(5,5,5),
    ABGR1555::new(14,9,5),
    ABGR1555::new(15,10,5),
    ABGR1555::new(16,10,5),
    ABGR1555::new(6,6,6),
    ABGR1555::new(17,10,6),
    ABGR1555::new(17,11,6),
    ABGR1555::new(18,11,6),
    ABGR1555::new(7,7,7),
    ABGR1555::new(18,11,7),
    ABGR1555::new(19,12,7),
    ABGR1555::new(0,0,8),
    ABGR1555::new(8,0,8),
    ABGR1555::new(8,8,8),
    ABGR1555::new(20,13,8),
    ABGR1555::new(0,0,9),
    ABGR1555::new(9,0,9),
    ABGR1555::new(9,9,9),
    ABGR1555::new(10,0,10),
    ABGR1555::new(10,10,10),
    ABGR1555::new(31,31,10),
    ABGR1555::new(0,0,11),
    ABGR1555::new(11,0,11),
    ABGR1555::new(11,11,11),
    ABGR1555::new(28,17,11),
    ABGR1555::new(0,0,12),
    ABGR1555::new(12,0,12),
    ABGR1555::new(12,12,12),
    ABGR1555::new(13,13,13),
    ABGR1555::new(0,0,14),
    ABGR1555::new(13,0,14),
    ABGR1555::new(14,14,14),
    ABGR1555::new(0,0,15),
    ABGR1555::new(15,15,15),
    ABGR1555::new(16,31,15),
    ABGR1555::new(16,0,16),
    ABGR1555::new(16,16,16),
    ABGR1555::new(0,0,17),
    ABGR1555::new(17,17,17),
    ABGR1555::new(0,0,18),
    ABGR1555::new(18,18,18),
    ABGR1555::new(19,0,19),
    ABGR1555::new(19,19,19),
    ABGR1555::new(31,31,19),
    ABGR1555::new(0,0,20),
    ABGR1555::new(20,20,20),
    ABGR1555::new(0,0,21),
    ABGR1555::new(21,21,21),
    ABGR1555::new(22,0,22),
    ABGR1555::new(22,22,22),
    ABGR1555::new(23,31,22),
    ABGR1555::new(31,31,22),
    ABGR1555::new(0,0,23),
    ABGR1555::new(23,23,23),
    ABGR1555::new(0,0,24),
    ABGR1555::new(24,24,24),
    ABGR1555::new(25,25,25),
    ABGR1555::new(0,0,26),
    ABGR1555::new(26,26,26),
    ABGR1555::new(26,31,26),
    ABGR1555::new(0,0,27),
    ABGR1555::new(27,27,27),
    ABGR1555::new(29,29,29),
    ABGR1555::new(8,8,31),
    ABGR1555::new(31,31,31),
];