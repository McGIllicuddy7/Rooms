

use raylib::{ffi::sqrt, prelude::*};
use rand;
struct TreeRoom{
    pub x: i32,
    pub y: i32, 
    pub height :i32, 
    pub width : i32, 
    pub dropped: bool,
    child_2 : Option<Box<TreeRoom>>,
    child_1 : Option<Box<TreeRoom>>
}
struct Room{
    pub x:i32, 
    pub y:i32, 
    pub height:i32, 
    pub width:i32,
}
fn draw_rectangle(handle: &mut RaylibDrawHandle, x:i32, y:i32, height:i32, width:i32){
    handle.draw_line(x, y,x+width, y ,Color::BLACK);
    handle.draw_line(x, y,x, y+height ,Color::BLACK);
    handle.draw_line(x, y+height,x+width, y+height ,Color::BLACK);
    handle.draw_line(x+width, y,x+width, y+height ,Color::BLACK);
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
        let dx :f64 = (cx-400.0)*(cx-400.0);
        let dy :f64 = (cy-400.0)*(cy-400.0);
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
            if self.height <10 || self.width<10{
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
    pub fn render(&mut self, handle: &mut RaylibDrawHandle){
        if self.is_bottom(){
            if !self.dropped{
                draw_rectangle(handle, self.x, self.y, self.height, self.width);
            }
            return;
        }
        if self.child_1.is_some(){
            self.child_1.as_mut().as_mut().unwrap().render(handle);
        }
        if self.child_2.is_some(){
            self.child_2.as_mut().as_mut().unwrap().render(handle);
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
            return vec![self.to_room()];
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
    pub fn generate_hallways(&mut self){

    }
}

fn main() {
    let mut r = TreeRoom::new(0,0, 800, 800);
    r.split(11);
    r.drop_random(0, 150, 300);
    r.generate_hallways();
    let a = r.flatten();
    raylib::set_trace_log(TraceLogLevel::LOG_ERROR);
    let (mut rl, thread) = raylib::init()
        .size(800, 800)
        .title("Hello, World")
        .build();
     
    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::WHITE);
        r.render(&mut d);
    }
}
