use std::env;
use std::process::Command;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    Command::new("clang")
        .args([
            "native/test.m",
            "-fno-objc-arc",
            "-shared",
            "-fPIC",
            "-framework",
            "Foundation",
            "-o",
        ])
        .arg(&format!("{}/libblock_muttest.dylib", out_dir))
        .status()
        .unwrap();
    println!("cargo:rustc-link-search=native={}", out_dir);
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=Cargo.lock");
    println!("cargo:rerun-if-changed=native/test.m");
}
