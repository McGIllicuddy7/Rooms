use crate::utils::{vec_2_unsafe, Vector2};
use crate::{
    config::{self, DEBUG_TIMING},
    layout,
    room::{self, inside_set, purge_not_on_top, TreeRoom},
    utils,
};
use layout::Direction;
use layout::Portal;
use layout::Stair;
use nalgebra_glm::*;
use rust_raylib::drawing::*;

use std::thread;
use std::time::Instant;
use trustme::*;
pub struct Building {
    pub floors: Vec<Vec<room::Room>>,
    pub stairs: Vec<Stair>,
    pub doors: Vec<Vec<Portal>>,
}
pub fn _floor_center(floor: &Vec<room::Room>) -> Vector2 {
    let mut out = vec2(0.0, 0.0);
    for i in floor {
        for j in i.corners() {
            out = vec2(out.x + j.x, out.y + j.y);
        }
    }
    return out * (floor.len() * 4) as f32;
}
pub fn _floor_divergence(floor: &Vec<room::Room>) -> f64 {
    let mut max: f64 = 0.0;
    let v = _floor_center(floor);
    for i in floor {
        for j in i.corners() {
            let l = distance(&j, &v) as f64;
            if l > max {
                max = l;
            }
        }
    }
    return max;
}
pub fn _floor_radius(floor: &Vec<room::Room>) -> f64 {
    let mut max: f64 = 0.0;
    let c = ((config::BUILDING_MAX + config::BUILDING_MIN) / 2) as f32;
    let v = vec2(c, c);
    for i in floor {
        for j in i.corners() {
            let l = distance(&j, &v) as f64;
            if l > max {
                max = l;
            }
        }
    }
    return max;
}
fn floor_area(floor: &Vec<room::Room>) -> f64 {
    let mut out = 0.0;
    for r in floor {
        out += (r.height * r.width) as f64;
    }
    return out;
}
fn _calc_depth(ground_floor_num: i32) -> usize {
    let amnt = (ground_floor_num) as f64;
    let amnt2 = amnt * amnt;
    let tmp = amnt2.log2();
    return (tmp as f64 + 0.25).ceil() as usize + 1;
}

fn comparitior_weight(a: &Vec<room::Room>) -> f64 {
    return floor_area(a) as f64 - _floor_radius(&a) as f64;
}
fn comparitor(a: &Vec<room::Room>, b: &Vec<room::Room>) -> bool {
    let wa = comparitior_weight(a);
    let wb = comparitior_weight(b);
    return wa > wb;
}
fn gen_frst_floor(max_depth: &usize, desired_num: i32, confg: config::Config) -> (f64, TreeRoom) {
    let mut root: TreeRoom = TreeRoom::new(1, 1, 10, 10);
    let mut weight: f64 = 0.0;
    let mut i = 0;
    let mx = 3;
    loop {
        if i > mx {
            break;
        }
        let mut tree = room::TreeRoom::new(
            config::BUILDING_MIN,
            config::BUILDING_MIN,
            config::BUILDING_MAX,
            config::BUILDING_MAX,
        );
        tree.split(*max_depth, &confg);
        tree.drop_to_number(desired_num);
        let floors = tree.flatten();
        let floors = room::purge_degenerates(&floors);
        let r = comparitior_weight(&floors);
        if r > weight {
            root = tree;
            weight = r;
        }
        i += 1;
    }
    return (weight, root);
}
fn generate_floors(ground_floor_num: i32, num_floors: usize, confg: &config::Config) -> Building {
    if ground_floor_num < 1 {
        panic!("not enough rooms");
    }
    let max_depth = 17;
    let mut out: Building = Building::new();
    let mut prev: TreeRoom;
    let mut root: TreeRoom = room::TreeRoom::new(
        config::BUILDING_MIN,
        config::BUILDING_MIN,
        config::BUILDING_MAX,
        config::BUILDING_MAX,
    );
    let mut weight: f64 = 0 as f64;
    let start = Instant::now();
    let mut threads = vec![];
    for _ in 0..10 {
        let cnfg = confg.clone();
        let a = thread::spawn(move || (gen_frst_floor(&max_depth, ground_floor_num, cnfg)));
        threads.push(a);
    }
    for a in threads {
        let t = a.join().unwrap();
        if t.0 > weight {
            weight = t.0;
            root = t.1;
        }
    }
    out.floors.push(root.flatten());
    prev = root;
    if DEBUG_TIMING {
        println!("first floor done in {:#?}", Instant::now() - start);
    }
    for i in 1..num_floors {
        let start = Instant::now();
        let mut l = prev.clone();
        l = l.template();
        l.split_recurse(max_depth, 0, confg);
        let mut tmp = l.flatten();
        tmp = purge_not_on_top(&tmp, &out.floors[i - 1]);
        prev = l.template();
        for _ in 0..3 {
            loop {
                let mut tre0 = prev.clone();
                tre0 = tre0.template();
                tre0.split_recurse(max_depth, 0, confg);
                let mut tmp1 = tre0.flatten();
                tmp1 = purge_not_on_top(&tmp1, &out.floors[i - 1]);
                //tmp1 = room::purge_degenerates(&tmp1);
                if comparitor(&tmp1, &tmp) {
                    tmp = tmp1;
                    prev = tre0.template();
                }
                let mut tree = room::TreeRoom::new(
                    config::BUILDING_MIN,
                    config::BUILDING_MIN,
                    config::BUILDING_MAX,
                    config::BUILDING_MAX,
                );
                tree.split(max_depth, confg);
                let mut tmp2 = tree.flatten();
                tmp2 = purge_not_on_top(&tmp2, &out.floors[i - 1]);
                tmp2 = room::purge_degenerates(&tmp2);
                if comparitor(&tmp2, &tmp) {
                    tmp = tmp2;
                    prev = tree.template();
                }
                break;
            }
        }
        if DEBUG_TIMING {
            println!("floor {} finished in {:#?}", i + 1, Instant::now() - start);
        }
        if tmp.len() < 1 {
            break;
        }
        out.floors.push(tmp);
    }
    return out;
}

pub fn generate_building(
    ground_floor_num: i32,
    num_floors: usize,
    confg: &config::Config,
) -> Building {
    let mut out = generate_floors(ground_floor_num, num_floors, confg);
    out.doors = layout::calc_doors(&out.floors);
    out.stairs = layout::calc_stairs(&out.floors);
    out.scale(confg.scale_size as f64);
    return out;
}
impl Building {
    pub fn new() -> Self {
        return Self {
            floors: Vec::new(),
            stairs: Vec::new(),
            doors: Vec::new(),
        };
    }
    pub fn render_floor(&self, floor: usize, handle: &mut DrawHandle) {
        if floor > self.floors.len() {
            return;
        }
        room::render_rooms(&self.floors[floor], handle);
        for i in 0..self.floors[floor].len() {
            let text = format!("{}", i);
            let c = self.floors[floor][i].center();
            handle.draw_text(
                text.as_str(),
                vec_2_unsafe(c.0 as f32, (c.1 - 12) as f32).into(),
                16,
                rust_raylib::color::Color::BLACK,
            );
        }
        if self.doors.len() > floor {
            for d in &self.doors[floor] {
                match d.dir {
                    Direction::Top | Direction::Bottom => {
                        utils::draw_rectangle_centered(handle, &d.location, 9, 18);
                    }
                    Direction::Right | Direction::Left => {
                        utils::draw_rectangle_centered(handle, &d.location, 18, 9);
                    }
                }
                /*
                let text0 = format!("{}", d.idx1);
                let text1= format!("{}", d.idx2);
                let x = d.location.x as i32;
                let y = d.location.y as i32;
                match d.dir{
                    Direction::Top=>{
                        handle.draw_text(text0.as_str(), x, y-16,12, Color::BLACK);
                        handle.draw_text(text1.as_str(), x, y+8, 12, Color::BLACK);
                    }
                    Direction::Bottom=>{
                        handle.draw_text(text0.as_str(), x, y+8,12, Color::BLACK);
                        handle.draw_text(text1.as_str(), x, y-16, 12, Color::BLACK);
                    }
                    Direction::Left=>{
                        handle.draw_text(text0.as_str(), x-8, y,12, Color::BLACK);
                        handle.draw_text(text1.as_str(), x+8, y,12, Color::BLACK);
                    }
                    Direction::Right=>{
                        handle.draw_text(text0.as_str(), x+8, y,12, Color::BLACK);
                        handle.draw_text(text1.as_str(), x-8, y,12, Color::BLACK);
                    }
                }
                        */
            }
        }
        for s in &self.stairs {
            let mut other: i32 = -1;
            if s.top == floor as i32 {
                other = s.bot as i32;
            }
            if s.bot == floor as i32 {
                other = s.top as i32;
            }
            if other == -1 {
                continue;
            }
            utils::render_stairs(handle, s.location);
            let msg = format!("to {}", other);
            handle.draw_text(
                &msg,
                vec_2_unsafe(
                    (s.location.x as i32 - 6) as f32,
                    (s.location.y as i32 + 12) as f32,
                )
                .into(),
                12,
                rust_raylib::color::Color::BLACK,
            );
        }
    }
    pub fn num_floors(&self) -> usize {
        return self.floors.len();
    }
    pub fn scale(&mut self, scale: f64) {
        let x = (config::SCREEN_WIDTH / 2) as f64;
        let y = (config::SCREEN_HEIGHT / 2) as f64;
        for i in &mut self.floors {
            for j in i {
                let dx = (j.x as f64 - x) * scale;
                let dy = (j.y as f64 - y) * scale;
                j.x = (x + dx) as i32;
                j.y = (y + dy) as i32;
                j.height = ((j.height as f64) * scale) as i32;
                j.width = ((j.width as f64) * scale) as i32;
            }
        }
        for i in &mut self.doors {
            for j in i {
                let delta = j.location - (vec2(x as f32, y as f32));
                j.location = delta * (scale as f32) + (vec2(x as f32, y as f32));
            }
        }
        for j in &mut self.stairs {
            let delta = j.location - (vec2(x as f32, y as f32));
            j.location = delta * (scale as f32) + (vec2(x as f32, y as f32));
        }
    }
    unsafe fn render_floor_out(&self, floor: usize, name: &str, confg: &config::Config) {
        let texture = rust_raylib::ffi::LoadRenderTexture(1000, 1000);
        let bg = generate_background(&self.floors[floor], confg);
        {
            rust_raylib::ffi::BeginTextureMode(texture.clone());
            rust_raylib::ffi::ClearBackground(rust_raylib::ffi::colors::WHITE);
            rust_raylib::ffi::DrawTexture(
                bg.texture.clone(),
                0,
                0,
                rust_raylib::ffi::colors::WHITE,
            );
            for f in &self.floors[floor] {
                f.render_unsafe();
            }
            for i in 0..self.floors[floor].len() {
                let text = format!("{}", i);
                let c = self.floors[floor][i].center();
                rust_raylib::ffi::DrawText(
                    text.as_str().as_ptr() as *const i8,
                    c.0,
                    c.1 - 16,
                    16,
                    rust_raylib::ffi::colors::BLACK,
                );
            }
        }
        for s in &self.stairs {
            let mut other: i32 = -1;
            if s.top == floor as i32 {
                other = s.bot as i32;
            }
            if s.bot == floor as i32 {
                other = s.top as i32;
            }
            if other == -1 {
                continue;
            }
            utils::render_stairs_unsafe(s.location);
            let msg = format!("to {}", other);
            rust_raylib::ffi::DrawText(
                msg.as_ptr() as *const i8,
                s.location.x as i32,
                s.location.y as i32 + 20,
                8,
                rust_raylib::ffi::colors::BLACK,
            );
        }
        for d in &self.doors[floor] {
            match d.dir {
                Direction::Top | Direction::Bottom => {
                    utils::draw_rectangle_centered_unsafe(&d.location, 9, 18);
                }
                Direction::Right | Direction::Left => {
                    utils::draw_rectangle_centered_unsafe(&d.location, 18, 9);
                }
            }
        }
        rust_raylib::ffi::EndTextureMode();
        rust_raylib::ffi::UnloadRenderTexture(bg);
        let mut image = rust_raylib::ffi::LoadImageFromTexture(texture.texture);
        rust_raylib::ffi::ImageFlipVertical(&mut image);
        let s = format!("output/{}{}.png", name, floor + 1);
        let tmp = &s;
        rust_raylib::ffi::ExportImage(image, tmp.as_ptr() as *const i8);
    }
    pub fn render_out(&self, name: &str, confg: &config::Config) {
        trustme! {
            let s = "testing 1 2 3";
            rust_raylib::ffi::SetTraceLogLevel(rust_raylib::ffi::TraceLogLevel::None as i32);
            rust_raylib::ffi::InitWindow(1000, 1000, s.as_ptr() as *const i8);
        }
        for i in 0..self.num_floors() {
            trustme! {
                if self.floors[i].len()>0{
                    self.render_floor_out(i, name,confg);
                }
            }
        }
    }
}
fn to_vec_2(x: i32, y: i32) -> Vector2 {
    return vec2(x as f32, y as f32);
}
fn sample_wood_texture(x: i32, y: i32) -> rust_raylib::ffi::Color {
    if x % 25 == 0 || y % 25 == 0 {
        return rust_raylib::ffi::colors::BLACK;
    }
    return rust_raylib::ffi::Color {
        r: 60,
        g: 40,
        b: 0,
        a: 255,
    };
}
fn sample_ground_texture(x: i32, y: i32) -> rust_raylib::ffi::Color {
    if x % 25 == 0 || y % 25 == 0 {
        return rust_raylib::ffi::colors::BLACK;
    }
    return rust_raylib::ffi::Color {
        r: 0,
        g: 125,
        b: 0,
        a: 255,
    };
}
unsafe fn generate_background(
    floor: &Vec<room::Room>,
    confg: &config::Config,
) -> rust_raylib::ffi::RenderTexture2D {
    use rust_raylib::ffi::*;
    let out = LoadRenderTexture(config::SCREEN_WIDTH, config::SCREEN_HEIGHT);
    {
        BeginTextureMode(out.clone());
        ClearBackground(colors::WHITE);
        let fs = room::Room {
            x: 0,
            y: 0,
            height: 0,
            width: 0,
        };
        for x in 0..config::SCREEN_WIDTH {
            for y in 0..config::SCREEN_HEIGHT {
                if (x % (confg.cell_size as i32) == 0 || y % (confg.cell_size as i32) == 0)
                    && confg.render_grid
                {
                    DrawPixel(
                        x,
                        config::SCREEN_HEIGHT - y,
                        rust_raylib::ffi::colors::BLACK,
                    );
                } else if confg.render_background {
                    if inside_set(to_vec_2(x, y), floor, &fs) {
                        let c = sample_wood_texture(x, y);
                        DrawPixel(x, config::SCREEN_HEIGHT - y, c);
                    } else {
                        let c = sample_ground_texture(x, y);
                        DrawPixel(x, config::SCREEN_HEIGHT - y, c);
                    }
                }
            }
        }
        EndTextureMode();
    }
    return out;
}
