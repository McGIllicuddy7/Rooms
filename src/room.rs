
use crate::utils;
use crate::config;
use raylib::{ffi::sqrt, prelude::*, math::Vector2};
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
        utils::draw_rectangle(handle, self.x, self.y, self.height, self.width);
        let cs = self.corners();
        for s in cs{
            handle.draw_circle_v(s, 4.0, Color::RED);
        }
    }
    pub fn _render_debug(&self, selves:&Vec<Self>,handle:&mut RaylibDrawHandle){
        utils::draw_rectangle(handle, self.x, self.y, self.height, self.width);
        let cs = self.corners();
        for s in cs{
            if inside_set(s, selves, self){
                handle.draw_circle_v(s, 4.0, Color::GREEN);
            }

        }
    }
    pub fn to_rect(&self)->Rectangle{
        return Rectangle{x: self.x as f32, y: self.y as f32, width: self.width as f32, height: self.height as f32};
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
        let dx :f64 = (cx-(config::SCREEN_WIDTH/2) as f64)*(cx-(config::SCREEN_WIDTH/2) as f64);
        let dy :f64 = (cy-(config::SCREEN_HEIGHT/2) as f64)*(cy-(config::SCREEN_HEIGHT/2) as f64);
        unsafe{
            return sqrt(dx+dy) as i32;
        }
    }
    fn split_x(&mut self, breadth: i32){
        let split : i32 = utils::generate_toward_mid(self.width/2-breadth, self.width/2+breadth,3);
        self.child_1 = Some(Box::new(TreeRoom::new(self.x, self.y,self.height, split)));
        self.child_2 = Some(Box::new(TreeRoom::new(self.x+split, self.y, self.height, self.width-split)));
    }
    fn split_y(&mut self, breadth: i32){
        let split : i32 = utils::generate_toward_mid(self.height/2-breadth, self.height/2+breadth,3);
        self.child_1 = Some(Box::new(TreeRoom::new(self.x, self.y,split, self.width)));
        self.child_2 =Some(Box::new(TreeRoom::new(self.x, self.y+split, self.height-split, self.width)));
    }
    pub fn split_recurse(&mut self, max_depth:usize, depth:usize){
        if depth>=max_depth{
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
            if self.height*self.width<config::MIN_AREA{
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
                breadth = (max)/2;
            }
            else {
                breadth = max/4;
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
                if utils::random()%2 == 0{
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
    pub fn _render(&self, handle: &mut RaylibDrawHandle){
        if self.is_bottom(){
            if !self.dropped{
                utils::draw_rectangle(handle, self.x, self.y, self.height, self.width);
            }
            return;
        }
        if self.child_1.is_some(){
            self.child_1.as_ref().as_ref().unwrap()._render(handle);
        }
        if self.child_2.is_some(){
            self.child_2.as_ref().as_ref().unwrap()._render(handle);
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
            if self.dist_to_center()>utils::generate_toward_mid(rad_min,rad_max, 4){
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
    fn template_iterate(&mut self)->i32{
        if self.is_bottom(){
            return 0;
        }
        if self.dropped{
            return 0;
        }
        let mut out1 = 0;
        let mut out2 = 0;
        if self.child_1.is_some(){
            let t = self.child_1.as_mut().unwrap().template_iterate();
            out1 = t;
            if t <3 {
                self.child_1 = None;
            }
        }
        if self.child_2.is_some(){
            let t =self.child_2.as_mut().unwrap().template_iterate();
            out2 = t;
            if t <3{
                self.child_2 = None;
            }
        }
        if out1>out2{
            return out1+1;
        }
        return out2+1;
    }
    pub fn template(&self)->TreeRoom{
        let mut out = self.clone();
        out.template_iterate();
        return out;
    } 
}

pub fn purge_not_on_top(modified:&Vec<Room>, base: &Vec<Room> )->Vec<Room>{
    let mut out:Vec<Room> = Vec::new();
    for room in modified{
        let s = room.corners();
        let mut applicable = 0;
        for corner in s{
            if inside_set(corner, base, &room){
                applicable += 1;
            }
        }
        if applicable == 4{
            out.push(room.clone());
        }
    }
    return out;
}
pub fn _new_building(radmin:usize, radmax:usize) ->Vec<Room>{
    let mut r = TreeRoom::new(0,0, 800, 800);
    r.split(8);
    r.drop_random(0, radmin as i32, radmax as i32);
    return r.flatten();
}
pub fn _new_floor(previous: &Vec<Room>)->Vec<Room>{
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
pub fn _render_rooms_debug(rooms: &Vec<Room>,prev_floor:&Vec<Room>, handle: &mut RaylibDrawHandle){
    for i in rooms{
        i._render_debug(prev_floor, handle);
    }
}
