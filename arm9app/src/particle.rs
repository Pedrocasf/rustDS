use crate::alloc::vec::Vec;
use arm9rt::{
    a::*,
};
use fixed::prelude::*;
use crate::sprites::*;
use fixed::types::I9F23;
use embedded_graphics_core::{
    Pixel,
    Drawable,
    pixelcolor::Bgr555,
  };
use embedded_graphics::{
    prelude::*,
};
use crate::{assets::*};
// rot speed = 0.05
const FLOOR:usize = 3*64*64;
const CEILING:usize = 6*64*64;
pub struct Particle{
    sprites:Sprites,
    P_SEN_ROT_SPEED:I9F23,
    P_COS_ROT_SPEED:I9F23,
    N_SEN_ROT_SPEED:I9F23,
    N_COS_ROT_SPEED:I9F23,
    MOV_SPEED:I9F23,
    pub posx:I9F23,
    pub posy:I9F23,
    pub planex:I9F23,
    pub planey:I9F23,
    pub dirx:I9F23,
    pub diry:I9F23,
    z_buffer:[I9F23;BG3::WIDTH],
    sprite_order:[usize;NUM_SPRITES],
    sprite_distance:[I9F23;NUM_SPRITES],
}
impl Particle{
    pub fn new()-> Particle{
        Particle{
            sprites:Sprites::new(),
            P_SEN_ROT_SPEED:I9F23::from_num(0.04997916927f32),
            P_COS_ROT_SPEED:I9F23::from_num(0.99875026039f32),
            N_SEN_ROT_SPEED:I9F23::from_num(-0.04997916927f32),
            N_COS_ROT_SPEED:I9F23::from_num(0.99875026039f32),
            MOV_SPEED:I9F23::from_num(0.084f32),
            posx:I9F23::from_num(22.0),
            posy:I9F23::from_num(11.5),
            planex:I9F23::from_num(0.0),
            planey:I9F23::from_num(0.66),
            dirx:I9F23::from_num(-1.0),
            diry:I9F23::from_num(0.0),
            z_buffer:[I9F23::from_num(0.0);BG3::WIDTH],
            sprite_order:[0;NUM_SPRITES],
            sprite_distance:[I9F23::from_num(0.0);NUM_SPRITES],
        }
    }
    pub fn forward(&mut self){
        if WORLD_MAP[(self.posx+self.dirx*self.MOV_SPEED).to_num::<usize>()][self.posy.to_num::<usize>()] == 0{self.posx+=self.dirx*self.MOV_SPEED};
        if WORLD_MAP[self.posx.to_num::<usize>()][(self.posy+self.diry*self.MOV_SPEED).to_num::<usize>()] == 0{self.posy+=self.diry*self.MOV_SPEED};
    }
    pub fn backwards(&mut self){
        if WORLD_MAP[(self.posx-self.dirx*self.MOV_SPEED).to_num::<usize>()][self.posy.to_num::<usize>()]==0{self.posx-=self.dirx*self.MOV_SPEED};
        if WORLD_MAP[self.posx.to_num::<usize>()][(self.posy-self.diry*self.MOV_SPEED).to_num::<usize>()]==0{self.posy-=self.diry*self.MOV_SPEED};
    }
    pub fn right(&mut self){
        let old_dir_x = self.dirx;
        self.dirx = self.dirx * self.P_COS_ROT_SPEED - self.diry * self.N_SEN_ROT_SPEED;
        self.diry = old_dir_x * self.N_SEN_ROT_SPEED + self.diry * self.P_COS_ROT_SPEED;
        let old_plane_x = self.planex;
        self.planex = self.planex * self.P_COS_ROT_SPEED - self.planey * self.N_SEN_ROT_SPEED;
        self.planey = old_plane_x * self.N_SEN_ROT_SPEED + self.planey * self.P_COS_ROT_SPEED;
    }
    pub fn left(&mut self){
        let old_dir_x = self.dirx;
        self.dirx = self.dirx * self.P_COS_ROT_SPEED - self.diry * self.P_SEN_ROT_SPEED;
        self.diry = old_dir_x * self.P_SEN_ROT_SPEED + self.diry * self.P_COS_ROT_SPEED;
        let old_plane_x = self.planex;
        self.planex = self.planex * self.P_COS_ROT_SPEED - self.planey * self.P_SEN_ROT_SPEED;
        self.planey = old_plane_x * self.P_SEN_ROT_SPEED + self.planey * self.P_COS_ROT_SPEED;
    }
    pub fn draw_floor_ceiling(&self, display: &mut BG3, textures:&[u8]){
        for y in BG3::HEIGHT>>1..BG3::HEIGHT{
            let ray_dir_x0 = self.dirx - self.planex;
            let ray_dir_y0 = self.diry - self.planey;
            let ray_dir_x1 = self.dirx + self.planex;
            let ray_dir_y1 = self.diry + self.planey;
            let p = y - (BG3::HEIGHT >> 1);
            let pos_z:I9F23 = BG2::FIX_HEIGHT >> 1;
            let row_distance = pos_z.checked_div(I9F23::from_num(p)).unwrap_or(I9F23::from_num(255));
            let floor_step_x = row_distance * (ray_dir_x1 - ray_dir_x0)>>8;
            let floor_step_y = row_distance * (ray_dir_y1 - ray_dir_y0)>>8;
            let mut floor_x = self.posx + row_distance * ray_dir_x0;
            let mut floor_y = self.posy + row_distance * ray_dir_y0;
            for x in 0..BG3::WORD_WIDTH{
                let mut bytesf = [0;4]; 
                let mut bytesc = [0;4]; 
                for c in 0..4{
                    let cell_x = floor_x.to_num::<i32>();
                    let cell_y = floor_y.to_num::<i32>();
                    let tx = (I9F23::from_num(TEXTURE_WIDTH)  * (floor_x - I9F23::from_num(cell_x))).to_num::<i32>() & (TEXTURE_WIDTH as i32 - 1);
                    let ty = (I9F23::from_num(TEXTURE_HEIGHT) * (floor_y - I9F23::from_num(cell_y))).to_num::<i32>() & (TEXTURE_HEIGHT as i32 - 1);
                    floor_x += floor_step_x;
                    floor_y += floor_step_y;
                    let tp = ty as usize + (TEXTURE_WIDTH * tx as usize);
                    let c0f = textures[FLOOR + tp];
                    let c0c = textures[CEILING + tp];
                    bytesc[c] = c0c;
                    bytesf[c] = c0f;
                }
                let colorf = U32(u32::from_ne_bytes(bytesf));
                Pixel(Point::new(x as i32,y as i32), colorf).draw(display).unwrap();
                let colorc = U32(u32::from_ne_bytes(bytesc));
                Pixel(Point::new(x as i32,BG3::HEIGHT as i32 - y as i32 - 1), colorc).draw(display).unwrap();
            }
        }
    }
    pub fn draw_walls(&mut self, display:&mut BG2, textures:&[u8]){
        for x in 0..BG2::WORD_WIDTH{
            let mut arr:[[u8;4];192] = [[0;4];192];
            for xi in 0..4{
                let camera_x =(I9F23::from_num((x << 2)+xi) >> 7) -I9F23::from_num(1.0); 
                let ray_dir_x = self.dirx + self.planex * camera_x;
                let ray_dir_y = self.diry + self.planey * camera_x;

                let mut map_x = self.posx.to_num::<isize>();
                let mut map_y = self.posy.to_num::<isize>();

                let mut side_dsit_x:I9F23;
                let mut side_dsit_y:I9F23;

                let delta_dist_x =
                    if ray_dir_y == I9F23::from_bits(0){
                        I9F23::from_bits(0)
                    }else{
                        (I9F23::from_num(1.0).checked_div(ray_dir_x).unwrap_or(I9F23::from_num(1.0))).abs()
                    };
                let delta_dist_y = 
                    if ray_dir_x == I9F23::from_bits(0){
                        I9F23::from_bits(0)
                    }else{
                        (I9F23::from_num(1.0).checked_div(ray_dir_y).unwrap_or(I9F23::from_num(1.0))).abs()
                    };

                let perp_wall_dist:I9F23;

                let step_x:isize;
                let step_y:isize;

                let mut hit:bool = false;
                let mut side:bool = false;

                if ray_dir_x < 0.0{
                    step_x = -1;
                    side_dsit_x = (self.posx - I9F23::from_num(map_x)) * delta_dist_x;
                }else{
                    step_x = 1;
                    side_dsit_x = (I9F23::from_num(map_x + 1) - self.posx) * delta_dist_x;
                }
                if ray_dir_y < 0.0{
                    step_y = -1;
                    side_dsit_y = (self.posy - I9F23::from_num(map_y)) * delta_dist_y;
                }else{
                    step_y = 1;
                    side_dsit_y = (I9F23::from_num(map_y + 1) - self.posy) * delta_dist_y;
                }
                while !hit {
                    if side_dsit_x < side_dsit_y{
                        side_dsit_x += delta_dist_x;
                        map_x += step_x;
                        side = false;
                    }else{
                        side_dsit_y+=delta_dist_y;
                        map_y += step_y;
                        side = true;
                    }
                    hit = WORLD_MAP[map_x as usize][map_y as usize] != 0;
                }
                if !side{
                    perp_wall_dist = (I9F23::from_num(map_x) - self.posx + ((I9F23::from_num(1.0)-I9F23::from_num(step_x))/I9F23::from_num(2)))/ray_dir_x
                } else {
                    perp_wall_dist = (I9F23::from_num(map_y) - self.posy + ((I9F23::from_num(1.0)-I9F23::from_num(step_y))/I9F23::from_num(2)))/ray_dir_y
                }
                
                let line_height:I9F23 = BG2::FIX_HEIGHT.checked_div(perp_wall_dist).unwrap_or(BG2::FIX_HEIGHT);

                let mut draw_start = I9F23::from_num((line_height >> 1)* I9F23::from_num(-1.0)) + (BG2::FIX_HEIGHT >> 1);
                if draw_start < 0 {draw_start = I9F23::from_bits(0)};
                let mut draw_end = I9F23::from_num(line_height>>1) + (BG2::FIX_HEIGHT >>1);
                if draw_end > BG2::FIX_HEIGHT{draw_end = BG2::FIX_HEIGHT - I9F23::from_num(1)};

                let tex_num = WORLD_MAP[map_x as usize][map_y as usize] -1;

                let mut wall_x:I9F23;
                if !side{wall_x=self.posy+(perp_wall_dist*ray_dir_y)}
                else{wall_x=self.posx+(perp_wall_dist*ray_dir_x)};
                wall_x -= wall_x.floor();

                let mut tex_x = (wall_x * I9F23::from_num(TEXTURE_WIDTH)).to_num::<i32>();
                if !side && (ray_dir_x > 0.0) {tex_x = TEXTURE_WIDTH as i32 - tex_x - 1};
                if side && (ray_dir_y < 0.0) {tex_x = TEXTURE_WIDTH as i32 - tex_x - 1};

                let step = I9F23::from_num(1.0)*I9F23::from_num(TEXTURE_HEIGHT).checked_div(line_height).unwrap_or(BG2::FIX_HEIGHT);
                let mut tex_pos:I9F23 = (draw_start - (BG2::FIX_HEIGHT>>1) +(line_height >>1)) * step;
                for y in draw_start.to_num::<usize>()..draw_end.to_num::<usize>(){
                    let tex_y = tex_pos.to_num::<i32>() & (TEXTURE_HEIGHT - 1) as i32;
                    tex_pos += step;
                    let c = textures[(tex_num as usize*TEXTURE_HEIGHT*TEXTURE_WIDTH)+((TEXTURE_HEIGHT*tex_y as usize)+tex_x as usize)];
                    arr[y][xi] = c;
                }
                self.z_buffer[((x as usize)<<2) + xi] = perp_wall_dist;
            } 
            for (y,w) in arr.iter().enumerate(){
                let color = U32(u32::from_le_bytes(*w));
                Pixel(Point::new(x as i32, y as i32), color).draw(display).unwrap();
            }
        }
    }
    pub fn draw_sprites(&mut self, display:&mut BG3, textures:&[u8]){
        for i in 0..NUM_SPRITES{
            self.sprite_order[i] = i;
            self.sprite_distance[i] = ((self.posx - self.sprites.0[i].x) * (self.posx - self.sprites.0[i].x))  + ((self.posy - self.sprites.0[i].y) * (self.posy - self.sprites.0[i].y));
        }
        let mut sprites:[(I9F23,usize);NUM_SPRITES] = [(I9F23::from_num(0.0),0);NUM_SPRITES];
        for i in 0..NUM_SPRITES{
            sprites[i] = (self.sprite_distance[i],self.sprite_order[i]);
        }
        sprites.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());
        for i in 0..NUM_SPRITES{
            self.sprite_distance[i] = sprites[NUM_SPRITES -i -1].0;
            self.sprite_order[i] = sprites[NUM_SPRITES -i -1].1;
        }
        for i in 0..NUM_SPRITES{
            let sprite_x = self.sprites.0[self.sprite_order[i]].x - self.posx;
            let sprite_y = self.sprites.0[self.sprite_order[i]].y - self.posy;

            let inv_det = I9F23::from_num(1.0) / ((self.planex * self.diry) - (self.dirx * self.planey));

            let transform_x = inv_det * (self.diry * sprite_x - self.dirx * sprite_y);
            let transform_y = inv_det * (-self.planey * sprite_x + self.planex * sprite_y);

            let sprite_screen_x = (I9F23::from_num(BG3::WIDTH / 2) * (I9F23::from_num(1.0) + transform_x / transform_y)).to_num::<isize>();    
            
            let v_move_screen = (self.sprites.0[self.sprite_order[i]].v_mov / transform_y).to_num::<isize>();

            let sprite_height = ((I9F23::from_num(BG3::HEIGHT) / transform_y).to_num::<isize>() / self.sprites.0[self.sprite_order[i]].v_div as isize).abs();
            let mut draw_start_y = -sprite_height / 2 + (BG3::HEIGHT/2) as isize + v_move_screen; 
            if draw_start_y < 0 {draw_start_y = 0};
            let mut draw_end_y = sprite_height / 2 + (BG3::HEIGHT/2) as isize + v_move_screen;
            if draw_end_y >= BG3::HEIGHT as isize {draw_end_y = BG3::HEIGHT as isize- 1};

            let sprite_width = ((I9F23::from_num(BG3::HEIGHT) / transform_y).to_num::<isize>() / self.sprites.0[self.sprite_order[i]].u_div as isize).abs();
            let mut draw_start_x = -sprite_width / 2 + sprite_screen_x; 
            if draw_start_x < 0 {draw_start_x = 0};
            let mut draw_end_x = sprite_width / 2 + sprite_screen_x;
            if draw_end_x >= BG3::WIDTH as isize {draw_end_x = BG3::WIDTH as isize- 1};

            for stripe in draw_start_x..draw_end_x{

                let tex_x = (256 * (stripe - (-sprite_width / 2 + sprite_screen_x)) * TEXTURE_WIDTH as isize / sprite_width) / 256 as isize;
                
                if transform_y > 0.0 && stripe > 0 && stripe < BG3::WIDTH as isize && transform_y < self.z_buffer[stripe as usize]{
                    for y in (draw_start_y..draw_end_y).step_by(4){
                        let mut c = [0;4]; 
                        for dy in 0..4{
                            let d = (y- v_move_screen) * 256 - BG3::HEIGHT as isize * 128 + sprite_height * 128;
                            let tex_y = ((d * TEXTURE_HEIGHT as isize)/ sprite_height) / 256;
                            let color = textures[(self.sprites.0[self.sprite_order[i]].texture as usize *64*64) + (TEXTURE_WIDTH * tex_y as usize) + tex_x as usize];
                            c[dy] = color;
                        }
                        Pixel(Point::new(stripe as i32,y as i32), U32(u32::from_le_bytes(c))).draw(display).unwrap();
                    }
                }
            }
        }
    }
}