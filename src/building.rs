

use raylib::drawing::RaylibDrawHandle;

use crate::room;
pub struct Building{
    floors: Vec<Vec<room::Room>>,

}
pub fn generate_building(ground_floor_num:i32, num_floors:usize)->Building{
    let mut out:Building = Building::new();
    let min = (ground_floor_num as f64 *0.9) as usize;
    let max = (ground_floor_num as f64 *1.1) as usize;
    let mrad = min*10;
    let maxrad = max*10;
    loop {
        let floors = room::new_building(mrad, maxrad);
        if floors.len()>min && floors.len()<max{
            out.floors.push(floors);
            break;
        }
    }
    for i in 1..num_floors{
        let tmp = room::new_floor(&out.floors[i-1]);
        out.floors.push(tmp);
    }
    return out;
}
impl Building{
    pub fn new()->Self{
        return Self{floors:Vec::new()};
    }
    pub fn render_floor(&self, floor:usize, handle:&mut RaylibDrawHandle){
        if floor> self.floors.len(){
            return;
        }
        room::render_rooms(&self.floors[floor], handle)
    }
    pub fn num_floors(&self)->usize{
        return self.floors.len();
    }
}