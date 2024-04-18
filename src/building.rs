
use raylib::drawing::RaylibDrawHandle;
use raylib::prelude::Vector2;
use crate::{config, room::{self, purge_not_on_top, TreeRoom}};
use std:: time::Instant;
use std::thread;
pub struct Building{
    pub floors: Vec<Vec<room::Room>>,

}
pub fn _floor_center(floor:&Vec<room::Room>)->Vector2{
    let mut out = Vector2{x:0.0, y:0.0};
    for i in floor{
        for j in i.corners(){
            out = Vector2{x: out.x+j.x, y: out.y+j.y};
        }
    }
    return out.scale_by((floor.len()*4) as f32);
}
pub fn _floor_divergence(floor: &Vec<room::Room>)->f64{
    let mut max:f64 = 0.0;
    let v = _floor_center(floor);
    for i in floor{
        for j in i.corners(){
            let l = j.distance_to(v) as f64;
            if l>max{
                max = l;
            }
        }
    }
    return max;
}
pub fn _floor_radius(floor: &Vec<room::Room>)->f64{
    let mut max:f64 = 0.0;
    let c = ((config::BUILDING_MAX+config::BUILDING_MIN)/2) as f32;
    let v = Vector2{x: c, y: c};
    for i in floor{
        for j in i.corners(){
            let l = j.distance_to(v) as f64;
            if l>max{
                max = l;
            }
        }
    }
    return max;
}
fn floor_area(floor: &Vec<room::Room>)->f64{
    let mut out = 0.0;
    for r in floor{
        out += (r.height*r.width) as f64;
    }
    return out;
}
fn _calc_depth(ground_floor_num:i32)->usize{
    let amnt = (ground_floor_num)  as f64;
    let amnt2 = amnt*amnt;
    let tmp = amnt2.log2();
    return (tmp as f64+0.25).ceil() as usize+1;
}

fn comparitior_weight(a: &Vec<room::Room>)->f64{
    return floor_area(a)as f64-_floor_radius(&a) as f64;
}
fn comparitor(a: &Vec<room::Room>, b:&Vec<room::Room>)->bool{
    let wa = comparitior_weight(a);
    let wb = comparitior_weight(b);
    return wa>wb;
}
fn gen_frst_floor(max_depth:&usize, desired_num:i32)->(f64,TreeRoom){
    let mut root:TreeRoom = TreeRoom::new(1,1,10,10,);
    let mut weight:f64 = 0.0;
    let mut i =0;
    let mx = 3;
    loop{
        if i>mx{
            break;
        }
        let mut tree= room::TreeRoom::new(config::BUILDING_MIN, config::BUILDING_MIN,config::BUILDING_MAX, config::BUILDING_MAX);
        tree.split(*max_depth);
        tree.drop_to_number(desired_num);
        let floors = tree.flatten();
        let floors = room::purge_degenerates(&floors);
        let r = comparitior_weight(&floors);
        if r>weight{
            root = tree;
            weight = r;
        }
        i += 1;
    } 
    return (weight,root);
}
fn generate_floors(ground_floor_num:i32, num_floors:usize)->Building{
    if ground_floor_num<1{
        panic!("not enough rooms");
    }
    let max_depth = 16;
    let mut out:Building = Building::new();
    let mut prev :TreeRoom;
    let mut root:TreeRoom = room::TreeRoom::new(config::BUILDING_MIN, config::BUILDING_MIN,config::BUILDING_MAX, config::BUILDING_MAX);
    let mut weight:f64  = 0 as f64;
    let start = Instant::now();
    let mut threads = vec![];
    for _ in 0..10{
        let a = thread::spawn(move||(gen_frst_floor(&max_depth, ground_floor_num)));
        threads.push(a);
    }
    for a in threads{
        let t = a.join().unwrap();
        if t.0>weight{
            weight = t.0;
            root = t.1;
        }
    }
    out.floors.push(root.flatten());
    prev = root;
    println!("first floor done in {:#?}", Instant::now()-start);
    for i in 1..num_floors{
        let start = Instant::now();
        let mut l = prev.clone();
        l = l.template();
        l.split_recurse(max_depth,0);
        let mut tmp = l.flatten();
        tmp = purge_not_on_top(&tmp, &out.floors[i-1]); 
        prev = l.template();
        for _ in 0..5{
            loop{
            let mut tre0 = prev.clone();
            tre0 = tre0.template();
            tre0.split_recurse(max_depth, 0);
            let mut tmp1 = tre0.flatten();
            tmp1 = purge_not_on_top(&tmp1, &out.floors[i-1]);
            tmp1 = room::purge_degenerates(&tmp1);
            if comparitor(&tmp1, &tmp){
                tmp = tmp1;
                prev = tre0.template();
            }
            let mut tree= room::TreeRoom::new(config::BUILDING_MIN, config::BUILDING_MIN,config::BUILDING_MAX, config::BUILDING_MAX);
            tree.split(max_depth);
            let mut tmp2 = tree.flatten();
            tmp2 = purge_not_on_top(&tmp2, &out.floors[i-1]);
            tmp2 = room::purge_degenerates(&tmp2);
            if comparitor(&tmp2, &tmp){
                tmp = tmp2;
                prev = tree.template();
            }
            break;
            }
        }
        println!("floor {} finished in {:#?}",i+1,Instant::now()-start);
        out.floors.push(tmp);
    }
    return out;
}

pub fn generate_building(ground_floor_num:i32, num_floors:usize)->Building{
    let out = generate_floors(ground_floor_num, num_floors);
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
