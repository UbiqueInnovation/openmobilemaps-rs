// Copyright (c) 2023 Ubique Innovation AG <https://www.ubique.ch>
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
use crate::ffi::*;
use autocxx::prelude::*;

use super::impls::DefaultLoaderInterface;
use crate::{LoaderInterfaceTrait, Tiled2dMapLayerConfigTrait};

pub struct LoaderInterfaceWrapperImpl(pub Box<dyn LoaderInterfaceTrait>);

impl Default for LoaderInterfaceWrapperImpl {
    fn default() -> Self {
        Self(Box::new(DefaultLoaderInterface))
    }
}

impl LoaderInterfaceWrapperImpl {
    pub fn loadTextureWrapper(
        &self,
        url: &cxx::CxxString,
        etag: cxx::UniquePtr<cxx::CxxString>,
    ) -> cxx::UniquePtr<TextureLoaderResult> {
        self.0.loadTextureWrapper(url, etag)
    }

    pub fn loadDataWrapper(
        &self,
        url: &cxx::CxxString,
        etag: cxx::UniquePtr<cxx::CxxString>,
    ) -> cxx::UniquePtr<DataLoaderResult> {
        self.0.loadDataWrapper(url, etag)
    }
}

// #[derive(Default)]
pub struct Tiled2dMapLayerConfigWrapperImpl(pub Box<dyn Tiled2dMapLayerConfigTrait>);

impl Tiled2dMapLayerConfigWrapperImpl {
    pub fn getCoordinateSystemIdentifier(&self) -> cxx::UniquePtr<cxx::CxxString> {
        CoordinateSystemIdentifiers::EPSG3857()
    }

    pub fn getTileUrl(&self, x: i32, y: i32, t: i32, zoom: i32) -> cxx::UniquePtr<cxx::CxxString> {
        self.0.getTileUrl(x, y, t, zoom)
    }

    pub fn getZoomLevelInfos(&self) -> cxx::UniquePtr<cxx::CxxVector<Tiled2dMapZoomLevelInfo>> {
        self.0.getZoomLevelInfos()
    }

    pub fn getZoomInfo(&self) -> cxx::UniquePtr<Tiled2dMapZoomInfo> {
        self.0.getZoomInfo()
    }

    pub fn getLayerName(&self) -> cxx::UniquePtr<cxx::CxxString> {
        self.0.getLayerName()
    }
}
