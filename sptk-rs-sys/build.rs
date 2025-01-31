// use autocxx_build::CFG;

use std::path::PathBuf;

fn main() -> miette::Result<()> {
    let mut header_files = walkdir::WalkDir::new("SPTK/include")
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_dir())
        .map(|f| f.path().to_path_buf()).collect::<Vec<PathBuf>>();

    let source_files = walkdir::WalkDir::new("SPTK/src")
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .filter(|f| f.path().extension().unwrap_or_default() == "cc")
        .map(|f| f.path().to_path_buf());

    let third_party_headers = walkdir::WalkDir::new("SPTK/third_party")
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_dir())
        .map(|f| f.path().to_path_buf()).collect::<Vec<PathBuf>>();

    // autocxx_build::bridge("src/lib.rs")
    //     .std("c++14")
    //     .includes(header_files)
    //     .includes(third_party_headers)
    //     .files(source_files)
    //     .compile("hello_world_rust");

    header_files.extend(third_party_headers);

    println!("header_files: {:?}", header_files);

    let mut b = autocxx_build::Builder::new("src/bridge.rs", &header_files).build()?;
    b.flag_if_supported("-std=c++14").files(source_files)
     .compile("sptk-math"); // arbitrary library name, pick anything

    println!("cargo:rerun-if-changed=src/lib.rs");
    println!("cargo:rerun-if-changed=cxx/math.hpp");

    Ok(())
}