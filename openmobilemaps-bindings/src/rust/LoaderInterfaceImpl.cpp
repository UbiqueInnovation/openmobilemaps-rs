// Copyright (c) 2023 Ubique Innovation AG <https://www.ubique.ch>
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#include "LoaderInterfaceImpl.h"
#include "TextureLoaderResult.h"
#include "DataLoaderResult.h"
#include "cxxgen.h"

TextureLoaderResult LoaderInterfaceImpl::loadTexture(const std::string &url, const std::optional<std::string> &etag)  {
    auto test = std::string("URL: ") + url;
    log_rs(test);
    if (etag.has_value()) {
        return this->loadTextureWrapper(url, *etag);
    } else {
        log_rs(std::string("calling wrapper function"));
        return this->loadTextureWrapper(url, std::string(""));
    }
}

DataLoaderResult LoaderInterfaceImpl::loadData(const std::string &url, const std::optional<std::string> &etag)  {

    if (etag.has_value()) {
        return this->loadDataWrapper(url, *etag);
    } else {
        return this->loadDataWrapper(url, std::string(""));
    }
}