use raylib::prelude::*;
pub fn draw_rectangle(handle: &mut RaylibDrawHandle, x:i32, y:i32, height:i32, width:i32){
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