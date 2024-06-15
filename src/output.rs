use std::{fs, io::Write};

use crate::building::Building;

pub fn output_floor(
    count: &mut usize,
    room_count: usize,
    floor_count: usize,
    name: &str,
) -> String {
    let mut out = format!("<h1>Floor {}</h1>\n <image src= \"{}{}.png\" alt = map of floor {} height:300 width:300>\n", floor_count+1, name, floor_count+1, floor_count+1);
    for i in 0..room_count {
        out += &format!(
            "        <h2>{}:</h2>
        <p2></p2>
        ",
            *count + i + 1
        );
        *count += 1;
    }
    return out;
}
pub fn output_building(b: &Building, name: &str) {
    let mut out = format!(
        "<!DOCTYPE html>
    <html>
    <head>
    <title>{}</title>
    </head>
    <h1>{}</h1>
    <body style=\"background-color: black;color:white;\">",
        name, name
    );
    let mut count = 0;
    for i in 0..b.floors.len() {
        out = out + &output_floor(&mut count, b.floors[i].len(), i, name);
    }
    out = out
        + "</body>
    </html>";
    let out_name = format!("output/{}.html", name);
    if let Ok(mut f) = fs::File::create(out_name) {
        let _ = f.write(out.as_bytes());
    }
}
