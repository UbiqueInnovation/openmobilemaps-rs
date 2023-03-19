use std::{fs, io, path::Path};

fn main() -> miette::Result<()> {
    let start = env!("CARGO_MANIFEST_DIR");

    let public = std::path::PathBuf::from(format!("../maps-core/shared/public")); // include path
    let src = std::path::PathBuf::from(format!("../maps-core/shared/src"));
    let graphics = std::path::PathBuf::from(format!("../maps-core/shared/src/graphics"));
    let rust = std::path::PathBuf::from("src/rust");
    let mut b =
        autocxx_build::Builder::new(format!("src/lib.rs"), &[&public, &src, &graphics, &rust])
            .extra_clang_args(&["-std=c++20"])
            .custom_gendir(format!("../cxx").into())
            .build()?;

    let _ = std::fs::remove_dir_all("./src/cxx");
    copy_dir_all("../cxx", "./src/cxx").expect("Could not copy directory");

    b.flag_if_supported("-std=c++20")
        .define("__OPENGL__", "1")
        .compile("openmobilemaps-bindings-sys-cxx"); // arbitrary library name, pick anything

    println!("cargo:rerun-if-changed=src/bindings/external_types.rs");
    println!("cargo:rerun-if-changed=src/generated.rs");
    println!("cargo:rerun-if-changed=src/lib.rs");
    println!("cargo:rerun-if-changed=src/bindings/impls.rs");
    println!("cargo:rerun-if-changed=src/bindings/manual.rs");
    println!("cargo:rustc-link-lib=openmobilemaps-bindings-sys-cxx");
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
