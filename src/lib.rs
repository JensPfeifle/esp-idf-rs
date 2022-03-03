pub mod clib {
    #![allow(non_upper_case_globals)]
    #![allow(non_camel_case_types)]
    #![allow(non_snake_case)]
    include!(concat!(env!("OUT_DIR"), "/clib.rs"));
}

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

    unsafe {
        clib::hello_c(1, 2);
    }

    println!("allocate");
    let mut state = Box::new(epd::EpdState::new());
    println!("draw");
    state.epd_draw_hline(20, 20, 20, 0x00);
    state.epd_fill_rect(0, 0, 200, 200, 0x00);
    state.epd_fill_rect(150, 150, 400, 400, 0xFF);

    println!("update");
    state.epd_hl_update_screen(25);

    let mut x = 0;
    println!("looping...");
    loop {
        x = x + 1;
    }
}
