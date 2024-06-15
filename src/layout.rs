use crate::room;
use crate::utils::Vector2;
use crate::{room::Room, utils};
use nalgebra_glm::*;
#[derive(Debug)]
pub enum Direction {
    Top,
    Bottom,
    Left,
    Right,
}
pub struct Portal {
    pub idx1: i32,
    pub idx2: i32,
    pub location: Vector2,
    pub dir: Direction,
}
pub struct Stair {
    pub top: i32,
    pub bot: i32,
    pub location: Vector2,
}
fn generate_midpoint(amin: i32, amax: i32, bmin: i32, bmax: i32) -> Option<i32> {
    // assert!(amin<amax);
    //assert!(bmin<bmax);
    if !inside(amin, amax, amax, bmin) {
        return None;
    }
    let lmin: i32;
    let lmax: i32;
    let smin: i32;
    let smax: i32;
    if (amax - amin) > (bmax - bmin) {
        lmin = amin;
        lmax = amax;
        smin = bmin;
        smax = bmax;
    } else {
        lmin = bmin;
        lmax = bmax;
        smin = amin;
        smax = amax;
    }
    let mut min_inter: i32 = -1;
    let mut max_inter: i32 = -1;
    let mut current = lmin;
    while current <= lmax {
        if current >= smin && current <= smax {
            if min_inter == -1 {
                min_inter = current;
            }
            max_inter = current;
        }
        current += 1;
    }
    if min_inter == -1 {
        return None;
    }
    if (max_inter - min_inter) < 4 {
        return None;
    }
    return Some((max_inter + min_inter) / 2);
}
impl Portal {
    //direction from idx1 to idx2
    pub fn link(floor: &Vec<Room>, idx1: i32, idx2: i32, dir: Direction) -> Option<Portal> {
        //println!("{:?}", dir);
        let location: Vector2;
        let r1 = floor[idx1 as usize].clone();
        let r2 = floor[idx2 as usize].clone();
        match dir {
            Direction::Top => {
                if let Some(mid) = generate_midpoint(r1.x, r1.x + r1.width, r2.x, r2.x + r2.width) {
                    location = vec2(mid as f32, (r1.y) as f32)
                } else {
                    return None;
                }
            }
            Direction::Bottom => {
                if let Some(mid) = generate_midpoint(r1.x, r1.x + r1.width, r2.x, r2.x + r2.width) {
                    location = vec2(mid as f32, (r1.y + r1.height) as f32);
                } else {
                    return None;
                }
            }
            Direction::Right => {
                if let Some(mid) = generate_midpoint(r1.y, r1.y + r1.height, r2.y, r2.y + r2.height)
                {
                    location = vec2((r2.x) as f32, mid as f32);
                } else {
                    return None;
                }
            }
            Direction::Left => {
                if let Some(mid) = generate_midpoint(r1.y, r1.y + r1.height, r2.y, r2.y + r2.height)
                {
                    location = vec2(r1.x as f32, mid as f32);
                } else {
                    return None;
                }
            }
        }
        return Some(Portal {
            idx1: idx2,
            idx2: idx1,
            location,
            dir,
        });
    }
}
fn near(a: i32, b: i32) -> bool {
    return a == b || a + 1 == b || a - 1 == b;
}
fn inside(ab: i32, ae: i32, bb: i32, be: i32) -> bool {
    return (ab >= bb && ab <= be || ae >= bb && ae <= be) || (ae >= be && ab <= bb);
}
fn shared_border(r0: &Room, r1: &Room) -> Option<Direction> {
    if near(r0.x, r1.x + r1.width) {
        if inside(r0.y, r0.y + r0.height, r1.y, r1.y + r1.height) {
            return Some(Direction::Left);
        }
    }
    if near(r1.x, r0.x + r0.width) {
        if inside(r0.y, r0.y + r0.height, r1.y, r1.y + r1.height) {
            return Some(Direction::Right);
        }
    }
    if near(r0.y, r1.y + r1.height) {
        if inside(r0.x, r0.x + r0.width, r1.x, r1.x + r1.width) {
            return Some(Direction::Top);
        }
    }
    if near(r1.y, r0.y + r0.height) {
        if inside(r1.x, r1.x + r1.width, r0.x, r0.x + r0.width) {
            return Some(Direction::Bottom);
        }
    }
    return None;
}
fn calc_doors_floor(floor: &Vec<Room>, first_floor: bool) -> Vec<Portal> {
    let mut out = vec![];
    let mut reached = vec![];
    for _ in 0..floor.len() {
        reached.push(false);
    }
    let mut potential_tops = vec![];
    let mut potential_bots = vec![];
    let mut potential_lefts = vec![];
    let mut potential_rights = vec![];
    for i in 0..floor.len() {
        let mut top = false;
        let mut bot = false;
        let mut left = false;
        let mut right = false;
        for j in 0..floor.len() {
            if i == j {
                continue;
            }
            let shared = shared_border(&floor[i], &floor[j]);
            if shared.is_some() {
                let dir = shared.unwrap();
                match dir {
                    Direction::Top => {
                        top = true;
                    }
                    Direction::Bottom => {
                        bot = true;
                    }
                    Direction::Left => {
                        left = true;
                    }
                    Direction::Right => {
                        right = true;
                    }
                }
                if reached[i] && reached[j] && utils::random() % 3 == 0 {
                    continue;
                }
                let portal = Portal::link(floor, i as i32, j as i32, dir);
                if portal.is_some() {
                    out.push(portal.unwrap());

                    reached[i] = true;
                    reached[j] = true;
                }
            }
        }
        if !top {
            potential_tops.push(i);
        }
        if !bot {
            potential_bots.push(i);
        }
        if !left {
            potential_lefts.push(i);
        }
        if !right {
            potential_rights.push(i);
        }
    }
    if !first_floor {
        return out;
    }
    let mut count = 0;
    let max_count = floor.len();
    loop {
        let mut idx = (utils::random() % 4) as i32;
        let mut dir: Direction = Direction::Top;
        let mut room_idx: usize = 0;
        if idx == 0 {
            if potential_tops.len() == 0 {
                idx += 1;
            }
            dir = Direction::Top;
            room_idx = utils::random() % potential_tops.len();
        }
        if idx == 1 {
            if potential_bots.len() == 0 {
                idx += 1;
            }
            dir = Direction::Bottom;
            room_idx = utils::random() % potential_bots.len();
        }
        if idx == 2 {
            if potential_rights.len() == 0 {
                idx += 1;
            }
            dir = Direction::Right;
            room_idx = utils::random() % potential_rights.len();
        }
        if idx == 3 {
            if potential_lefts.len() == 0 {
                idx = -1;
            }
            dir = Direction::Left;
            room_idx = utils::random() % potential_lefts.len();
        }
        if idx == -1 {
            break;
        }
        let location: Vector2;
        let normal: Vector2;
        let r = &floor[room_idx];
        let x = r.x as f32;
        let y = r.y as f32;
        let w = r.width as f32;
        let h = r.height as f32;
        match dir {
            Direction::Top => {
                location = vec2(x + w / 2.0, y);
                normal = vec2(0.0, 1.0);
            }
            Direction::Left => {
                location = vec2(x, y + h / 2.0);
                normal = vec2(-1.0, 0.0);
            }
            Direction::Right => {
                location = vec2(x + w, y + h / 2.0);
                normal = vec2(1.0, 0.0);
            }
            Direction::Bottom => {
                location = vec2(x + w / 2.0, y + h);
                normal = vec2(0.0, -1.0);
            }
        }
        if room::inside_set(location + normal * 10.0, floor, r) {
            continue;
        }
        out.push(Portal {
            idx1: room_idx as i32,
            idx2: room_idx as i32,
            location: location,
            dir: dir,
        });
        count += 1;
        if count >= max_count {
            break;
        }
    }
    return out;
}
pub fn calc_doors(building: &Vec<Vec<Room>>) -> Vec<Vec<Portal>> {
    let mut out = vec![];
    for i in 0..building.len() {
        out.push(calc_doors_floor(&building[i], 1 == 0));
    }
    return out;
}
pub fn calc_stairs_floor(f0: &Vec<Room>, f1: &Vec<Room>, floor: i32) -> Vec<Stair> {
    if f1.len() == 0 {
        return vec![];
    }
    let count: usize;
    let frac = 5;
    if f1.len() < frac * 2 {
        count = 2;
    } else {
        count = f1.len() / 2;
    }
    let mut out = vec![];
    for _ in 0..count {
        loop {
            let idx = utils::random() % f1.len();
            let loc = f1[idx].center();
            let lv = vec2(loc.0 as f32, loc.1 as f32);
            let tmp = Stair {
                bot: floor - 1,
                top: floor,
                location: lv,
            };
            if !room::inside_set(lv, f0, &f1[0]) {
                continue;
            }
            out.push(tmp);
            break;
        }
    }
    return out;
}
pub fn calc_stairs(building: &Vec<Vec<Room>>) -> Vec<Stair> {
    let mut out = vec![];
    for i in 1..building.len() {
        let mut p = calc_stairs_floor(&building[i - 1], &building[i], i as i32);
        out.append(&mut p);
    }
    return out;
}
