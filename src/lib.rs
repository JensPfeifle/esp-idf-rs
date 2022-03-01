use esp_idf_sys::{vTaskDelay, TickType_t};

pub mod epd;
pub mod epd_highlevel;

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
    epd.set_all(0xFF);
    epd.fill_rect(250, 100, 500, 300, 0x00);
    epd.update_screen(25i32);

    println!("looping...");
    loop {
        unsafe { delay() };
    }
}
