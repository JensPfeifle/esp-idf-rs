pub mod epd_driver {
    #![allow(non_upper_case_globals)]
    #![allow(non_camel_case_types)]
    #![allow(non_snake_case)]
    include!(concat!(env!("OUT_DIR"), "/epd_driver.rs"));
}

pub mod epd_internals {
    #![allow(non_upper_case_globals)]
    #![allow(non_camel_case_types)]
    #![allow(non_snake_case)]
    include!(concat!(env!("OUT_DIR"), "/epd_internals.rs"));
}

pub mod epd;

#[no_mangle]
extern "C" fn app_main() {
    println!("Hello, world!");
    epd::init_and_clear();

    let mut state = epd::EpdState::new();

    state.epd_draw_hline(20, 20, 20, 0x00);
    state.epd_hl_update_screen(25);
}
