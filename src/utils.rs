use raylib::prelude::*;
pub fn draw_rectangle(handle: &mut RaylibDrawHandle, x:i32, y:i32, height:i32, width:i32){
    let cy =y;
    handle.draw_line(x, cy,x+width, cy ,Color::BLACK);
    handle.draw_line(x, cy,x, cy+height ,Color::BLACK);
    handle.draw_line(x, cy+height,x+width, cy+height ,Color::BLACK);
    handle.draw_line(x+width, cy,x+width, cy+height ,Color::BLACK);
}
pub unsafe fn draw_rectangle_unsafe(x:i32, y:i32, height:i32,width:i32){
    let lt = to_vec_2_unsafe(x, y);
    let rt = to_vec_2_unsafe(x+width, y);
    let lb = to_vec_2_unsafe(x, y+height);
    let rb = to_vec_2_unsafe(x+width, y+height);
    let t = 4 as f32;
    rust_raylib::ffi::DrawLineEx(lt.clone(), rt.clone(),t, rust_raylib::ffi::colors::BLACK);
    rust_raylib::ffi::DrawLineEx(lt.clone(), lb.clone(),t, rust_raylib::ffi::colors::BLACK);
    rust_raylib::ffi::DrawLineEx(rt.clone(), rb.clone(),t, rust_raylib::ffi::colors::BLACK);
    rust_raylib::ffi::DrawLineEx(rb.clone(), lb.clone(),t, rust_raylib::ffi::colors::BLACK);
}
pub fn draw_rectangle_centered(handle:&mut RaylibDrawHandle, center:&Vector2, height:i32, width:i32){
    let c = (*center)-Vector2{x:(width/2 )as f32, y:(height/2) as f32};
    handle.draw_rectangle_v(c, Vector2{x:width as f32, y:height as f32}, Color::BLACK);
}
pub unsafe fn draw_rectangle_centered_unsafe(center:&Vector2, height:i32, width:i32){
    let c = (*center)-Vector2{x:(width/2 )as f32, y:(height/2) as f32};
    rust_raylib::ffi::DrawRectangle(c.x as i32, c.y as i32, width, height, rust_raylib::ffi::colors::BLACK);
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
    if max == min{
        return max;
    }
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
pub unsafe fn to_vec_2_unsafe(x:i32, y:i32)->rust_raylib::ffi::Vector2{
    return rust_raylib::ffi::Vector2{x:x as f32, y:y as f32};
}
pub fn vec_2(x:f32, y:f32)->Vector2{
    return Vector2 { x: x, y: y };
}
pub fn vec_2_unsafe(x:f32, y:f32)->rust_raylib::ffi::Vector2{
    return rust_raylib::ffi::Vector2 { x: x, y: y };
}
pub fn render_stairs(handle:&mut RaylibDrawHandle,location:Vector2){
    let rad = 12.0;
    let cont = 7;
    let mut prev = location;
    for i in 0..cont{
        let theta = (i as f32)/(cont as f32)*5.0/6.0*2.0*3.1415;
        let loc2 = vec_2(rad*theta.cos(), rad*theta.sin())+location;
        let thick = (cont-i) as f32/2.0 as f32+1.0;
        handle.draw_line_ex(location,loc2, thick,Color::BLACK);
        if i>0{
            handle.draw_line_ex(prev,loc2, 1.0,Color::BLACK);
        }
        prev = loc2;
    }
}
pub unsafe fn convert_to_unsafe_vector(v:Vector2)->rust_raylib::ffi::Vector2{
    return rust_raylib::ffi::Vector2{x:v.x, y:v.y};
}
pub unsafe fn render_stairs_unsafe(loc:Vector2){
    let location = convert_to_unsafe_vector(loc);
    let rad = 12.0;
    let cont = 7;
    let mut prev =location.clone();
    for i in 0..cont{
        let theta = (i as f32)/(cont as f32)*5.5/6.0*2.0*3.1415;
        let loc2 = vec_2_unsafe(rad*theta.cos()+location.clone().x, rad*theta.sin()+location.clone().y);
        let thick = (cont-i) as f32/3.0 as f32+0.9;
        rust_raylib::ffi::DrawLineEx(location.clone(),loc2.clone(), thick,rust_raylib::ffi::colors::BLACK);
        if i>0{
            rust_raylib::ffi::DrawLineEx(prev,loc2.clone(), 1.0,rust_raylib::ffi::colors::BLACK);
        }
        prev = loc2;
    }
} 