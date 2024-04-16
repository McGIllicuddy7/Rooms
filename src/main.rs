

use raylib::{ffi::sqrt, prelude::*, consts::KeyboardKey, math::Vector2};
use rand;
const SCREEN_HEIGHT:i32 = 1000;
const SCREEN_WIDTH:i32 = 1000;
pub struct TreeRoom{
    pub x: i32,
    pub y: i32, 
    pub height :i32, 
    pub width : i32, 
    pub dropped: bool,
    child_2 : Option<Box<TreeRoom>>,
    child_1 : Option<Box<TreeRoom>>
}
pub struct Room{
    pub x:i32, 
    pub y:i32, 
    pub height:i32, 
    pub width:i32,
}
impl Clone for Room{
    fn clone(&self)->Self {
        return Room{x:self.x, y:self.y, height:self.height,width:self.width};
    }
}
impl Room{
    pub fn corners(&self)->Vec<Vector2>{
        let x = self.x as f32;
        let y = self.y as f32;
        let h = self.height as f32;
        let w = self.width as f32;
        return vec![Vector2{x:x, y:y},Vector2{x:x, y:y+h}, Vector2{x:x+w, y:y},Vector2{x:x+w, y:y+h}];
    }
    pub fn render(&self,handle: &mut RaylibDrawHandle){
        draw_rectangle(handle, self.x, self.y, self.height, self.width);
        let cs = self.corners();
        for s in cs{
            handle.draw_circle_v(s, 4.0, Color::RED);
        }
    }
    pub fn render_debug(&self, selves:&Vec<Self>,handle:&mut RaylibDrawHandle){
        draw_rectangle(handle, self.x, self.y, self.height, self.width);
        let cs = self.corners();
        for s in cs{
            if inside_set(s, selves, self){
                handle.draw_circle_v(s, 4.0, Color::GREEN);
            }

        }
    }
    pub fn to_rect(&self)->Rectangle{
        return Rectangle{x: self.x as f32, y: self.y as f32, width: self.width as f32, height: self.width as f32};
    }
    pub fn inside(&self, loc:Vector2)->bool{
        return self.to_rect().check_collision_point_rec(loc);
    }
    pub fn is_equal(&self, other:&Self)->bool{
        if self.x != other.x{
            return false;
        }
        if self.y != other.y{
            return false;
        }
        if self.height != other.height{
            return false;
        }
        if self.width != other.width{
            return false;
        }
        return true;
    }
    fn is_border(&self, flattened: &Vec<Room>)->bool{
        fn check_point(loc: Vector2, flattened: &Vec<Room>, ignore:&Room)->bool{
            let l1 = Vector2 {x:loc.x+1.0, y:loc.y};
            let l2 = Vector2 {x:loc.x-1.0, y:loc.y};
            let l3 = Vector2{x:loc.x, y:loc.y+1.0};
            let l4  = Vector2{x:loc.x, y:loc.y-1.0};
            if inside_set(l1, flattened, ignore){
                return false;
            }
            if inside_set(l2, flattened,ignore){
                return false;
            }
            if inside_set(l3, flattened,ignore){
                return false;
            }
            if inside_set(l4, flattened,ignore){
                return false;
            }
            return true
        }
        let v0 = Vector2{x:self.x as f32, y: self.y as f32};
        let b0 = check_point(v0, flattened, self);
        let v1 = Vector2{x:self.x as f32, y: self.y as f32};
        let b1 = check_point(v0, flattened,self);
        todo!();
    }
}
fn draw_rectangle(handle: &mut RaylibDrawHandle, x:i32, y:i32, height:i32, width:i32){
    let cy =y;
    handle.draw_line(x, cy,x+width, cy ,Color::BLACK);
    handle.draw_line(x, cy,x, cy+height ,Color::BLACK);
    handle.draw_line(x, cy+height,x+width, cy+height ,Color::BLACK);
    handle.draw_line(x+width, cy,x+width, cy+height ,Color::BLACK);
}
pub fn random() ->usize{
    rand::random::<usize>()
}
pub fn dist(a:i32, b:i32) ->i32{
    let f = a-b;
    if f<0{
        return -f;
    }
    return f;

}
pub fn generate_toward_mid(min:i32, max:i32, num_its:i32)->i32{
    let mid = (max+min)/2;
    let mut closest: i32 = (random() as i32)%(max-min)+min;
    let mut min_dist :i32 = dist(closest, mid);
    for _ in 0..num_its{
        let tmp = (random() as i32)%(max-min)+min;
        let d = dist(tmp, mid);
        if d<min_dist{
            min_dist = d;
            closest = tmp;
        }
    }
    return closest;
}
fn obtr_clone(start: &Option<Box<TreeRoom>>) ->Option<Box<TreeRoom>>{
    if start.as_ref().is_none(){
        return None;
    }
    else {
        return Some(start.as_ref().unwrap().clone());
    }
}  
impl Clone for TreeRoom{
    fn clone(&self) -> Self {
        return Self{ x: self.x, y: self.y, height:self.height, width:self.width, dropped:self.dropped, child_1:obtr_clone(&self.child_1), child_2:obtr_clone(&self.child_2)}
    }
}
fn inside_set(point:Vector2, set: &Vec<Room>,ignore: &Room)->bool{
    for i in  set{
        if i.is_equal(ignore){
            continue;
        }
        if i.inside(point){
            return true;
        }
    }
    return false;
}
impl TreeRoom {
    pub fn new(x:i32, y:i32, height:i32, width:i32) -> Self {
        //println!("x:{},y:{}, height:{}, width:{}",x,y,height,width);
        return Self { x: x,y: y,height: height, width: width,dropped:false, child_1: None, child_2:None};
    }
    fn is_bottom(&self)->bool{
        if self.child_1.is_none() && self.child_2.is_none(){
            return true;
        }
        return false;
    }
    pub fn center(&self) ->(i32,i32){
        return ((self.x+self.width/2), (self.y+self.height/2));
    }
    pub fn dist_to_center(&self)->i32 {
        let tmp :(i32,i32) = self.center();
        let cx :f64 =  tmp.0 as f64;
        let cy :f64 = tmp.1 as f64;
        let dx :f64 = (cx-(SCREEN_WIDTH/2) as f64)*(cx-(SCREEN_WIDTH/2) as f64);
        let dy :f64 = (cy-(SCREEN_HEIGHT/2) as f64)*(cy-(SCREEN_HEIGHT/2) as f64);
        unsafe{
            return sqrt(dx+dy) as i32;
        }
    }
    fn split_x(&mut self, breadth: i32){
        let split : i32 = generate_toward_mid(self.width/2-breadth, self.width/2+breadth,3);
        self.child_1 = Some(Box::new(TreeRoom::new(self.x, self.y,self.height, split)));
        self.child_2 = Some(Box::new(TreeRoom::new(self.x+split, self.y, self.height, self.width-split)));
    }
    fn split_y(&mut self, breadth: i32){
        let split : i32 = generate_toward_mid(self.height/2-breadth, self.height/2+breadth,3);
        self.child_1 = Some(Box::new(TreeRoom::new(self.x, self.y,split, self.width)));
        self.child_2 =Some(Box::new(TreeRoom::new(self.x, self.y+split, self.height-split, self.width)));
    }
    fn split_recurse(&mut self, max_depth:usize, depth:usize){
        if depth>=max_depth {
            return;
        }
        if self.is_bottom() {
            let rat:f32 = self.height as f32 /self.width as f32 ;
            let rat2 :f32 = self.width as f32 /self.height as f32 ;
            if depth == max_depth-2{
                if rat >0.666667 && rat<1.5{
                    return; 
                }
            }
            if self.height <SCREEN_HEIGHT/100 || self.width<SCREEN_WIDTH/100{
                return;
            }
            let max:i32;
            if self.height>self.width{
                max = self.width
            }else{
                max = self.height;
            }
            let breadth:i32;
            if depth<max_depth-2{
                breadth = (max)/4;
            }
            else {
                breadth = max/8;
            }
            if rat>1.2|| rat2>1.2 {
                if rat>rat2{
                    self.split_y(breadth);
                } else{
                    self.split_x(breadth);
                }
            } else {
                if depth >= max_depth-2{
                    if rat >0.833333 && rat<1.2{
                        return;
                    }
                }
                if random()%2 == 0{
                    self.split_x(breadth);
                } else{
                    self.split_y(breadth);
                }
            }

        }
        if self.child_1.is_some(){
            //self.child_1.as_mut().as_mut().unwrap().split_recurse(max_depth, depth+1);
            self.child_1.as_mut().unwrap().split_recurse(max_depth, depth+1);
        }
        if self.child_2.is_some(){
            //self.child_2.as_mut().as_mut().unwrap().split_recurse(max_depth, depth+1);
            self.child_2.as_mut().unwrap().split_recurse(max_depth, depth+1);
        }
    }
    pub fn split(&mut self, max_depth:usize){
        self.split_recurse(max_depth, 0);
    }
    pub fn render(&self, handle: &mut RaylibDrawHandle){
        if self.is_bottom(){
            if !self.dropped{
                draw_rectangle(handle, self.x, self.y, self.height, self.width);
            }
            return;
        }
        if self.child_1.is_some(){
            self.child_1.as_ref().as_ref().unwrap().render(handle);
        }
        if self.child_2.is_some(){
            self.child_2.as_ref().as_ref().unwrap().render(handle);
        }

    }
    fn drop_c1(&mut self, depth :i32, rad_min:i32, rad_max:i32){
        if self.child_1.is_some(){
            self.child_1.as_mut().as_mut().unwrap().drop_random(depth+1, rad_min, rad_max);
        }
    }
    fn drop_c2(&mut self, depth :i32, rad_min:i32, rad_max:i32){
        if self.child_2.is_some(){
            self.child_2.as_mut().as_mut().unwrap().drop_random(depth+1, rad_min, rad_max);
        }
    }
    pub fn drop_random(&mut self,depth :i32,rad_min:i32, rad_max:i32){
        if depth == 0{
            self.drop_c1(depth, rad_min, rad_max);
            self.drop_c2(depth, rad_min, rad_max);
            return;
        }
        if depth>4{
            if self.dist_to_center()>generate_toward_mid(rad_min,rad_max, 4){
                self.child_1 = None;
                self.child_2 = None;
                if self.is_bottom(){
                    self.dropped = true;
                }
            }
        }
        self.drop_c1(depth, rad_min, rad_max);
        self.drop_c2(depth, rad_min, rad_max);

    }
    pub fn _pr_ratio(&mut self){
        if self.is_bottom(){
            let rat1 = self.height as f32/ self.width as f32;
            let rat2 = self.width as f32/ self.height as f32;
            if rat1 >rat2{
                println!("ratio:{}", rat1);
            }
            else{
                println!("ratio:{}", rat2);
            }
            return;
        } 
        if self.child_1.is_some(){
            self.child_1.as_mut().unwrap()._pr_ratio();
        }
        if self.child_2.is_some(){
            self.child_2.as_mut().unwrap()._pr_ratio();
        }
    }
    pub fn to_room(&self) ->Room{
        return Room{x:self.x, y:self.y, width:self.width, height:self.height};
    }
    pub fn flatten(&self) ->Vec<Room>{
        if self.is_bottom(){
            if !self.dropped{
                return vec![self.to_room()];
            } else{
                return vec![];
            }
        }
        let mut out:Vec<Room> = vec![];
        if self.child_1.is_some(){
            let a = self.child_1.as_ref().unwrap().flatten();
            for i in a.into_iter(){
                out.push(i);
            }
        }
        if self.child_2.is_some(){
            let a = self.child_2.as_ref().unwrap().flatten();
            for i in a.into_iter(){
                out.push(i);
            }
        }
        return out;
    }
}

pub fn purge_not_on_top(modified:&Vec<Room>, base: &Vec<Room> )->Vec<Room>{
    let mut out:Vec<Room> = Vec::new();
    for room in modified{
        let s = room.corners();
        let mut applicable = true;
        for corner in s{
            if !inside_set(corner, base, &room){
                println!("continuing\n");
                applicable= false;
            }
        }
        if applicable{
            out.push(room.clone());
        }
    }
    return out;
}
pub fn new_building() ->Vec<Room>{
    let mut r = TreeRoom::new(0,0, 800, 800);
    r.split(8);
    r.drop_random(0, 150, SCREEN_WIDTH/3);
    return r.flatten();
}
pub fn new_floor(previous: &Vec<Room>)->Vec<Room>{
    let mut r = TreeRoom::new(0,0, 800, 800);
    r.split(8);
    let tout = r.flatten();
    let out = purge_not_on_top(&tout,previous);
    return out;
}
pub fn render_rooms(rooms: &Vec<Room>, handle: &mut RaylibDrawHandle){
    for i in rooms{
        i.render(handle);
    }
}
pub fn render_rooms_debug(rooms: &Vec<Room>,prev_floor:&Vec<Room>, handle: &mut RaylibDrawHandle){
    for i in rooms{
        i.render_debug(prev_floor, handle);
    }
}
fn main() {
    let mut a= new_building();
    let mut b = new_floor(&a);
    raylib::set_trace_log(TraceLogLevel::LOG_ERROR);
    let (mut rl, thread) = raylib::init()
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
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
            a = new_building();
            b = new_floor(&a);
        }
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::WHITE);
        if render_both{
            render_rooms(&a, &mut d);
            render_rooms_debug(&b, &a,&mut d);
            d.draw_text("rendering 1st floor", 800, 800, 16, Color::BLACK);
        } else{
            if render_tree{
                render_rooms(&a, &mut d);
                d.draw_text("rendering 1st floor", 800, 800, 16, Color::BLACK);
            } else{
                render_rooms_debug(&b,  &a, &mut d);
                d.draw_text("rendering 2nd floor", 800, 800, 16, Color::BLACK);
            }
        }
    }
}
