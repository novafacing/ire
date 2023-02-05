use anyhow::Result;
use cmake::Config;
use reqwest::blocking::get;
use std::env::var;
use std::fs::{create_dir_all, File};
use std::io::{BufReader, Read};
use std::path::{Path, PathBuf};
use tar::Archive;
use xz2::read::XzDecoder;

const LLVM_URL: &str = "https://github.com/llvm/llvm-project/releases/download/llvmorg-15.0.7/llvm-project-15.0.7.src.tar.xz";
const LLVM_SRC_DIR: &str = "llvm-project-15.0.7.src";

fn download_and_unpack(url: &str, dest: &Path) -> Result<(), Box<dyn std::error::Error>> {
    // Download the file
    let response = get(url)?;
    let data = response.bytes()?.to_vec();

    // Unpack the file
    let buf_reader = BufReader::new(data.as_slice());
    let xz_reader = XzDecoder::new(buf_reader);
    let mut tar = Archive::new(xz_reader);
    tar.unpack(dest)?;

    Ok(())
}

fn main() {
    let llvm_download_dir =
        PathBuf::from(var("OUT_DIR").expect("OUT_DIR not set")).join("llvm-project");
    let llvm_install_dir =
        PathBuf::from(var("OUT_DIR").expect("OUT_DIR not set")).join("llvm-install");
    download_and_unpack(&LLVM_URL, &llvm_download_dir).expect("Failed to download and unpack LLVM");
    // Create the install directory
    create_dir_all(&llvm_install_dir).expect("Failed to create tables directory");

    let llvm_dir = llvm_download_dir.join(LLVM_SRC_DIR).join("llvm");

    let config = Config::new(&llvm_dir)
        .define("CMAKE_BUILD_TYPE", "Debug")
        .build();
}
