
mod utils;
mod room;
mod config;
use raylib::{prelude::*, consts::KeyboardKey};


fn main() {
    let mut a= room::new_building();
    let mut b = room::new_floor(&a);
    raylib::set_trace_log(TraceLogLevel::LOG_ERROR);
    let (mut rl, thread) = raylib::init()
        .size(config::SCREEN_WIDTH, config::SCREEN_HEIGHT)
        .title("Hello, World")
        .build();
    let mut render_tree = true;
    let mut render_both = false;
    while !rl.window_should_close() {
        if rl.is_key_pressed(KeyboardKey::KEY_H){
            render_tree = !render_tree;
        }
        if rl.is_key_pressed(KeyboardKey::KEY_B){
            render_both = !render_both;
        }
        if rl.is_key_pressed(KeyboardKey::KEY_N){
            a = room::new_building();
            b = room::new_floor(&a);
        }
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::WHITE);
        if render_both{
            room::render_rooms(&a, &mut d);
            room::render_rooms_debug(&b, &a,&mut d);
            d.draw_text("rendering 1st floor", 800, 800, 16, Color::BLACK);
        } else{
            if render_tree{
                room::render_rooms(&a, &mut d);
                d.draw_text("rendering 1st floor", 800, 800, 16, Color::BLACK);
            } else{
                room::render_rooms_debug(&b,  &a, &mut d);
                d.draw_text("rendering 2nd floor", 800, 800, 16, Color::BLACK);
            }
        }
    }
}
