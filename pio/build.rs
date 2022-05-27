use std::env;
use std::path::{Path, PathBuf};

use embuild::{
    self, bingen,
    build::{CfgArgs, LinkArgs},
    cargo, symgen,
};

fn main() -> anyhow::Result<()> {
    let target = env::var("TARGET").unwrap();
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    let header = "./components/epdiy/include/epd_highlevel.h";
    let out = out_dir.join("epd_highlevel.rs");
    run_bindgen(&target, header, &out);

    // Necessary because of this issue: https://github.com/rust-lang/cargo/issues/9641
    LinkArgs::output_propagated("ESP_IDF")?;
    let cfg = CfgArgs::try_from_env("ESP_IDF")?;
    cfg.output();

    Ok(())
}

fn run_bindgen(target: &str, header: &str, out: &Path) {
    let mut builder = bindgen::Builder::default();
    builder = builder.header(header).clang_args([
        "-I./.pio/build/debug/config/", // sdkconfig.h
        "-I/opt/esp/idf/components/esp_common/include",
        "-I/opt/esp/idf/components/xtensa/include",
        "-I/opt/esp/tools/xtensa-esp32-elf/esp-2021r2-patch3-8.4.0/xtensa-esp32-elf/xtensa-esp32-elf/include/",
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
