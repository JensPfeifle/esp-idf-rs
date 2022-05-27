use std::env;
use std::path::{Path, PathBuf};

fn main() -> anyhow::Result<()> {
    let target = env::var("TARGET").unwrap();
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    let header = "./components/epdiy/include/epd_highlevel.h";
    let out = out_dir.join("epd_highlevel.rs");
    run_bindgen(&target, header, &out);

    Ok(())
}

fn run_bindgen(target: &str, header: &str, out: &Path) {
    let mut builder = bindgen::Builder::default();
    builder = builder.header(header).clang_args([
        "-I./.pio/build/debug/config/", // sdkconfig.h
        "-I/opt/esp/idf/components/esp_common/include", // esp_attr.h
        // bits/libc-header-start.h
        "-I/opt/esp/tools/xtensa-esp32-elf/esp-2021r2-patch3-8.4.0/xtensa-esp32-elf/xtensa-esp32-elf/include/",
    ]);
    match target {
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
