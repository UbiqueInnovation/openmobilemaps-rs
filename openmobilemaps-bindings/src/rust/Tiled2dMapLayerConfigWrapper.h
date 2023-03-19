// Copyright (c) 2023 Ubique Innovation AG <https://www.ubique.ch>
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
#pragma once

#include "Tiled2dMapLayerConfig.h"
#include "Tiled2dMapVectorSettings.h"
#include "Tiled2dMapZoomInfo.h"
#include <memory>

#define base_call(function, ...)                                                                                                   \
    {                                                                                                                              \
        auto inner = new_layer_config_inner_wrapper();                                                                             \
        return *(inner->function(__VA_ARGS__));                                                                                       \
    }

class Tiled2dMapLayerConfigWrapper : public Tiled2dMapLayerConfig {
  public:
    virtual std::optional<Tiled2dMapVectorSettings> getVectorSettings() override;

    virtual std::string getCoordinateSystemIdentifier() override;

    virtual std::string getTileUrl(int32_t x, int32_t y, int32_t t, int32_t zoom) override;

    virtual std::vector<Tiled2dMapZoomLevelInfo> getZoomLevelInfos() override;

    virtual Tiled2dMapZoomInfo getZoomInfo() override;

    virtual std::string getLayerName() override;

    // virtual std::unique_ptr<Tiled2dMapVectorSettings> getVectorSettingsWrapped() const = 0;

    static std::shared_ptr<Tiled2dMapLayerConfig> asTiled2dMapLayerConfig(std::unique_ptr<Tiled2dMapLayerConfigWrapper> myself) {
        std::shared_ptr<Tiled2dMapLayerConfigWrapper> ptr = std::move(myself);
        return std::dynamic_pointer_cast<Tiled2dMapLayerConfig>(ptr);
    }
};

std::optional<Tiled2dMapVectorSettings> Tiled2dMapLayerConfigWrapper::getVectorSettings() { return std::nullopt; }