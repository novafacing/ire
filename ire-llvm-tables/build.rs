use anyhow::Result;
use cmake::Config;
use reqwest::blocking::get;
use std::env::var;
use std::fs::{copy, create_dir_all};
use std::io::BufReader;
use std::path::{Path, PathBuf};
use std::process::Command;
use tar::Archive;
use xz2::read::XzDecoder;

const LLVM_URL: &str = "https://github.com/llvm/llvm-project/releases/download/llvmorg-15.0.7/llvm-project-15.0.7.src.tar.xz";
const LLVM_SRC_DIR: &str = "llvm-project-15.0.7.src";

fn download_and_unpack(url: &str, dest: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let response = get(url)?;
    let data = response.bytes()?.to_vec();
    let buf_reader = BufReader::new(data.as_slice());
    let xz_reader = XzDecoder::new(buf_reader);
    let mut tar = Archive::new(xz_reader);

    tar.unpack(dest)?;

    Ok(())
}

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    let llvm_download_dir =
        PathBuf::from(var("OUT_DIR").expect("OUT_DIR not set")).join("llvm-project");
    let llvm_dir = llvm_download_dir.join(LLVM_SRC_DIR).join("llvm");
    let table_install_dir = PathBuf::from(var("OUT_DIR").expect("OUT_DIR not set")).join("tables");
    let llvm_table_paths: Vec<PathBuf> = vec![
        "lib/Target/AArch64/AArch64.td",
        // Don't need GPU
        // "llvm/lib/Target/AMDGPU/AMDGPU.td",
        "lib/Target/ARC/ARC.td",
        "lib/Target/ARM/ARM.td",
        "lib/Target/AVR/AVR.td",
        "lib/Target/BPF/BPF.td",
        "lib/Target/CSKY/CSKY.td",
        // Don't need GPU
        // "llvm/lib/Target/DirectX/DirectX.td",
        "lib/Target/Hexagon/Hexagon.td",
        "lib/Target/Lanai/Lanai.td",
        "lib/Target/LoongArch/LoongArch.td",
        "lib/Target/M68k/M68k.td",
        "lib/Target/Mips/Mips.td",
        "lib/Target/MSP430/MSP430.td",
        // Don't need GPU
        // "llvm/lib/Target/NVPTX/NVPTX.td",
        "lib/Target/PowerPC/PowerPC.td",
        "lib/Target/RISCV/RISCV.td",
        "lib/Target/Sparc/Sparc.td",
        "lib/Target/SPIRV/SPIRV.td",
        "lib/Target/SystemZ/SystemZ.td",
        // Don't need GPU
        // "llvm/lib/Target/VE/VE.td",
        "lib/Target/WebAssembly/WebAssembly.td",
        "lib/Target/X86/X86.td",
        "lib/Target/XCore/XCore.td",
    ]
    .iter()
    .map(|p| llvm_dir.join(p))
    .collect();

    download_and_unpack(&LLVM_URL, &llvm_download_dir).expect("Failed to download and unpack LLVM");

    create_dir_all(&table_install_dir).expect("Failed to create tables directory");

    llvm_table_paths
        .iter()
        .for_each(|table| match table.file_name() {
            Some(name) => {
                let table_out_json = table_install_dir.join(name).with_extension("json");
                let llvm_include_dir = llvm_dir.join("include");
                let target_dir = llvm_dir.join(table.parent().expect("Failed to get parent"));
                println!(
                    "Running llvm-tblgen -I {} --dump-json {:?} -o {} from {}",
                    llvm_include_dir.display(),
                    name,
                    table_out_json.display(),
                    target_dir.display()
                );
                Command::new("llvm-tblgen")
                    .arg("-I")
                    .arg(llvm_include_dir)
                    .arg("--dump-json")
                    .arg(name)
                    .arg("-o")
                    .arg(table_out_json)
                    .current_dir(target_dir)
                    .output()
                    .expect("Failed to run llvm-tablegen");
            }
            None => {}
        });

    // Rerun the build script if any of the tables change (or don't exist)
    llvm_table_paths.iter().for_each(|table| {
        println!("cargo:rerun-if-changed={}", table.display());
    });
}
