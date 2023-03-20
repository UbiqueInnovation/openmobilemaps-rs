// Copyright (c) 2023 Ubique Innovation AG <https://www.ubique.ch>
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#include "make_shared.h"

std::shared_ptr<SchedulerInterface> transform_unique(std::unique_ptr<SchedulerInterfaceStaticWrapper> ptr)
{
    std::shared_ptr<SchedulerInterfaceStaticWrapper> thePtr = std::move(ptr);
    return std::dynamic_pointer_cast<SchedulerInterface>(thePtr);
}

std::shared_ptr<TextureHolderInterface> transform_texture_holder_interface(std::unique_ptr<TextureHolderInterface> ptr)
{
    return ptr;
};

std::unique_ptr<TextureLoaderResult> make_loader_result(std::shared_ptr<TextureHolderInterface> ptr, LoaderStatus status)
{
    return std::make_unique<TextureLoaderResult>(ptr, std::nullopt, status, std::nullopt);
}

std::shared_ptr<LayerInterface> down_cast_to_layer_interface(std::shared_ptr<Tiled2dMapRasterLayerInterface> ptr)
{
    return ptr->asLayerInterface();
}