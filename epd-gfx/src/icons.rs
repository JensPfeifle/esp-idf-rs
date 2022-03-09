// Symbols are drawn on a relative 10x10grid and 1 scale unit = 1 drawing unit

const BLACK: u8 = 0x0;
const WHITE: u8 = 0xF;
const Large: u32 = 28; // For icon drawing, needs to be odd number for best effect
const Small: u32 = 8; // 6  For icon drawing, needs to be odd number for best effect

use crate::{draw_line, draw_pixel, fill_circle, fill_rect, fill_triangle, Point};

#[derive(PartialEq, Clone, Copy)]
pub enum IconSize {
    SMALL,
    LARGE,
}

pub fn addmoon(fb: &mut [u8], x: i32, y: i32, scale: f32, size: IconSize) {
    if size == IconSize::LARGE {
        fill_circle(fb, x - 85, y - 100, (scale * 0.8) as i32, BLACK);
        fill_circle(fb, x - 57, y - 100, (scale * 1.6) as i32, WHITE);
    } else {
        fill_circle(fb, x - 28, y - 37, (scale * 1.0) as i32, BLACK);
        fill_circle(fb, x - 20, y - 37, (scale * 1.6) as i32, WHITE);
    }
}

pub fn addcloud(fb: &mut [u8], x: i32, y: i32, scale: f32, linesize: u32) {
    // Draw cloud outer
    fill_circle(fb, x - scale as i32 * 3, y, scale as i32, BLACK); // Left most circle
    fill_circle(fb, x + scale as i32 * 3, y, scale as i32, BLACK); // Right most circle
    fill_circle(
        fb,
        x - scale as i32,
        y - scale as i32,
        (scale * 1.4) as i32,
        BLACK,
    ); // left middle upper circle
    fill_circle(
        fb,
        x + (scale * 1.5) as i32,
        y - (scale * 1.3) as i32,
        (scale * 1.75) as i32,
        BLACK,
    ); // Right middle upper circle
    fill_rect(
        fb,
        x - scale as i32 * 3 - 1,
        y - scale as i32,
        scale as i32 * 6,
        scale as i32 * 2 + 1,
        BLACK,
    ); // Upper and lower lines
       // Clear cloud inner
    fill_circle(
        fb,
        x - scale as i32 * 3,
        y,
        (scale - linesize as f32) as i32,
        WHITE,
    ); // Clear left most circle
    fill_circle(
        fb,
        x + scale as i32 * 3,
        y,
        (scale - linesize as f32) as i32,
        WHITE,
    ); // Clear right most circle
    fill_circle(
        fb,
        x - scale as i32,
        y - scale as i32,
        ((scale * 1.4) - (linesize as f32)) as i32,
        WHITE,
    ); // left middle upper circle
    fill_circle(
        fb,
        x + (scale * 1.5) as i32,
        y - (scale * 1.3) as i32,
        ((scale * 1.75) - (linesize as f32)) as i32,
        WHITE,
    ); // Right middle upper circle
    fill_rect(
        fb,
        x - scale as i32 * 3 + 2,
        y - scale as i32 + linesize as i32 - 1,
        (scale * 5.9) as i32,
        ((scale * 2.0) - (linesize as f32) + 2.0) as i32,
        WHITE,
    ); // Upper and lower lines
}

pub fn addraindrop(fb: &mut [u8], mut x: i32, mut y: i32, scale: f32) {
    fill_circle(fb, x, y, scale as i32 / 2, BLACK);
    fill_triangle(
        fb,
        Point {
            x: x - (scale / 2.0) as i32,
            y,
        },
        Point {
            x,
            y: y - (scale * 1.2) as i32,
        },
        Point {
            x: x + (scale / 2.0) as i32,
            y,
        },
        BLACK,
    );
    x = x + (scale * 1.6) as i32;
    y = y + (scale / 3.0) as i32;
    fill_circle(fb, x, y, (scale / 2.0) as i32, BLACK);
    fill_triangle(
        fb,
        Point {
            x: x - (scale / 2.0) as i32,
            y,
        },
        Point {
            x,
            y: y - (scale * 1.2) as i32,
        },
        Point {
            x: x + (scale / 2.0) as i32,
            y,
        },
        BLACK,
    );
}

pub fn addrain(fb: &mut [u8], x: i32, y: i32, mut scale: f32, size: IconSize) {
    if size == IconSize::SMALL {
        scale = scale * 1.34;
    }
    for d in 0..4 {
        addraindrop(
            fb,
            x + (scale * (7.8 - d as f32 * 1.95) - (scale * 5.2)) as i32,
            y + (scale * 2.1 - scale / 6.0) as i32,
            scale / 1.6,
        );
    }
}

pub fn addsnow(fb: &mut [u8], x: i32, y: i32, scale: f32, size: IconSize) {
    let mut dxo;
    let mut dyo;
    let mut dxi;
    let mut dyi;
    for flakes in 0..5 {
        for i in (0..360).step_by(45) {
            dxo = 0.5 * scale * ((i as f32 - 90.0) * 3.14 / 180.0).cos();
            dxi = dxo * 0.1;
            dyo = 0.5 * scale * ((i as f32 - 90.0) * 3.14 / 180.0).sin();
            dyi = dyo * 0.1;
            draw_line(
                fb,
                (dxo + x as f32 + (flakes as f32 * 1.5 - 3.0) * scale) as i32,
                (dyo + y as f32 + scale * 2.0) as i32,
                (dxi + x as f32 + (flakes as f32 * 1.5 - 3.0) * scale) as i32,
                (dyi + y as f32 + scale * 2.0) as i32,
                BLACK,
            );
        }
    }
}
//
//pub fn addtstorm(fb: &mut [u8], x: i32, mut y: i32, scale: f32) {
//    y = y + scale / 2;
//    //for i in 0..5 {
//    //    draw_line(
//    //        fb,
//    //        x - scale * 4 + scale * i * 1.5 + 0,
//    //        y + (scale * 1.5) as u32,
//    //        x - (scale * 3.5) as u32 + scale * i * 1.5 + 0,
//    //        y + scale,
//    //        BLACK,
//    //    );
//    //    if (scale != Small) {
//    //        draw_line(
//    //            fb,
//    //            x - scale * 4 + scale * i * 1.5 + 1,
//    //            y + (scale * 1.5) as u32,
//    //            x - (scale * 3.5) as u32 + scale * i * 1.5 + 1,
//    //            y + scale,
//    //            BLACK,
//    //        );
//    //        draw_line(
//    //            fb,
//    //            x - scale * 4 + scale * i * 1.5 + 2,
//    //            y + (scale * 1.5) as u32,
//    //            x - (scale * 3.5) as u32 + scale * i * 1.5 + 2,
//    //            y + scale,
//    //            BLACK,
//    //        );
//    //    }
//    //    draw_line(
//    //        fb,
//    //        x - scale * 4 + scale * i * 1.5,
//    //        y + scale * 1.5 + 0,
//    //        x - scale * 3 + scale * i * 1.5 + 0,
//    //        y + scale * 1.5 + 0,
//    //        BLACK,
//    //    );
//    //    if (scale != Small) {
//    //        draw_line(
//    //            fb,
//    //            x - scale * 4 + scale * i * 1.5,
//    //            y + scale * 1.5 + 1,
//    //            x - scale * 3 + scale * i * 1.5 + 0,
//    //            y + scale * 1.5 + 1,
//    //            BLACK,
//    //        );
//    //        draw_line(
//    //            fb,
//    //            x - scale * 4 + scale * i * 1.5,
//    //            y + scale * 1.5 + 2,
//    //            x - scale * 3 + scale * i * 1.5 + 0,
//    //            y + scale * 1.5 + 2,
//    //            BLACK,
//    //        );
//    //    }
//    //    draw_line(
//    //        fb,
//    //        x - scale * 3.5 + scale * i * 1.4 + 0,
//    //        y + scale * 2.5,
//    //        x - scale * 3 + scale * i * 1.5 + 0,
//    //        y + scale * 1.5,
//    //        BLACK,
//    //    );
//    //    if (scale != Small) {
//    //        draw_line(
//    //            fb,
//    //            x - scale * 3.5 + scale * i * 1.4 + 1,
//    //            y + scale * 2.5,
//    //            x - scale * 3 + scale * i * 1.5 + 1,
//    //            y + scale * 1.5,
//    //            BLACK,
//    //        );
//    //        draw_line(
//    //            fb,
//    //            x - scale * 3.5 + scale * i * 1.4 + 2,
//    //            y + scale * 2.5,
//    //            x - scale * 3 + scale * i * 1.5 + 2,
//    //            y + scale * 1.5,
//    //            BLACK,
//    //        );
//    //    }
//    //}
//}
//
pub fn addsun(fb: &mut [u8], x: i32, y: i32, scale: f32, size: IconSize) {
    if scale <= 0.0 {
        return;
    };

    let linesize = match size {
        IconSize::SMALL => 1,
        IconSize::LARGE => 3,
    };

    fill_rect(
        fb,
        x - scale as i32 * 2,
        y,
        scale as i32 * 4,
        linesize,
        BLACK,
    );
    fill_rect(
        fb,
        x,
        y - scale as i32 * 2,
        linesize,
        scale as i32 * 4,
        BLACK,
    );
    draw_line(
        fb,
        x - (scale * 1.3) as i32,
        y - (scale * 1.3) as i32,
        x + (scale * 1.3) as i32,
        y + (scale * 1.3) as i32,
        BLACK,
    );
    draw_line(
        fb,
        x - (scale * 1.3) as i32,
        y + (scale * 1.3) as i32,
        x + (scale * 1.3) as i32,
        y - (scale * 1.3) as i32,
        BLACK,
    );
    if size == IconSize::LARGE {
        draw_line(
            fb,
            1 + x - (scale * 1.3) as i32,
            y - (scale * 1.3) as i32,
            1 + x + (scale * 1.3) as i32,
            y + (scale * 1.3) as i32,
            BLACK,
        );
        draw_line(
            fb,
            2 + x - (scale * 1.3) as i32,
            y - (scale * 1.3) as i32,
            2 + x + (scale * 1.3) as i32,
            y + (scale * 1.3) as i32,
            BLACK,
        );
        draw_line(
            fb,
            3 + x - (scale * 1.3) as i32,
            y - (scale * 1.3) as i32,
            3 + x + (scale * 1.3) as i32,
            y + (scale * 1.3) as i32,
            BLACK,
        );
        draw_line(
            fb,
            1 + x - (scale * 1.3) as i32,
            y + (scale * 1.3) as i32,
            1 + x + (scale * 1.3) as i32,
            y - (scale * 1.3) as i32,
            BLACK,
        );
        draw_line(
            fb,
            2 + x - (scale * 1.3) as i32,
            y + (scale * 1.3) as i32,
            2 + x + (scale * 1.3) as i32,
            y - (scale * 1.3) as i32,
            BLACK,
        );
        draw_line(
            fb,
            3 + x - (scale * 1.3) as i32,
            y + (scale * 1.3) as i32,
            3 + x + (scale * 1.3) as i32,
            y - (scale * 1.3) as i32,
            BLACK,
        );
    }
    fill_circle(fb, x, y, (scale * 1.3) as i32, WHITE);
    fill_circle(fb, x, y, scale as i32, BLACK);
    fill_circle(fb, x, y, scale as i32 - linesize, WHITE);
}
//
//pub fn addfog(fb: &mut [u8], x: i32, mut y: i32, scale: f32, mut linesize: u32, size: IconSize) {
//    if (size == IconSize::SMALL) {
//        y -= 10;
//        linesize = 1;
//    }
//    for i in 0..6 {
//        fill_rect(
//            fb,
//            x - scale * 3,
//            y + (scale * 1.5) as i32,
//            scale * 6,
//            linesize,
//            BLACK,
//        );
//        fill_rect(
//            fb,
//            x - scale * 3,
//            y + (scale * 2.0) as i32,
//            scale * 6,
//            linesize,
//            BLACK,
//        );
//        fill_rect(
//            fb,
//            x - scale * 3,
//            y + (scale * 2.5) as i32,
//            scale * 6,
//            linesize,
//            BLACK,
//        );
//    }
//}
//

pub fn sunny(fb: &mut [u8], x: i32, mut y: i32, size: IconSize, name: &str) {
    let scale: f32 = match size {
        IconSize::SMALL => Small as f32,
        IconSize::LARGE => Large as f32,
    };
    if size == IconSize::SMALL {
        y = y - 3; // Shift up small sun icon
    }
    if name.ends_with("n") {
        addmoon(fb, x, y + 3, scale, size);
    }
    addsun(fb, x, y, scale * 1.6, size);
}

pub fn mostly_sunny(fb: &mut [u8], x: i32, y: i32, size: IconSize, name: &str) {
    let mut linesize = 3;
    let mut offset = 5;

    let scale: f32;

    match size {
        IconSize::LARGE => {
            scale = Large as f32;
            offset = 10;
        }
        IconSize::SMALL => {
            scale = Small as f32;
            linesize = 1;
        }
    }
    if name.ends_with("n") {
        addmoon(fb, x, y + offset, scale, size);
    }
    addcloud(fb, x, y + offset, scale, linesize);
    addsun(
        fb,
        x - (scale * 1.8) as i32,
        y - (scale * 1.8) as i32 + offset,
        scale,
        size,
    );
}

pub fn mostly_cloudy(fb: &mut [u8], x: i32, y: i32, size: IconSize, name: &str) {
    let scale: f32;
    let linesize;
    match size {
        IconSize::LARGE => {
            scale = Large as f32;
            linesize = 3;
        }
        IconSize::SMALL => {
            scale = Small as f32;
            linesize = 1;
        }
    }

    if name.ends_with("n") {
        addmoon(fb, x, y, scale, size);
    }
    addcloud(fb, x, y, scale, linesize);
    addsun(
        fb,
        x - (scale * 1.8) as i32,
        y - (scale * 1.8) as i32,
        scale,
        size,
    );
    addcloud(fb, x, y, scale, linesize);
}

pub fn cloudy(fb: &mut [u8], x: i32, y: i32, size: IconSize, name: &str) {
    let scale: f32;
    let linesize;
    let offset;
    match size {
        IconSize::LARGE => {
            scale = Large as f32;
            linesize = 3;
            offset = 10;
        }
        IconSize::SMALL => {
            scale = Small as f32;
            linesize = 1;
            offset = 0;
        }
    }

    if name.ends_with("n") {
        addmoon(fb, x, y + offset, scale, size);
    }

    addcloud(fb, x, y, scale, linesize); // Main cloud
    if size == IconSize::LARGE {
        addcloud(fb, x + 30, y, 8.0, linesize); // Cloud top right
        addcloud(fb, x - 40, y - 80, 10.0, linesize); // Cloud top left
    }
}

pub fn rain(fb: &mut [u8], x: i32, y: i32, size: IconSize, name: &str) {
    let scale;
    let linesize;
    match size {
        IconSize::SMALL => {
            scale = Small as f32;
            linesize = 1;
        }
        IconSize::LARGE => {
            scale = Large as f32;
            linesize = 3;
        }
    }
    if name.ends_with("n") {
        addmoon(fb, x, y, scale, size);
    }
    addcloud(fb, x, y, scale, linesize);
    addrain(fb, x, y, scale, size);
}

//pub fn ExpectRain(fb: &mut [u8], x: i32, y: i32, size: IconSize, name: &str) {
//    let mut scale: f32 = Large;
//    let mut linesize = 3;
//    if (size == IconSize::SMALL) {
//        scale = Small;
//        linesize = 1;
//    }
//    if (name.ends_with("n")) {
//        addmoon(fb, x, y, scale, size);
//    }
//    addsun(
//        fb,
//        x - (scale * 1.8) as i32,
//        y - (scale * 1.8) as i32,
//        scale,
//        size,
//    );
//    addcloud(fb, x, y, scale, linesize);
//    addrain(fb, x, y, scale, size);
//}
//
//pub fn ChanceRain(fb: &mut [u8], x: i32, y: i32, size: IconSize, name: &str) {
//    let mut scale: f32 = Large;
//    let mut linesize = 3;
//    if (size == IconSize::SMALL) {
//        scale = Small;
//        linesize = 1;
//    }
//    if (name.ends_with("n")) {
//        addmoon(fb, x, y, scale, size);
//    }
//    addsun(
//        fb,
//        x - (scale * 1.8) as i32,
//        y - (scale * 1.8) as i32,
//        scale,
//        size,
//    );
//    addcloud(fb, x, y, scale, linesize);
//    addrain(fb, x, y, scale, size);
//}
//
//pub fn Tstorms(fb: &mut [u8], x: i32, y: i32, size: IconSize, name: &str) {
//    let mut scale: f32 = Large;
//    let mut linesize = 3;
//    if (size == IconSize::SMALL) {
//        scale = Small;
//        linesize = 1;
//    }
//    if (name.ends_with("n")) {
//        addmoon(fb, x, y, scale, size);
//    }
//    addcloud(fb, x, y, scale, linesize);
//    addtstorm(fb, x, y, scale);
//}
//
//pub fn Snow(fb: &mut [u8], x: i32, y: i32, size: IconSize, name: &str) {
//    let mut scale: f32 = Large;
//    let mut linesize = 3;
//    if (size == IconSize::SMALL) {
//        scale = Small;
//        linesize = 1;
//    }
//    if (name.ends_with("n")) {
//        addmoon(fb, x, y, scale, size);
//    }
//    addcloud(fb, x, y, scale, linesize);
//    addsnow(fb, x, y, scale, size);
//}
//
//pub fn Fog(fb: &mut [u8], x: i32, y: i32, size: IconSize, name: &str) {
//    let mut linesize = 3;
//    let mut scale = Large;
//    if (size == IconSize::SMALL) {
//        scale = Small;
//        linesize = 1;
//    }
//    if (name.ends_with("n")) {
//        addmoon(fb, x, y, scale, size);
//    }
//    addcloud(fb, x, y - 5, scale, linesize);
//    addfog(fb, x, y - 5, scale, linesize, size);
//}
//
//pub fn Haze(fb: &mut [u8], x: i32, y: i32, size: IconSize, name: &str) {
//    let mut linesize = 3;
//    let mut scale = Large;
//    if (size == IconSize::SMALL) {
//        scale = Small;
//        linesize = 1;
//    }
//    if (name.ends_with("n")) {
//        addmoon(fb, x, y, scale, size);
//    }
//    addsun(fb, x, y - 5, (scale * 1.4) as u32, size);
//    addfog(fb, x, y - 5, (scale * 1.4) as u32, linesize, size);
//}
//
//pub fn CloudCover(fb: &mut [u8], x: i32, y: i32, CCover: u32) {
//    addcloud(fb, x - 9, y - 3, (Small as f32 * 0.8) as i32, 2); // Cloud top left
//    addcloud(fb, x + 3, y - 3, (Small as f32 * 0.8) as i32, 2); // Cloud top right
//    addcloud(fb, x, y, (Small as f32 * 0.8) as u32, 2); // Main cloud
//                                                        //drawString(x + 44, y - 14, String(CCover) + "%", EPD_DRAW_ALIGN_LEFT);
//}
//
//pub fn Visibility(fb: &mut [u8], x: i32, mut y: i32, visibility: &str) {
//    y = y - 3;
//    let mut start_angle = 0.52;
//    let mut end_angle = 2.61;
//    let r = 14;
//    //for i in (start_angle..end_angle).step_by(0.05) {
//    //    draw_pixel(fb, x + r * i.cos(), y - r / 2 + r * i.sin(), BLACK);
//    //    draw_pixel(fb, x + r * i.cos(), 1 + y - r / 2 + r * i.sin(), BLACK);
//    //}
//    //start_angle = 3.61;
//    //end_angle = 5.78;
//    //for i in (start_angle..end_angle).step_by(0.05) {
//    //    draw_pixel(fb, x + r * i.cos(), y + r / 2 + r * i.sin(), BLACK);
//    //    draw_pixel(fb, x + r * i.cos(), 1 + y + r / 2 + r * i.sin(), BLACK);
//    //}
//    fill_circle(fb, x, y, r / 4, BLACK);
//    //drawString(x + 14, y - 12, Visi, EPD_DRAW_ALIGN_LEFT);
//}
//
////pub fn DisplayGeneralInfoSection()
////{
////
////    drawString(4, 24, City, EPD_DRAW_ALIGN_LEFT);
////    drawString(540 / 2, 24, Date_str, EPD_DRAW_ALIGN_CENTER);
////    drawString(480, 24, "@" + Time_str, EPD_DRAW_ALIGN_RIGHT);
////    draw_line(fb,0, 24, 540, 24, GxEPD_BLACK);
////}
////
////pub fn DisplayConditionsSection(fb: &mut [u8], x: u32, y: u32, name: &str , size: IconSize)
////{
////    if (IconName == "01d" || IconName == "01n")
////        Sunny(x, y, IconSize, IconName);
////    else if (IconName == "02d" || IconName == "02n")
////        MostlySunny(x, y, IconSize, IconName);
////    else if (IconName == "03d" || IconName == "03n")
////        Cloudy(x, y, IconSize, IconName);
////    else if (IconName == "04d" || IconName == "04n")
////        MostlySunny(x, y, IconSize, IconName);
////    else if (IconName == "09d" || IconName == "09n")
////        ChanceRain(x, y, IconSize, IconName);
////    else if (IconName == "10d" || IconName == "10n")
////        Rain(x, y, IconSize, IconName);
////    else if (IconName == "11d" || IconName == "11n")
////        Tstorms(x, y, IconSize, IconName);
////    else if (IconName == "13d" || IconName == "13n")
////        Snow(x, y, IconSize, IconName);
////    else if (IconName == "50d")
////        Haze(x, y, IconSize, IconName);
////    else if (IconName == "50n")
////        Fog(x, y, IconSize, IconName);
////    if (size == IconSize::LARGE)
////    {
////        drawString(x + 360, y - 74, String(WxConditions[0].Humidity, 0) + "%", EPD_DRAW_ALIGN_CENTER);
////        if (WxConditions[0].Visibility > 0)
////            Visibility(x - 100, y + 130, String(WxConditions[0].Visibility) + "M");
////        if (WxConditions[0].Cloudcover > 0)
////            CloudCover(x + 60, y + 130, WxConditions[0].Cloudcover);
////    }
////}
////pub fn DisplayTemperatureSection(fb: &mut [u8], x: u32, y: u32, twidth: u32, tdepth: u32)
////{
////    drawString(x, y, String(WxConditions[0].Temperature, 1) + "°C", EPD_DRAW_ALIGN_CENTER, &OpenSans16);
////    drawString(x, y + 40, String(WxConditions[0].High, 0) + "° | " + String(WxConditions[0].Low, 0) + "°", EPD_DRAW_ALIGN_CENTER, &OpenSans16);
////}
////
////pub fn DisplayPressureSection(fb: &mut [u8], x: u32, y: u32, pwidth: u32, pdepth: u32, pressure: f32, slope: &str)
////{
////    pressure = pressure * 0.750062; // convert to mmhg
////    drawString(x, y, String(pressure, (Units == "M" ? 0 : 1)) + (Units == "M" ? "mm" : "in"), EPD_DRAW_ALIGN_LEFT);
////}
////
////pub fn DisplayPrecipitationSection(fb: &mut [u8], x: u32, y: u32, pwidth: u32, pdepth: u32)
////{
////    if (WxForecast[1].Rainfall >= 0.005)
////    {
////        drawString(x, y + 80, String(WxForecast[1].Rainfall, 2) + (Units == "M" ? "mm" : "in"), EPD_DRAW_ALIGN_LEFT);
////        addraindrop(fb, x + 102, y + 84, 7);
////    }
////    if (WxForecast[1].Snowfall >= 0.005)
////    {
////        drawString(x, y + 110, String(WxForecast[1].Snowfall, 2) + (Units == "M" ? "mm" : "in") + " **", EPD_DRAW_ALIGN_LEFT);
////    }
////}
////
////pub fn DisplayMainWeatherSection(fb: &mut [u8], x: u32, y: u32)
////{ // (x=500, y=190)
////    DisplayConditionsSection(x, y, WxConditions[0].Icon, IconSize::LARGE);
////    DisplayTemperatureSection(x + 230, y - 30, 180, 170);
////    DisplayPressureSection(x + 160, y + 70, 180, 170, WxConditions[0].Pressure, WxConditions[0].Trend);
////    DisplayPrecipitationSection(x + 268, y - 8, 181, 170);
////}
////
////pub fn DisplayForecastWeather(fb: &mut [u8], x: u32, y: u32, index: usize)
////{
////    int fwidth = 103;
////    x = x + fwidth * (index - 1);
////    DisplayConditionsSection(x + fwidth / 2, y + 90, WxForecast[index].Icon, SmallIcon);
////    drawString(x + fwidth / 2 - 10, y + 30, String(WxForecast[index].Period.substring(11, 16)), EPD_DRAW_ALIGN_CENTER);
////    drawString(x + fwidth / 2 + 0, y + 130, String(WxForecast[index].High, 0) + "°/" + String(WxForecast[index].Low, 0) + "°", EPD_DRAW_ALIGN_CENTER);
////}
////
////pub fn DisplayForecastSection(fb: &mut [u8], x: u32, y: u32)
////{
////    int f = 1;
////    do
////    {
////        DisplayForecastWeather(x, y, f);
////        f++;
////    } while (f < max_readings);
////}
////
////pub fn WindDegToDirection(winddirection: f32) -> String
////{
////    if (winddirection >= 348.75 || winddirection < 11.25)
////        return TXT_N;
////    if (winddirection >= 11.25 && winddirection < 33.75)
////        return TXT_NNE;
////    if (winddirection >= 33.75 && winddirection < 56.25)
////        return TXT_NE;
////    if (winddirection >= 56.25 && winddirection < 78.75)
////        return TXT_ENE;
////    if (winddirection >= 78.75 && winddirection < 101.25)
////        return TXT_E;
////    if (winddirection >= 101.25 && winddirection < 123.75)
////        return TXT_ESE;
////    if (winddirection >= 123.75 && winddirection < 146.25)
////        return TXT_SE;
////    if (winddirection >= 146.25 && winddirection < 168.75)
////        return TXT_SSE;
////    if (winddirection >= 168.75 && winddirection < 191.25)
////        return TXT_S;
////    if (winddirection >= 191.25 && winddirection < 213.75)
////        return TXT_SSW;
////    if (winddirection >= 213.75 && winddirection < 236.25)
////        return TXT_SW;
////    if (winddirection >= 236.25 && winddirection < 258.75)
////        return TXT_WSW;
////    if (winddirection >= 258.75 && winddirection < 281.25)
////        return TXT_W;
////    if (winddirection >= 281.25 && winddirection < 303.75)
////        return TXT_WNW;
////    if (winddirection >= 303.75 && winddirection < 326.25)
////        return TXT_NW;
////    if (winddirection >= 326.25 && winddirection < 348.75)
////        return TXT_NNW;
////    return "?";
////}
////
////pub fn DisplayDisplayWindSection(fb: &mut [u8], x: u32, y: u32, angle: f32, windspeed: f32, Cradius: u32)
////{
////    arrow(x, y, Cradius - 22, angle, 18, 33); // Show wind direction on outer circle of width and length
////    // int dxo, dyo, dxi, dyi;
////    // drawCircle(x, y, Cradius, GxEPD_BLACK);       // Draw compass circle
////    // drawCircle(x, y, Cradius + 1, GxEPD_BLACK);   // Draw compass circle
////    // drawCircle(x, y, Cradius * 0.7, GxEPD_BLACK); // Draw compass inner circle
////    // for (float a = 0; a < 360; a = a + 22.5)
////    //{
////    //     dxo = Cradius * cos((a - 90) * PI / 180);
////    //     dyo = Cradius * sin((a - 90) * PI / 180);
////    //     if (a == 45)
////    //         drawString(dxo + x + 27, dyo + y - 20, TXT_NE, EPD_DRAW_ALIGN_CENTER);
////    //     if (a == 135)
////    //         drawString(dxo + x + 27, dyo + y - 2, TXT_SE, EPD_DRAW_ALIGN_CENTER);
////    //     if (a == 225)
////    //         drawString(dxo + x - 43, dyo + y - 2, TXT_SW, EPD_DRAW_ALIGN_CENTER);
////    //     if (a == 315)
////    //         drawString(dxo + x - 43, dyo + y - 20, TXT_NW, EPD_DRAW_ALIGN_CENTER);
////    //     dxi = dxo * 0.9;
////    //     dyi = dyo * 0.9;
////    //     draw_line(fb,dxo + x, dyo + y, dxi + x, dyi + y, GxEPD_BLACK);
////    //     dxo = dxo * 0.7;
////    //     dyo = dyo * 0.7;
////    //     dxi = dxo * 0.9;
////    //     dyi = dyo * 0.9;
////    //     draw_line(fb,dxo + x, dyo + y, dxi + x, dyi + y, GxEPD_BLACK);
////    // }
////    // drawString(x - 3, y - Cradius - 30, TXT_N, EPD_DRAW_ALIGN_CENTER);
////    // drawString(x - 5, y + Cradius + 18, TXT_S, EPD_DRAW_ALIGN_CENTER);
////    // drawString(x - Cradius - 27, y - 11, TXT_W, EPD_DRAW_ALIGN_CENTER);
////    // drawString(x + Cradius + 15, y - 11, TXT_E, EPD_DRAW_ALIGN_CENTER);
////    drawString(x - 12, y - 57, WindDegToDirection(angle), EPD_DRAW_ALIGN_CENTER);
////    drawString(x + 3, y + 50, String(angle, 0) + "°", EPD_DRAW_ALIGN_CENTER);
////    drawString(x + 3, y - 16, String(windspeed, 1), EPD_DRAW_ALIGN_CENTER);
////    drawString(x + 16, y - 12, (Units == "M" ? "m/s" : "mph"), EPD_DRAW_ALIGN_LEFT);
////}
////
