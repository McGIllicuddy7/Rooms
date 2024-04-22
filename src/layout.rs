use crate::{room::Room, utils};
use raylib::prelude::Vector2;
#[derive(Debug)]
pub enum Direction {
    Top, Bottom, Left, Right,
}
pub struct Portal{
    pub idx1:i32, 
    pub idx2:i32,
    pub location:Vector2,
    pub dir:Direction,
}
fn generate_midpoint(amin:i32, amax:i32, bmin:i32, bmax:i32)->Option<i32>{
    if !inside(amin, amax,amax, bmin){
        return None;
    }
    let amn:i32;
    let bmn:i32;
    let amx:i32;
    let bmx:i32;
    if amin<bmin{
        amn = bmin;
        amx = bmax;
        bmn = amin;
        bmx = amax;
    } else{
        amn = amin;
        amx = amax;
        bmn = bmin;
        bmx = bmax;
    }
    if amx<=bmn{
        return None;
    }
    if amx<=bmx{
        return Some((amn+amx)/2);
    } else{
        if bmx>amax{
            return Some((bmn+amx)/2);
        }
        else{
            return Some((bmn+bmx)/2);
        }
    }
}
impl Portal{
    //direction from idx1 to idx2
    pub fn link(floor: &Vec<Room>,idx1:i32, idx2:i32 , dir:Direction)->Option<Portal>{
        //println!("{:?}", dir);
        let location:Vector2;
        let r1 = floor[idx1 as usize].clone();
        let r2 = floor[idx2 as usize].clone();
        match dir {
            Direction::Top=>{   
                if let Some(mid) = generate_midpoint(r1.x, r1.x+r1.width, r2.x, r2.x+r2.width){
                    location = Vector2{x:mid as f32,y:(r1.y) as f32};
                }
                else{
                    return None;
                }
            }
            Direction::Bottom=>{
                if let Some(mid) = generate_midpoint(r1.x, r1.x+r1.width, r2.x, r2.x+r2.width){
                    location = Vector2{x:mid as f32,y:(r1.y+r1.height) as f32};
                }
                else{
                    return None;
                }
            }
            Direction::Right=>{
                if let Some(mid) = generate_midpoint(r1.y, r1.y+r1.height, r2.y, r2.y+r2.height){
                    location = Vector2{x:(r1.x+r1.width) as f32,y: mid as f32};
                }
                else{
                    return None;
                }
            }
            Direction::Left=>{
                if let Some(mid) = generate_midpoint(r1.y, r1.y+r1.height, r2.y, r2.y+r2.height){
                    location = Vector2{x:(r1.x) as f32,y: mid as f32};
                }
                else{
                    return None;
                }
            }
        }
        return Some( Portal { idx1: idx2, idx2: idx1, location, dir});
    }
}
fn near(a:i32, b:i32)->bool{
    return a == b || a+1 == b || a-1 == b;
}
fn inside(ab:i32,ae:i32, bb:i32,be:i32)->bool{
    return (ab >=bb && ab<=be ||ae>=bb && ae<=be) || (bb >=ab && bb<=ae ||be>=ab && be<=ae);
}
fn shared_border(r0:&Room, r1:&Room)->Option<Direction>{
    if near(r0.x, r1.x+r1.width){
        if inside(r0.y, r0.y+r0.height, r1.y, r1.y+r1.height){
            return Some(Direction::Left);
        }
    }
    if near(r1.x, r0.x+r0.width){
        if inside(r0.y, r0.y+r0.height, r1.y, r1.y+r1.height){
            return Some(Direction::Right);
        }
    }
    if near(r0.y, r1.y+r1.height){
        if inside(r0.x, r0.x+r0.width, r1.x, r1.x+r1.width){
            return Some(Direction::Top);
        }
    }
    if near(r1.y, r0.y+r0.height){
        if inside(r1.x, r1.x+r1.width, r0.x, r0.x+r0.width){
            return Some(Direction::Bottom);
        }
    }
    return None;
}
pub fn calc_doors(floor:&Vec<Room>)->Vec<Portal> {
    let mut out = vec![];
    let mut reached = vec![];
    for _ in 0..floor.len(){
        reached.push(false);
    }
    for i in 0..floor.len(){
        for j in i+1..floor.len(){
            if reached[j] && utils::random()%3 != 0{
                continue;
            }
            let shared = shared_border(&floor[i], &floor[j]);
            if shared.is_some(){
                let dir = shared.unwrap();
                let portal = Portal::link(floor, i as i32,j as i32, dir);
                if portal.is_some(){
                    out.push(portal.unwrap());
                }
                reached[j] = true;
                reached[i] = true;
            }
        }
    }
    return out;
}