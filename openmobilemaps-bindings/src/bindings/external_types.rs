// Copyright (c) 2023 Ubique Innovation AG <https://www.ubique.ch>
// 
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
use autocxx::prelude::*;
use crate::ffi::*;

#[derive(Default)]
pub struct Tiled2dMapLayerConfigWrapperImpl;

impl Tiled2dMapLayerConfigWrapperImpl {
    pub fn getCoordinateSystemIdentifier(&self) -> cxx::UniquePtr<cxx::CxxString> {
        CoordinateSystemIdentifiers::EPSG3857()
    }

    pub fn getTileUrl(&self, x: i32, y: i32, t: i32, zoom: i32) -> cxx::UniquePtr<cxx::CxxString> {
        // println!("getTIle url");
        let the_url = format!("https://a.tile.openstreetmap.org/{zoom}/{x}/{y}.png");
        // println!("{the_url}");
        make_string(&the_url)
    }

    pub fn getZoomLevelInfos(&self) -> cxx::UniquePtr<cxx::CxxVector<Tiled2dMapZoomLevelInfo>> {
        let mut zoom_infos = make_vec_zoom_level_info();
        let epsg3857Bounds = RectCoord::new(
            Coord::new(
                CoordinateSystemIdentifiers::EPSG3857(),
                -20037508.34,
                20037508.34,
                0.0,
            )
            .within_unique_ptr(),
            Coord::new(
                CoordinateSystemIdentifiers::EPSG3857(),
                20037508.34,
                -20037508.34,
                0.0,
            )
            .within_unique_ptr(),
        )
        .within_unique_ptr();
        add_zoom_level_info(
            zoom_infos.pin_mut(),
            Tiled2dMapZoomLevelInfo::new(
                559082264.029,
                40075016.0,
                1,
                1,
                1,
                0,
                &epsg3857Bounds,
            )
            .within_unique_ptr()
            .pin_mut(),
        );
        add_zoom_level_info(
            zoom_infos.pin_mut(),
            Tiled2dMapZoomLevelInfo::new(
                279541132.015,
                20037508.0,
                2,
                2,
                1,
                1,
                &epsg3857Bounds,
            )
            .within_unique_ptr()
            .pin_mut(),
        );
        add_zoom_level_info(
            zoom_infos.pin_mut(),
            Tiled2dMapZoomLevelInfo::new(
                139770566.007,
                10018754.0,
                4,
                4,
                1,
                2,
                &epsg3857Bounds,
            )
            .within_unique_ptr()
            .pin_mut(),
        );
        add_zoom_level_info(
            zoom_infos.pin_mut(),
            Tiled2dMapZoomLevelInfo::new(
                69885283.0036,
                5009377.1,
                8,
                8,
                1,
                3,
                &epsg3857Bounds,
            )
            .within_unique_ptr()
            .pin_mut(),
        );
        add_zoom_level_info(
            zoom_infos.pin_mut(),
            Tiled2dMapZoomLevelInfo::new(
                34942641.5018,
                2504688.5,
                16,
                16,
                1,
                4,
                &epsg3857Bounds,
            )
            .within_unique_ptr()
            .pin_mut(),
        );
        add_zoom_level_info(
            zoom_infos.pin_mut(),
            Tiled2dMapZoomLevelInfo::new(
                17471320.7509,
                1252344.3,
                32,
                32,
                1,
                5,
                &epsg3857Bounds,
            )
            .within_unique_ptr()
            .pin_mut(),
        );
        add_zoom_level_info(
            zoom_infos.pin_mut(),
            Tiled2dMapZoomLevelInfo::new(
                4367830.18773,
                313086.1,
                128,
                128,
                1,
                7,
                &epsg3857Bounds,
            )
            .within_unique_ptr()
            .pin_mut(),
        );
        zoom_infos
    }

    pub fn getZoomInfo(&self) -> cxx::UniquePtr<Tiled2dMapZoomInfo> {
        Tiled2dMapZoomInfo::new(1.0, 0, true, false, true, true).within_unique_ptr()
    }

    pub fn getLayerName(&self) -> cxx::UniquePtr<cxx::CxxString> {
        make_string("fancy_pancy")
    }
}