use fixed::types::I9F23;
pub const NUM_SPRITES:usize = 19;
pub struct Sprites(pub [Sprite;NUM_SPRITES]);
impl Sprites{
    pub fn new()-> Sprites{
        Sprites(
            [
                Sprite::new(I9F23::from_num(20.5f32),I9F23::from_num(11.5f32),10, 1, 1, I9F23::from_num(0.0f32)),
                Sprite::new(I9F23::from_num(18.5f32),I9F23::from_num(4.5f32),10, 1, 1, I9F23::from_num(0.0f32)),
                Sprite::new(I9F23::from_num(10.0f32),I9F23::from_num(4.5f32),10, 1, 1, I9F23::from_num(0.0f32)),
                Sprite::new(I9F23::from_num(10.0f32),I9F23::from_num(18.5f32),10, 1, 1, I9F23::from_num(0.0f32)),
                Sprite::new(I9F23::from_num(3.5f32),I9F23::from_num(6.5f32),10, 1, 1, I9F23::from_num(0.0f32)),
                Sprite::new(I9F23::from_num(3.5f32),I9F23::from_num(20.5f32),10, 1, 1, I9F23::from_num(0.0f32)),
                Sprite::new(I9F23::from_num(3.5f32),I9F23::from_num(14.5f32),10, 1, 1, I9F23::from_num(0.0f32)),
                Sprite::new(I9F23::from_num(14.5f32),I9F23::from_num(20.5f32),10, 1, 1, I9F23::from_num(0.0f32)),
                Sprite::new(I9F23::from_num(18.5f32),I9F23::from_num(10.5f32),9, 1, 1, I9F23::from_num(0.0f32)),
                Sprite::new(I9F23::from_num(18.5f32),I9F23::from_num(11.5f32),9, 1, 1, I9F23::from_num(0.0f32)),
                Sprite::new(I9F23::from_num(18.5f32),I9F23::from_num(12.5f32),9, 1, 1, I9F23::from_num(0.0f32)),
                Sprite::new(I9F23::from_num(21.5f32),I9F23::from_num(1.5f32),8, 2, 2, I9F23::from_num(64.0f32)),
                Sprite::new(I9F23::from_num(15.5f32),I9F23::from_num(1.5f32),8, 2, 2, I9F23::from_num(64.0f32)),
                Sprite::new(I9F23::from_num(16.0f32),I9F23::from_num(1.8f32),8, 2, 2, I9F23::from_num(64.0f32)),
                Sprite::new(I9F23::from_num(16.2f32),I9F23::from_num(1.2f32),8, 2, 2, I9F23::from_num(64.0f32)),
                Sprite::new(I9F23::from_num(3.5f32),I9F23::from_num(2.5f32),8, 2, 2, I9F23::from_num(64.0f32)),
                Sprite::new(I9F23::from_num(9.5f32),I9F23::from_num(15.5f32),8, 2, 2, I9F23::from_num(64.0f32)),
                Sprite::new(I9F23::from_num(10.0f32),I9F23::from_num(15.1f32),8, 2, 2, I9F23::from_num(64.0f32)),
                Sprite::new(I9F23::from_num(10.5f32),I9F23::from_num(15.8f32),8, 2, 2, I9F23::from_num(64.0f32)),
            ]           
        )
    }
}
pub struct Sprite{
    pub x:I9F23,
    pub y:I9F23,
    pub texture:u8,
    pub u_div:u8,
    pub v_div:u8,
    pub v_mov:I9F23
}
impl Sprite{
    pub const fn new(x:I9F23,y:I9F23,t:u8, u:u8, v:u8, m:I9F23) -> Sprite{
        Sprite{
            x:x,
            y:y,
            texture:t,
            u_div:u,
            v_div:v,
            v_mov:m
        }
    }
}