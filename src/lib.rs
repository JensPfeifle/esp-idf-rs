pub mod c {
    #![allow(non_upper_case_globals)]
    #![allow(non_camel_case_types)]
    #![allow(non_snake_case)]
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

#[no_mangle]
extern "C" fn app_main() {
    println!("Hello, world!");
    unsafe {
        let options: c::EpdInitOptions = 0u32; // EPD_OPTIONS_DEFAULT
        c::epd_init(options);
        c::epd_clear();
    }
}
