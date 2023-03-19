// Copyright (c) 2023 Ubique Innovation AG <https://www.ubique.ch>
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#pragma once

#include "LoaderInterface.h"
#include "Tiled2dMapLayerConfig.h"
#include "Tiled2dMapRasterLayerInterface.h"
#include <memory>
#include <vector>

class Tiled2dMapRasterLayerInterfaceBuilder {
    std::vector<std::shared_ptr<LoaderInterface>> loaders;
    std::shared_ptr<Tiled2dMapLayerConfig> layerConfig;

    Tiled2dMapRasterLayerInterfaceBuilder()
        : loaders()
        , layerConfig() {}

  public:
    static Tiled2dMapRasterLayerInterfaceBuilder builder() { return Tiled2dMapRasterLayerInterfaceBuilder(); }

    void addLoader(std::shared_ptr<LoaderInterface> loader) { this->loaders.push_back(loader); }
    void setConfig(std::shared_ptr<Tiled2dMapLayerConfig> layerConfig) { this->layerConfig = layerConfig; }

    std::shared_ptr<Tiled2dMapRasterLayerInterface> build() {
        return Tiled2dMapRasterLayerInterface::create(this->layerConfig, this->loaders);
    }
};