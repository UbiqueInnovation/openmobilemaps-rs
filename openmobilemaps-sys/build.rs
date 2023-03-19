// Copyright (c) 2023 Ubique Innovation AG <https://www.ubique.ch>
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
use glob::glob;

fn main() {
    std::env::set_var("NUM_JOBS", "10");
    println!("cargo:rerun-if-changed=../src/maps-core");
    println!("cargo:rerun-if-changed=../src/rust");
    println!("cargo:rerun-if-changed=../src/cxx");
    cc::Build::new()
        .flag_if_supported("-std=c++20")
        .cpp(true)
        .files(
            glob("../src/maps-core/shared/**/*.cpp")
                .unwrap()
                .filter_map(|a| a.ok()),
        )
        .files(glob("../src/rust/**/*.cpp").unwrap().filter_map(|a| a.ok()))
        .files(
            glob("../src/maps-core/external/earcut/earcut/include/*.hpp")
                .unwrap()
                .filter_map(|a| a.ok()),
        )
        .files(
            glob("../src/maps-core/external/protozero/protozero/include/*.hpp")
                .unwrap()
                .filter_map(|a| a.ok()),
        )
        .files(
            glob("../src/maps-core/external/vtzero/vtzero/include/*.hpp")
                .unwrap()
                .filter_map(|a| a.ok()),
        )
        .include("../src/cxx/include")
        .include("../src/rust/cpp/graphics/objects")
        .include("../src/rust/cpp/graphics/shader")
        .include("../src/rust/cpp/graphics")
        .include("../src/rust/cpp")
        .include("../src/rust")
        .include("../src/maps-core/external/earcut/earcut/glfw/include")
        .include("../src/maps-core/shared/src/external/gpc")
        .include("../src/maps-core/shared/src/external/pugixml")
        .include("../src/maps-core/external/protozero/protozero/include")
        .include("../src/maps-core/external/protozero/protozero/test/include")
        .include("../src/maps-core/external/vtzero/vtzero/include")
        .include("../src/maps-core/external/earcut/earcut/include/mapbox")
        .include("../src/maps-core/external/earcut/earcut/include")
        .include("../src/maps-core/shared/public")
        .include("../src/maps-core/shared/src")
        .include("../src/maps-core/shared/src/graphics")
        .include("../src/maps-core/shared/src/map/coordinates")
        .include("../src/maps-core/shared/src/map/layers/tiled/vector/sublayers/raster")
        .include("../src/maps-core/shared/src/map/layers/tiled/vector/sublayers/background")
        .include("../src/maps-core/shared/src/map/layers/tiled/vector/sublayers/line")
        .include("../src/maps-core/shared/src/map/layers/tiled/vector/sublayers/polygon")
        .include("../src/maps-core/shared/src/map/layers/tiled/vector/sublayers/symbol")
        .include("../src/maps-core/shared/src/map/layers/icon")
        .include("../src/maps-core/shared/src/map/layers/text")
        .compile("openmobilemaps-cxx");
    println!("cargo:rustc-link-lib=framework=OpenGL");
}
