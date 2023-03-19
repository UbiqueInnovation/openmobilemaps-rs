// Copyright (c) 2023 Ubique Innovation AG <https://www.ubique.ch>
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#pragma once

#include "LoaderInterface.h"
#include <optional>

class LoaderInterfaceImpl : public LoaderInterface {
  public:
    virtual TextureLoaderResult loadTexture(const std::string &url, const std::optional<std::string> &etag) const  override;
    virtual DataLoaderResult loadData(const std::string &url, const std::optional<std::string> &etag) const  override;
    virtual TextureLoaderResult loadTextureWrapper(const std::string &url, const std::string etag) const = 0;
    virtual DataLoaderResult loadDataWrapper(const std::string &url, const std::string etag) const = 0;

    static std::shared_ptr<LoaderInterfaceImpl> toShared(std::unique_ptr<LoaderInterfaceImpl> ptr) { return ptr; }
    static std::shared_ptr<LoaderInterface> asLoaderInterface(std::shared_ptr<LoaderInterfaceImpl> myself) {
        return std::static_pointer_cast<LoaderInterface>(myself);
    }
};