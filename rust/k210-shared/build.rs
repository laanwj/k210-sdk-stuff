use std::{env, fs};
use std::path::PathBuf;
use std::io::Write;

fn main() {
    // Put the linker script somewhere the linker can find it
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    println!("cargo:rustc-link-search={}", out_dir.display());

    fs::File::create(out_dir.join("memory.x")).unwrap()
        .write_all(include_bytes!("memory.x")).unwrap();
    println!("cargo:rerun-if-changed=memory.x");
}
