mod room;
mod config;
mod building;
mod utils;
mod layout;
mod output;
use std::{env::args, fmt::format};
use std::process::exit;
use raylib::{consts::KeyboardKey, prelude::*};
fn usage_message(){
    println!("usage: room --floors <how many floors> --rooms <how many rooms on first floor>...");
    println!("options:");
    println!("    --name <name>, name of the building, defaults to \"Building\"");
    println!("    --cell-size <cell size>: size of the cells in pixels(defaults to 25)");
    println!("    --scale-size <scale size>:amount to scale the building by(defaults to 5)");
    println!("    --show-bg <t/f>: whether or not to show a background(defaults to f");
    println!("    --show-grid <t/f>: whether or not to show the grid(defualts to t");
    println!("    --floors <number of floors>: required, how many floors to make");
    println!("    --rooms <number of rooms>: required, approximately how many rooms to put on the first floor");
}

fn read_input()->Option<config::Config>{
    enum States{
        None,
        Name,
        CellSize,
        ScaleSize,
        RenderBackground, 
        RenderGrid,
        NumRooms,
        NumFloors,
    }
    let mut out:config::Config = config::Config{cell_size:25.0, scale_size:6.0, render_background:false, render_grid:true, num_floors:0, num_rooms:0, name:format!("Building")};
    let mut args = args();
    let mut state:States = States::None;
    let mut num_rooms_configed = false;
    let mut num_floors_configed = false;
    let mut count = 0;
    let _pg_name = args.next();
    let mut last = format!("");
    for arg in args{
        if count == 0 && arg == "help" || arg == "Help" || arg == "h"{
            usage_message();
            break;
        }
        match state{
            States::None=>{
                last = arg.clone();
                if arg == "--cell-size"{
                    state = States::CellSize;
                }
                else if arg == "--name"{
                    state = States::Name;
                }
                else if arg == "--scale-size"{
                    state = States::ScaleSize;
                }
                else if arg == "--show-bg"{
                    state = States::RenderBackground;
                }
                else if arg== "--show-grid"{
                    state = States::RenderGrid;
                }
                else if arg == "--floors"{
                    state = States::NumFloors;
                }
                else if arg == "--rooms"{
                    state = States::NumRooms;
                } else{
                    return None;
                } 
                count += 1;
                continue;
            }
            States::CellSize=>{
                let t = arg.parse();
                if t.is_err(){
                    eprintln!("last: {}, {}",last, t.unwrap_err());
                    return None;
                }
                out.cell_size = t.unwrap();
            }
            States::ScaleSize=>{
                let t = arg.parse();
                if t.is_err(){
                    eprintln!("last: {}, {}",last, t.unwrap_err());
                    return None;
                }
                out.scale_size = t.unwrap();
            }
            States::RenderBackground=>{
                let t = arg.parse();
                if t.is_err(){
                    eprintln!("last: {}, {}",last, t.unwrap_err());
                    return None;
                }
                out.render_background = t.unwrap();
            }
            States::RenderGrid=>{
                let t = arg.parse();
                if t.is_err(){
                    eprintln!("last: {}, {}",last, t.unwrap_err());
                    return None;
                }
                out.render_grid = t.unwrap();
            }
            States::NumFloors=>{
                let t = arg.parse();
                if t.is_err(){
                    eprintln!("last: {}, {}",last, t.unwrap_err());
                    return None;
                }
                out.num_floors= t.unwrap();
                num_floors_configed = true;
            }
            States::NumRooms=>{
                let t = arg.parse();
                if t.is_err(){
                    eprintln!("last: {}, {}",last, t.unwrap_err());
                    return None;
                }
                out.num_rooms= t.unwrap();
                num_rooms_configed = true;
            }
            States::Name=>{
                out.name = arg;
            }
        }
        state = States::None;
    }
    if !num_rooms_configed{
        return None;
    }
    if !num_floors_configed{
        return None;
    }
    return Some(out);
}
fn display(mut b:building::Building, confg:config::Config){
    raylib::set_trace_log(TraceLogLevel::LOG_ERROR);
    let (mut rl, thread) = raylib::init()
        .size(config::SCREEN_WIDTH, config::SCREEN_HEIGHT)
        .title("Hello, World")
        .build();
    let mut floor:usize = 0;
    while !rl.window_should_close() {
        if rl.is_key_pressed(KeyboardKey::KEY_N){
            floor += 1;
            if floor>b.num_floors()-1{
                floor = 0;
            }
        }
        if rl.is_key_pressed(KeyboardKey::KEY_P){
            if floor == 0{
                floor = b.num_floors()-1;
            } else{
                floor -=1;
            }
        }
        if rl.is_key_pressed(KeyboardKey::KEY_W){
            b= building::generate_building(confg.num_rooms, confg.num_floors, &confg);
            floor = 0;
        }
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::WHITE);
        b.render_floor(floor, &mut d);
        let s = format(format_args!("drawing floor: {} with {} rooms", floor,b.floors[floor].len()));
        d.draw_text(&s,800, 800, 16, Color::BLACK);
        exit(0);
    }
}
fn main() {
    let confg_opt= read_input();
    if confg_opt.is_none(){
        eprintln!("usage: room --floors <how many floors> --rooms <how many rooms on first floor>...");
        exit(1);
    }
    let confg = confg_opt.unwrap();
    let b: building::Building = building::generate_building(confg.num_rooms, confg.num_floors,&confg);
    b.render_out(&confg.name, &confg);
    output::output_building(&b, &confg.name);
    display(b, confg);
}
