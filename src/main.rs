mod room;
mod config;
mod building;
mod utils;
use std::fmt::format;
use raylib::{consts::KeyboardKey, prelude::*};


fn main() {
    let num_floors = 5;
    let num_rooms = 10;
    let mut b: building::Building = building::generate_building(num_rooms, num_floors);
    raylib::set_trace_log(TraceLogLevel::LOG_ERROR);
    let (mut rl, thread) = raylib::init()
        .size(config::SCREEN_WIDTH, config::SCREEN_HEIGHT)
        .title("Hello, World")
        .build();
    let mut floor:usize = 0;
    while !rl.window_should_close() {
        if rl.is_key_pressed(KeyboardKey::KEY_N){
            floor += 1;
            if floor>num_floors-1{
                floor = 0;
            }
        }
        if rl.is_key_pressed(KeyboardKey::KEY_P){
            if floor == 0{
                floor = num_floors-1;
            } else{
                floor -=1;
            }
        }
        if rl.is_key_pressed(KeyboardKey::KEY_W){
            b= building::generate_building(num_rooms, num_floors);
            floor = 0;
        }
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::WHITE);
        b.render_floor(floor, &mut d);
        let s = format(format_args!("drawing floor: {}", floor));
        d.draw_text(&s,800, 800, 16, Color::BLACK);
    }
}
