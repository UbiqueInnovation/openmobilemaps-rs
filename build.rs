use std::{fs, io, path::Path};

use glob::glob;

fn main() -> miette::Result<()> {
    let public = std::path::PathBuf::from("src/maps-core/shared/public"); // include path
    let src = std::path::PathBuf::from("src/maps-core/shared/src");
    let graphics = std::path::PathBuf::from("src/maps-core/shared/src/graphics");
    let rust = std::path::PathBuf::from("src/rust");
    let mut b = autocxx_build::Builder::new("src/main.rs", &[&public, &src, &graphics, &rust])
        .extra_clang_args(&["-std=c++20"])
        .custom_gendir("./cxx".into())
        .build()?;
    // This assumes all your C++ bindings are in main.rs
    b.flag_if_supported("-std=c++20")
        .compile("openmobilemaps-rs"); // arbitrary library name, pick anything
    std::fs::remove_dir_all("./src/cxx").expect("Cleanup of old bindings did not work");
    copy_dir_all("./cxx", "./src/cxx").expect("Could not copy directory");
    println!("cargo:rerun-if-changed=src/main.rs");
    println!("cargo:rustc-link-lib=openmobilemaps-cxx");
    Ok(())
}

fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> io::Result<()> {
    fs::create_dir_all(&dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            copy_dir_all(entry.path(), dst.as_ref().join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(())
}
