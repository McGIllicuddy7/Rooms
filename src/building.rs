

use raylib::drawing::RaylibDrawHandle;

use crate::{config, room::{self, purge_not_on_top, TreeRoom}};
pub struct Building{
    floors: Vec<Vec<room::Room>>,

}
fn calc_depth(ground_floor_num:i32)->usize{
    let amnt = (ground_floor_num)  as f64;
    let tmp = amnt.log2();
    return (tmp as usize)+2;
}
pub fn generate_building(ground_floor_num:i32, num_floors:usize)->Building{
    if ground_floor_num<1{
        panic!("not enough rooms");
    }
    let max_depth = calc_depth(ground_floor_num);
    let mut out:Building = Building::new();
    let min = (ground_floor_num as f64 *0.9) as usize;
    let max = (ground_floor_num as f64 *1.2) as usize;
    let mrad = min*10;
    let maxrad = max*10;
    let mut prev :TreeRoom;
    loop {
        let mut tree= room::TreeRoom::new(config::BUILDING_MIN, config::BUILDING_MIN,config::BUILDING_MAX, config::BUILDING_MAX);
        tree.split(max_depth);
        tree.drop_random(0, mrad as i32, maxrad as i32);
        let floors = tree.flatten();
        if floors.len()>min && floors.len()<max{
            out.floors.push(floors);
            prev = tree.template();
            break;
        }
    }
    for i in 1..num_floors{
        let mut l = prev.clone();
        l.split_recurse(max_depth+1,0);
        prev = l.template();
        let mut tmp = l.flatten();
        tmp = purge_not_on_top(&tmp, &out.floors[i-1]);
        let mut tree= room::TreeRoom::new(config::BUILDING_MIN, config::BUILDING_MIN,config::BUILDING_MAX, config::BUILDING_MAX);
        tree.split(max_depth);
        tree.drop_random(0, mrad as i32, maxrad as i32);
        let mut tmp2 = tree.flatten();
        tmp2 = purge_not_on_top(&tmp2, &out.floors[i-1]);
        if tmp2.len()>tmp.len(){
            out.floors.push(tmp2);
        } else{
            out.floors.push(tmp);
        }

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
    pub fn _num_floors(&self)->usize{
        return self.floors.len();
    }
}