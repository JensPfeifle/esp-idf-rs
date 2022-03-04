use std::env;
use std::path::{Path, PathBuf};

fn main() {
    let target = env::var("TARGET").unwrap();
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    let header = "./components/clib/include/CApi.h";
    let out = out_dir.join("clib.rs");
    run_bindgen(&target, header, &out);

    let header = "./components/epdiy/include/epd_driver.h";
    let out = out_dir.join("epd_driver.rs");
    run_bindgen(&target, header, &out);

    let header = "./components/epdiy/include/epd_internals.h";
    let out = out_dir.join("epd_internals.rs");
    run_bindgen(&target, header, &out);
}

fn run_bindgen(target: &str, header: &str, out: &Path) {
    let mut builder = bindgen::Builder::default();
    builder = builder.header(header).clang_args([
        "-I./.pio/build/debug/config/", // sdkconfig.h
        "-I/root/.platformio/packages/framework-espidf/components/xtensa/include",
    ]);
    match target {
        "riscv32imc-esp-espidf" => {
            builder = builder.clang_arg("--target=riscv32");
            builder = builder.use_core();
            builder = builder.ctypes_prefix("crate::ffi");
        }
        "xtensa-esp32-espidf" => {
            // Make sure that LLVM_CONFIG_PATH has been set to point to the
            // Xtensa build of llvm-config.
            builder = builder.clang_arg("--target=xtensa-esp32-elf");
        }
        _ => {
            panic!("Unexpect target archtitecture: {}", &target);
        }
    }

    let bindings = builder.generate().expect("Couldn't generate bindings!");

    bindings
        .write_to_file(&out)
        .expect("Couldn't save bindings!");

    println!("cargo:rerun-if-changed={}", header);
    println!("cargo:rerun-if-changed={}", out.display());
}
