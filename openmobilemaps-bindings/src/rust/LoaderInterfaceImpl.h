// Copyright (c) 2023 Ubique Innovation AG <https://www.ubique.ch>
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#pragma once

#include "LoaderInterface.h"
#include <optional>
#include <memory>
#include "cxx.h"

struct LoaderInterfaceWrapperImpl;

class LoaderInterfaceImpl : public LoaderInterface
{
  ::rust::Box<LoaderInterfaceWrapperImpl> rustBox;
  std::unique_ptr<TextureLoaderResult> cachedResponse;

public:
  ~LoaderInterfaceImpl();
  LoaderInterfaceImpl(const LoaderInterfaceImpl &) = delete;
  LoaderInterfaceImpl &operator=(const LoaderInterfaceImpl &) = delete;
  LoaderInterfaceImpl(LoaderInterfaceWrapperImpl *ptr);
  virtual TextureLoaderResult loadTexture(const std::string &url, const std::optional<std::string> &etag) override;
  virtual DataLoaderResult loadData(const std::string &url, const std::optional<std::string> &etag) override;

  static std::shared_ptr<LoaderInterfaceImpl> toShared(std::unique_ptr<LoaderInterfaceImpl> ptr) { return ptr; }
  static std::shared_ptr<LoaderInterface> asLoaderInterface(std::shared_ptr<LoaderInterfaceImpl> myself)
  {
    return std::static_pointer_cast<LoaderInterface>(myself);
  }
};