pub mod drawables;
pub mod drawing;
pub mod font;
pub mod icons;

pub use drawing::{set_all, split_byte, to_landscape};

#[derive(Clone, Copy, Debug)]
pub struct Point {
    x: i32,
    y: i32,
}

pub fn deg_to_direction(winddirection: f32) -> String {
    if winddirection >= 348.75 || winddirection < 11.25 {
        return String::from("N");
    }
    if winddirection >= 11.25 && winddirection < 33.75 {
        return String::from("NNE");
    }
    if winddirection >= 33.75 && winddirection < 56.25 {
        return String::from("NE");
    }
    if winddirection >= 56.25 && winddirection < 78.75 {
        return String::from("ENE");
    }
    if winddirection >= 78.75 && winddirection < 101.25 {
        return String::from("E");
    }
    if winddirection >= 101.25 && winddirection < 123.75 {
        return String::from("ESE");
    }
    if winddirection >= 123.75 && winddirection < 146.25 {
        return String::from("SE");
    }
    if winddirection >= 146.25 && winddirection < 168.75 {
        return String::from("SSE");
    }
    if winddirection >= 168.75 && winddirection < 191.25 {
        return String::from("S");
    }
    if winddirection >= 191.25 && winddirection < 213.75 {
        return String::from("SSW");
    }
    if winddirection >= 213.75 && winddirection < 236.25 {
        return String::from("SW");
    }
    if winddirection >= 236.25 && winddirection < 258.75 {
        return String::from("WSW");
    }
    if winddirection >= 258.75 && winddirection < 281.25 {
        return String::from("W");
    }
    if winddirection >= 281.25 && winddirection < 303.75 {
        return String::from("WNW");
    }
    if winddirection >= 303.75 && winddirection < 326.25 {
        return String::from("NW");
    }
    if winddirection >= 326.25 && winddirection < 348.75 {
        return String::from("NNW");
    }
    return String::from("?");
}
