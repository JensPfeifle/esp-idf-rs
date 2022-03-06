use epd_gfx;

use esp_idf_sys::{vTaskDelay, TickType_t};

pub mod epd;
pub mod epd_highlevel;
pub mod firasans;

unsafe fn delay() {
    //https://github.com/espressif/esp-idf/issues/1646#issuecomment-913190625
    let delay: TickType_t = 500;
    vTaskDelay(delay);
}

#[no_mangle]
extern "C" fn app_main() {
    println!("initializing...");
    let mut epd = epd::Epd::new();
    epd.init();
    epd.clear();

    println!("drawing...");
    let mut fb = epd.get_framebuffer().unwrap();
    epd_gfx::set_all(&mut fb, 0xFF);
    epd_gfx::fill_rect(&mut fb, 50, 75, 400, 250, 0x0);
    epd_gfx::fill_rect(&mut fb, 200, 150, 400, 250, 0x8);
    epd_gfx::font::draw_text(&mut fb, 0, 0, "Hello from RustType!", 32);

    epd.update_screen(25i32);

    println!("looping...");
    loop {
        unsafe { delay() };
    }
}
