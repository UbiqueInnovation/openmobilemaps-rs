// Copyright (c) 2023 Ubique Innovation AG <https://www.ubique.ch>
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#include "Tiled2dMapLayerConfigWrapper.h"
#include "cxxgen.h"
#include "cxxgen1.h"
#include "cxxgen2.h"
#include "cxxgen3.h"

Tiled2dMapLayerConfigWrapper::Tiled2dMapLayerConfigWrapper(Tiled2dMapLayerConfigWrapperImpl *ptr) : rustBox(::rust::Box<Tiled2dMapLayerConfigWrapperImpl>::from_raw(ptr))
{
}

std::string Tiled2dMapLayerConfigWrapper::getCoordinateSystemIdentifier() base_call(getCoordinateSystemIdentifier);

std::string Tiled2dMapLayerConfigWrapper::getTileUrl(int32_t x, int32_t y, int32_t t, int32_t zoom)
    base_call(getTileUrl, x, y, t, zoom);

std::vector<Tiled2dMapZoomLevelInfo> Tiled2dMapLayerConfigWrapper::getZoomLevelInfos() base_call(getZoomLevelInfos);

Tiled2dMapZoomInfo Tiled2dMapLayerConfigWrapper::getZoomInfo() base_call(getZoomInfo);

std::string Tiled2dMapLayerConfigWrapper::getLayerName() base_call(getLayerName);