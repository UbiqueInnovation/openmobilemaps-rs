// Copyright (c) 2023 Ubique Innovation AG <https://www.ubique.ch>
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::{path::PathBuf, str::FromStr};

use anyhow::bail;
use cxx::{UniquePtr, SharedPtr};

use crate::*;

pub fn create_open_streetmap_raster_layer()  -> anyhow::Result<(SharedPtr<LoaderInterfaceImpl>, SharedPtr<LayerInterface>)> {
     let mut builder = Tiled2dMapRasterLayerInterfaceBuilder::builder().within_unique_ptr();

    if builder.is_null() {
        bail!("Failed to initialize raster layer builder");
    }
    let config_wrapper = unsafe {
        let wrapper = Tiled2dMapLayerConfigWrapperImpl(Box::new(OpenStreetmapZoomInfo));
        let pointer = Box::into_raw(Box::new(wrapper));
        Tiled2dMapLayerConfigWrapper::new1(pointer as _).within_unique_ptr()
    };

    if config_wrapper.is_null() {
        bail!("Failed to setup config wrapper");
    }

    let config = Tiled2dMapLayerConfigWrapper::asTiled2dMapLayerConfig(config_wrapper);

    builder.pin_mut().setConfig(config);

    let loader = LoaderInterfaceWrapperImpl::default();
    let pointer = Box::into_raw(Box::new(loader));
    let loader = unsafe { LoaderInterfaceImpl::new1(pointer as _).within_unique_ptr() };

    if loader.is_null() {
        bail!("Failed to initialize loader");
    }
    let loader = LoaderInterfaceImpl::toShared(loader);

    let loader_shared = LoaderInterfaceImpl::asLoaderInterface(loader.clone());
    builder.pin_mut().addLoader(loader_shared);

    let tiled = builder.pin_mut().build();
    Ok((loader, down_cast_to_layer_interface(tiled)))
}

pub struct OpenStreetmapZoomInfo;

impl Tiled2dMapLayerConfigTrait for OpenStreetmapZoomInfo {
    fn getCoordinateSystemIdentifier(&self) -> UniquePtr<cxx::CxxString> {
        CoordinateSystemIdentifiers::EPSG4326()
    }

    fn getTileUrl(&self, x: i32, y: i32, _t: i32, zoom: i32) -> UniquePtr<cxx::CxxString> {
        log::debug!("getTIle url");
        let p = PathBuf::from_str(&format!("tiles/{zoom}/{x}")).unwrap();
        if !p.exists() {
            std::fs::create_dir_all(p).unwrap();
        }
        let the_url = format!("https://a.tile.openstreetmap.org/{zoom}/{x}/{y}.png");
        log::debug!("{the_url}");
        make_string(&the_url)
    }

    fn getZoomLevelInfos(&self) -> UniquePtr<CxxVector<Tiled2dMapZoomLevelInfo>> {
        let mut zoom_infos = make_vec_zoom_level_info();
        let epsg_3857_bounds = RectCoord::new(
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
            Tiled2dMapZoomLevelInfo::new(559082264.029, 40075016.0, 1, 1, 1, 0, &epsg_3857_bounds)
                .within_unique_ptr()
                .pin_mut(),
        );
        add_zoom_level_info(
            zoom_infos.pin_mut(),
            Tiled2dMapZoomLevelInfo::new(279541132.015, 20037508.0, 2, 2, 1, 1, &epsg_3857_bounds)
                .within_unique_ptr()
                .pin_mut(),
        );
        add_zoom_level_info(
            zoom_infos.pin_mut(),
            Tiled2dMapZoomLevelInfo::new(139770566.007, 10018754.0, 4, 4, 1, 2, &epsg_3857_bounds)
                .within_unique_ptr()
                .pin_mut(),
        );
        add_zoom_level_info(
            zoom_infos.pin_mut(),
            Tiled2dMapZoomLevelInfo::new(69885283.0036, 5009377.1, 8, 8, 1, 3, &epsg_3857_bounds)
                .within_unique_ptr()
                .pin_mut(),
        );
        add_zoom_level_info(
            zoom_infos.pin_mut(),
            Tiled2dMapZoomLevelInfo::new(34942641.5018, 2504688.5, 16, 16, 1, 4, &epsg_3857_bounds)
                .within_unique_ptr()
                .pin_mut(),
        );
        add_zoom_level_info(
            zoom_infos.pin_mut(),
            Tiled2dMapZoomLevelInfo::new(17471320.7509, 1252344.3, 32, 32, 1, 5, &epsg_3857_bounds)
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
                &epsg_3857_bounds,
            )
            .within_unique_ptr()
            .pin_mut(),
        );
        add_zoom_level_info(
            zoom_infos.pin_mut(),
            Tiled2dMapZoomLevelInfo::new(
                2183915.09386,
                156543.0,
                256,
                256,
                1,
                8,
                &epsg_3857_bounds,
            )
            .within_unique_ptr()
            .pin_mut(),
        );
        add_zoom_level_info(
            zoom_infos.pin_mut(),
            Tiled2dMapZoomLevelInfo::new(1091957.54693, 78271.5, 512, 512, 1, 9, &epsg_3857_bounds)
                .within_unique_ptr()
                .pin_mut(),
        );
        add_zoom_level_info(
            zoom_infos.pin_mut(),
            Tiled2dMapZoomLevelInfo::new(
                545978.773466,
                39135.8,
                1024,
                1024,
                1,
                10,
                &epsg_3857_bounds,
            )
            .within_unique_ptr()
            .pin_mut(),
        );

        add_zoom_level_info(
            zoom_infos.pin_mut(),
            Tiled2dMapZoomLevelInfo::new(
                272989.386733,
                19567.9,
                2048,
                2048,
                1,
                11,
                &epsg_3857_bounds,
            )
            .within_unique_ptr()
            .pin_mut(),
        );

        add_zoom_level_info(
            zoom_infos.pin_mut(),
            Tiled2dMapZoomLevelInfo::new(
                136494.693366,
                9783.94,
                4096,
                4096,
                1,
                12,
                &epsg_3857_bounds,
            )
            .within_unique_ptr()
            .pin_mut(),
        );
        add_zoom_level_info(
            zoom_infos.pin_mut(),
            Tiled2dMapZoomLevelInfo::new(
                68247.3466832,
                4891.97,
                8192,
                8192,
                1,
                13,
                &epsg_3857_bounds,
            )
            .within_unique_ptr()
            .pin_mut(),
        );
        add_zoom_level_info(
            zoom_infos.pin_mut(),
            Tiled2dMapZoomLevelInfo::new(
                34123.6733416,
                2445.98,
                16384,
                16384,
                1,
                14,
                &epsg_3857_bounds,
            )
            .within_unique_ptr()
            .pin_mut(),
        );
        add_zoom_level_info(
            zoom_infos.pin_mut(),
            Tiled2dMapZoomLevelInfo::new(
                17061.8366708,
                1222.99,
                32768,
                32768,
                1,
                15,
                &epsg_3857_bounds,
            )
            .within_unique_ptr()
            .pin_mut(),
        );
        add_zoom_level_info(
            zoom_infos.pin_mut(),
            Tiled2dMapZoomLevelInfo::new(
                8530.91833540,
                611.496,
                65536,
                65536,
                1,
                16,
                &epsg_3857_bounds,
            )
            .within_unique_ptr()
            .pin_mut(),
        );
        add_zoom_level_info(
            zoom_infos.pin_mut(),
            Tiled2dMapZoomLevelInfo::new(
                4265.45916770,
                305.748,
                131072,
                131072,
                1,
                17,
                &epsg_3857_bounds,
            )
            .within_unique_ptr()
            .pin_mut(),
        );
        add_zoom_level_info(
            zoom_infos.pin_mut(),
            Tiled2dMapZoomLevelInfo::new(
                2132.72958385,
                152.874,
                262144,
                262144,
                1,
                18,
                &epsg_3857_bounds,
            )
            .within_unique_ptr()
            .pin_mut(),
        );
        add_zoom_level_info(
            zoom_infos.pin_mut(),
            Tiled2dMapZoomLevelInfo::new(
                1066.36479193,
                76.437,
                524288,
                524288,
                1,
                19,
                &epsg_3857_bounds,
            )
            .within_unique_ptr()
            .pin_mut(),
        );
        add_zoom_level_info(
            zoom_infos.pin_mut(),
            Tiled2dMapZoomLevelInfo::new(
                533.18239597,
                38.2185,
                1048576,
                1048576,
                1,
                20,
                &epsg_3857_bounds,
            )
            .within_unique_ptr()
            .pin_mut(),
        );
        zoom_infos
    }

    fn getZoomInfo(&self) -> UniquePtr<Tiled2dMapZoomInfo> {
        Tiled2dMapZoomInfo::new(1.0, 0, true, false, true, true).within_unique_ptr()
    }

    fn getLayerName(&self) -> UniquePtr<cxx::CxxString> {
        make_string("OpenStreetMapDefaultLayer")
    }
}
