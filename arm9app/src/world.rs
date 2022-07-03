use crate::vertices::{
    Vec3,TexCoords
};
const GRASS_POS:u8 = 1;
const STONE_POS:u8 = 5;
use fixed::types::I20F12;
use alloc::boxed::Box;
pub struct World{
    blocks:Box<[u8;2*1024*1024]>
}
impl World{
    pub fn new()->World{
        World{
            blocks:Box::new([0;2*1024*1024])
        }
    }

    pub fn init(&mut self){
        for x in -64..64{
            for z in -64..64{
                self.add_block(
                    Vec3(
                        I20F12::from_num(x),
                        I20F12::from_num(1),
                        I20F12::from_num(z)
                    ),
                    GRASS_POS
                );
                self.add_block(
                    Vec3(
                        I20F12::from_num(x),
                        I20F12::from_num(0),
                        I20F12::from_num(z)
                    ),
                    STONE_POS
                );
            }
        }
    }
    pub fn add_block(&mut self, pos:Vec3, tx:u8){
        let x:usize = pos.0.to_num::<usize>();
        let y:usize = pos.1.to_num::<usize>();
        let z:usize = pos.2.to_num::<usize>();
        let pos = x + (y<<7) + (z<<14);
        self.blocks[pos] = tx;
    }
}