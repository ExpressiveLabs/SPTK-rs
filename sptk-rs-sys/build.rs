use cxx_build::CFG;

fn main() {
    CFG.include_prefix = "sptk-rs";

    let header_files = walkdir::WalkDir::new("SPTK/include")
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_dir())
        .map(|f| f.path().to_path_buf());

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
        .map(|f| f.path().to_path_buf());

    cxx_build::bridge("src/lib.rs")
        .std("c++14")
        .includes(header_files)
        .includes(third_party_headers)
        .files(source_files)
        .compile("hello_world_rust");

    println!("cargo:rerun-if-changed=src/lib.rs");
    println!("cargo:rerun-if-changed=cxx/math.hpp");
}
