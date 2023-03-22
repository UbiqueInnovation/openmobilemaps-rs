// Copyright (c) 2023 Ubique Innovation AG <https://www.ubique.ch>
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
use glob::glob;

fn main() {
    std::env::set_var("NUM_JOBS", "10");
    println!("cargo:rerun-if-changed=src/lib.rs");
    println!("cargo:rerun-if-changed=../maps-core");
    println!("cargo:rerun-if-changed=../openmobilemaps-bindings/src");
    let mut ccbuild = cc::Build::new();
    if cfg!(target_os = "macos") {
        ccbuild.define("__MAXOS_BUILD__", "1");
    }
    if cfg!(target_os = "linux") {
        ccbuild.define("__LINUX_BUILD__", "1");
    }
    ccbuild
        .flag_if_supported("-std=c++20")
        .cpp(true)
        .flag_if_supported("-include memory")
        .flag_if_supported("-include string")
        .flag_if_supported("-include cstring")
        .flag_if_supported("-include cmath")
        .flag_if_supported("-include mutex")
        .define("__OPENGL__", "1")
        .files(
            glob("../maps-core/shared/**/*.cpp")
                .unwrap()
                .filter_map(|a| a.ok()),
        )
        .files(
            glob("../openmobilemaps-bindings/src/rust/**/*.cpp")
                .unwrap()
                .filter_map(|a| a.ok()),
        )
        .files(
            glob("../maps-core/external/earcut/earcut/include/*.hpp")
                .unwrap()
                .filter_map(|a| a.ok()),
        )
        .files(
            glob("../maps-core/external/protozero/protozero/include/*.hpp")
                .unwrap()
                .filter_map(|a| a.ok()),
        )
        .files(
            glob("../maps-core/external/vtzero/vtzero/include/*.hpp")
                .unwrap()
                .filter_map(|a| a.ok()),
        )
        .include("../cxx/include")
        .include("../openmobilemaps-bindings/src/rust/cpp/graphics/objects")
        .include("../openmobilemaps-bindings/src/rust/cpp/graphics/shader")
        .include("../openmobilemaps-bindings/src/rust/cpp/graphics")
        .include("../openmobilemaps-bindings/src/rust/cpp")
        .include("../openmobilemaps-bindings/src/rust")
        .include("../maps-core/external/earcut/earcut/glfw/include")
        .include("../maps-core/shared/src/external/gpc")
        .include("../maps-core/shared/src/external/pugixml")
        .include("../maps-core/external/protozero/protozero/include")
        .include("../maps-core/external/protozero/protozero/test/include")
        .include("../maps-core/external/vtzero/vtzero/include")
        .include("../maps-core/external/earcut/earcut/include/mapbox")
        .include("../maps-core/external/earcut/earcut/include")
        .include("../maps-core/shared/public")
        .include("../maps-core/shared/src")
        .include("../maps-core/shared/src/graphics")
        .include("../maps-core/shared/src/map/coordinates")
        .include("../maps-core/shared/src/map/layers/tiled/vector/sublayers/raster")
        .include("../maps-core/shared/src/map/layers/tiled/vector/sublayers/background")
        .include("../maps-core/shared/src/map/layers/tiled/vector/sublayers/line")
        .include("../maps-core/shared/src/map/layers/tiled/vector/sublayers/polygon")
        .include("../maps-core/shared/src/map/layers/tiled/vector/sublayers/symbol")
        .include("../maps-core/shared/src/map/layers/icon")
        .include("../maps-core/shared/src/map/layers/text")
        .compile("openmobilemaps-cxx");
    if cfg!(target_os = "macos") {
        println!("cargo:rustc-link-lib=framework=OpenGL");
    }
    if cfg!(target_os = "linux") {
        println!("cargo:rustc-link-lib=GL");
        println!("cargo:rustc-link-lib=GLU");
        println!("cargo:rustc-link-lib=glut");
    }
}
