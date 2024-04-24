pub const SCREEN_HEIGHT:i32 = 1000;
pub const SCREEN_WIDTH:i32 = 1000;
pub const MIN_AREA:i32 = 20*20*2;
pub const BUILDING_MIN:i32= 200;
pub const BUILDING_MAX:i32 = 600;
pub const DEBUG_TIMING:bool = true;

pub struct Config{
    pub cell_size:f32,
    pub scale_size:f32, 
}
impl Config{
    fn scale_factor(&self)->f32{
        return self.cell_size/self.scale_size;
    }
    pub fn normalize(&self, a:i32)->i32{
        return ((a as f32 /self.scale_factor()).round() as i32  as f32 *self.scale_factor()) as i32;
    }
}
impl Clone for Config{
    fn clone(&self)->Self{
        return Config{cell_size:self.cell_size, scale_size:self.scale_size};
    }
}