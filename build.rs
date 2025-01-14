use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    let lib_name = if cfg!(target_os = "windows") {
        "libmemtrack.dll"
    } else if cfg!(target_os = "macos") {
        "libmemtrack.dylib"
    } else {
        "libmemtrack.so"
    };

    let profile = std::env::var("PROFILE").unwrap();

    let mut lib_path = PathBuf::from("./target").join(profile).join("deps").join(lib_name);

    if let Some(out_dir) = env::var("CARGO_HOME").ok() {
        let out_dir = PathBuf::from(out_dir).join("bin");
        fs::create_dir_all(&out_dir).expect("Failed to create output directory");

        let target_path = out_dir.join(lib_name);
        fs::copy(&lib_path, &target_path).expect("Failed to copy the dynamic library");
        lib_path = target_path;
    }


    println!("cargo:rustc-env=LIB_PATH={}", lib_path.display());
    println!("cargo:rerun-if-changed={}", lib_path.display());
}