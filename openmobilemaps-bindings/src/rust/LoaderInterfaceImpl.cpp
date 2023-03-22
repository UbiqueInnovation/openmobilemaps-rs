// Copyright (c) 2023 Ubique Innovation AG <https://www.ubique.ch>
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#include "LoaderInterfaceImpl.h"
#include "TextureLoaderResult.h"
#include "DataLoaderResult.h"
#include "cxxgen.h"
#include "cxxgen4.h"

LoaderInterfaceImpl::~LoaderInterfaceImpl()
{
    log_rs("Destructor called");
}

LoaderInterfaceImpl::LoaderInterfaceImpl(LoaderInterfaceWrapperImpl *ptr) : rustBox(::rust::Box<LoaderInterfaceWrapperImpl>::from_raw(ptr)), cachedResponse()
{
}

TextureLoaderResult LoaderInterfaceImpl::loadTexture(const std::string &url, const std::optional<std::string> &etag)
{
    auto test = std::string("URL: ") + url;
    // log_rs(test);
    if (etag.has_value())
    {
        auto result = this->rustBox->loadTextureWrapper(url, std::make_unique<std::string>(*etag));
        auto holder = TextureLoaderResult(result->data, result->etag, result->status, result->errorCode);
        return holder;
    }
    else
    {
        auto result = this->rustBox->loadTextureWrapper(url, std::make_unique<std::string>(""));
        auto holder = TextureLoaderResult(result->data, result->etag, result->status, result->errorCode);
        return holder;
    }
}

DataLoaderResult LoaderInterfaceImpl::loadData(const std::string &url, const std::optional<std::string> &etag)
{

    if (etag.has_value())
    {
        return *this->rustBox->loadDataWrapper(url, std::make_unique<std::string>(*etag)).release();
    }
    else
    {
        return *this->rustBox->loadDataWrapper(url, std::make_unique<std::string>(std::string(""))).release();
    }
}