/// Build script for C++ components of pcode-rs
use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    let cwd = env::current_dir().unwrap().to_string_lossy().to_string();

    let ghidra_out_dir = PathBuf::from(env::var("OUT_DIR").unwrap()).join("ghidra");
    let ghidra_build_dir = PathBuf::from(cwd.clone()).join("ghidra-build");
    let ghidra_out_path = ghidra_out_dir.to_str().unwrap();
    let ghidra_build_path = ghidra_build_dir.to_str().unwrap();
    println!("cargo:rustc-link-search=native={}/lib", ghidra_out_path);

    Command::new("meson")
        .arg(ghidra_build_path.clone())
        .arg(format!("-Dinstall_dir={}", ghidra_out_path))
        .current_dir(cwd.clone())
        .status()
        .expect("Could not configure ghidra libraries.");

    Command::new("meson")
        .arg("compile")
        .arg("-C")
        .arg(ghidra_build_path.clone())
        .current_dir(cwd.clone())
        .status()
        .expect("Could not build ghidra libraries.");

    Command::new("meson")
        .arg("install")
        .arg("-C")
        .arg(ghidra_build_path.clone())
        .current_dir(cwd.clone())
        .status()
        .expect("Could not install ghidra libraries.");
}
