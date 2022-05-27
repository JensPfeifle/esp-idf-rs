pub mod epd;
pub mod epd_bindings;

#[no_mangle]
extern "C" fn app_main() {
    println!("Hello, world!")
}
