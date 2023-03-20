// Copyright (c) 2023 Ubique Innovation AG <https://www.ubique.ch>
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
#pragma once

#include "SchedulerInterface.h"
#include "Coord.h"
#include "CoordinateSystemIdentifiers.h"
#include "LayerInterface.h"
#include "MapCallbackInterface.h"
#include "MapReadyCallbackInterface.h"
#include "PolygonCoord.h"
#include "SchedulerInterfaceStaticWrapper.h"
#include "TaskConfig.h"
#include "TaskInterface.h"
#include "TextureHolderInterface.h"
#include "TextureLoaderResult.h"
#include "Tiled2dMapRasterLayerInterface.h"
#include "Tiled2dMapZoomLevelInfo.h"
#include "IconInfoInterface.h"
#include <iostream>
#include <memory>
#include <string>

#include <chrono>
#include <thread>


template <typename T>
std::shared_ptr<T> transform_unique_internal(std::unique_ptr<T> &ptr)
{
    return ptr;
}

std::shared_ptr<SchedulerInterface> transform_unique(std::unique_ptr<SchedulerInterfaceStaticWrapper>);
std::shared_ptr<TextureHolderInterface> transform_texture_holder_interface(std::unique_ptr<TextureHolderInterface> ptr);
std::shared_ptr<MapReadyCallbackInterface> transform_ready_state(std::unique_ptr<MapReadyCallbackInterface> ptr) { return ptr; }

std::unique_ptr<TextureLoaderResult> make_loader_result(std::shared_ptr<TextureHolderInterface>, LoaderStatus status);
std::shared_ptr<IconInfoInterface> transform_icon_info_interface(std::unique_ptr<IconInfoInterface> ptr) { return ptr;  }
std::shared_ptr<LayerInterface> down_cast_to_layer_interface(std::shared_ptr<Tiled2dMapRasterLayerInterface> ptr);

std::shared_ptr<MapCallbackInterface> to_map_callback_interface_shared_pointer(std::unique_ptr<MapCallbackInterface> interface)
{
    return interface;
}

inline std::vector<Tiled2dMapZoomLevelInfo> make_vec_zoom_level_info() { return std::vector<Tiled2dMapZoomLevelInfo>(); }

inline void add_zoom_level_info(std::vector<Tiled2dMapZoomLevelInfo> &zoomLevels, Tiled2dMapZoomLevelInfo &zoomLevel)
{
    zoomLevels.push_back(zoomLevel);
}
inline std::string get_id(std::shared_ptr<TaskInterface> interface) { return interface->getConfig().id; }
inline void run_task(std::shared_ptr<TaskInterface> interface)
{
    auto config = interface->getConfig();
    std::this_thread::sleep_for(std::chrono::milliseconds(config.delay));
    interface->run();
}

inline bool is_graphics(std::shared_ptr<TaskInterface> interface)
{
    return interface->getConfig().executionEnvironment == ExecutionEnvironment::GRAPHICS;
}

// inline std::vector<> get_loader_list(Tiled2dMapRasterLayerInterface li) { li-> }

inline std::unique_ptr<PolygonCoord> make_polygon_coord()
{
    std::vector<Coord> coords;

    coords.push_back(Coord(CoordinateSystemIdentifiers::EPSG2056(), 2684200.0, 1244833.3, 0.0));
    coords.push_back(Coord(CoordinateSystemIdentifiers::EPSG2056(), 2684200.0, 1345833.3, 0.0));
    coords.push_back(Coord(CoordinateSystemIdentifiers::EPSG2056(), 2785200.0, 1345833.3, 0.0));
    coords.push_back(Coord(CoordinateSystemIdentifiers::EPSG2056(), 2785200.0, 1244833.3, 0.0));

    std::vector<std::vector<Coord>> holes;

    return std::make_unique<PolygonCoord>(coords, holes);
}

class PolygonCoordBuilder
{
    std::vector<Coord> coords;
    std::vector<std::vector<Coord>> holes;

public:
    PolygonCoordBuilder() : coords(), holes(){};
    void addCoord(Coord coord)
    {
        coords.push_back(coord);
    }
    void addNewHoles(std::vector<Coord> holes)
    {
        this->holes.push_back(holes);
    }
    static void addHole(std::vector<Coord> &holes, Coord newHole)
    {
        holes.push_back(newHole);
    }
    std::vector<Coord> new_holes()
    {
        return std::vector<Coord>();
    }
    std::unique_ptr<PolygonCoord> build()
    {
        return std::make_unique<PolygonCoord>(coords, holes);
    }
};

