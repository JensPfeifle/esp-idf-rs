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
        c::hello_c(0, 90);
    }
}
