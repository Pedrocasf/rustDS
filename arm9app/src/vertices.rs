use fixed::types::I20F12;
use core::ops::Mul;
pub struct Cube(
    [I20F12;72]
);
impl Cube{
    pub fn cube_vertices(x:I20F12,y:I20F12,z:I20F12,n:I20F12)->Cube{
        Cube([
            x-n,y+n,z-n, x-n,y+n,z+n, x+n,y+n,z+n, x+n,y+n,z-n,  //top
            x-n,y-n,z-n, x+n,y-n,z-n, x+n,y-n,z+n, x-n,y-n,z+n,  //bottom
            x-n,y-n,z-n, x-n,y-n,z+n, x-n,y+n,z+n, x-n,y+n,z-n,  //left
            x+n,y-n,z+n, x+n,y-n,z-n, x+n,y+n,z-n, x+n,y+n,z+n,  //right
            x-n,y-n,z+n, x+n,y-n,z+n, x+n,y+n,z+n, x-n,y+n,z+n,  //front
            x+n,y-n,z-n, x-n,y-n,z-n, x-n,y+n,z-n, x+n,y+n,z-n   //back
        ])
    }
}
pub struct TexCoord(
    [i32;8]
);
impl TexCoord{
    pub const fn tex_coord(x:i32,y:i32)->TexCoord{
        let m = 16;
        let dx = x <<4;
        let dy = y <<4;
        TexCoord([
            dx, dy, dx + m, dy, dx + m, dy + m, dx, dy + m
        ])
    }
}
pub struct Vec2i(
    pub i32,
    pub i32
);
pub struct Vec3i(
    pub i32,
    pub i32,
    pub i32
);
pub struct Vec2(
    pub I20F12,
    pub I20F12
);
pub struct Vec3(
    pub I20F12,
    pub I20F12,
    pub I20F12
);
const SECTOR_SIZE:I20F12 = I20F12::from_bits(16<<12);
impl Vec3{
    pub fn normalize(&self)->Vec3{
        Vec3(
            self.0.floor(),
            self.1.floor(),
            self.2.floor(),
        )
    }
    pub fn sectorize(&self)->Vec3{
        let v = self.normalize();
        let v = Vec3(
            v.0 >>4,
            v.1 >>4,
            v.2 >>4
        );
        v.normalize()
    }
}
pub struct TexCoords(
    [TexCoord;3]
);
impl TexCoords{
    pub const fn tex_coords(top:Vec2i,bottom:Vec2i,side:Vec2i)->TexCoords{
        TexCoords([
            TexCoord::tex_coord(top.0,top.1),
            TexCoord::tex_coord(bottom.0,bottom.1),
            TexCoord::tex_coord(side.0,side.1),
        ])
    }
}
//const TEXTURE:&[u8] = include_bytes!("../../eldpack/terrain.raw");
const GRASS:TexCoords = TexCoords::tex_coords(
    Vec2i(
        1,
        0,
    ),
    Vec2i(
        0,
        1,
    ),
    Vec2i(
        0,
        0,
    )
);
const SAND:TexCoords = TexCoords::tex_coords(
    Vec2i(
        1,
        1,
    ),
    Vec2i(
        1,
        1,
    ),
    Vec2i(
        1,
        1,
    )
);

const BRICK:TexCoords = TexCoords::tex_coords(
    Vec2i(
        2,
        0,
    ),
    Vec2i(
        2,
        0,
    ),
    Vec2i(
        2,
        0,
    )
);

const STONE:TexCoords = TexCoords::tex_coords(
    Vec2i(
        2,
        1,
    ),
    Vec2i(
        2,
        1,
    ),
    Vec2i(
        2,
        1,
    )
);
const TEXTURES_POS:[Option<TexCoords>;5] = [
    None,Some(GRASS),Some(SAND),Some(BRICK),Some(STONE)
];
const FACES:[Vec3i;6] = [
    Vec3i(
       0,
       1,
       0
    ),
    Vec3i(
        0,
        -1,
        0
    ),
    Vec3i(
        -1,
        0,
        0
    ),
    Vec3i(
        1,
        0,
        0
    ),
    Vec3i(
        0,
        0,
        1
    ),
    Vec3i(
        0,
        0,
        -1
    ),
];