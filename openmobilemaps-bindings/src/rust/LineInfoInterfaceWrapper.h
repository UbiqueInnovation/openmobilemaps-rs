// Copyright (c) 2023 Ubique Innovation AG <https://www.ubique.ch>
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
#pragma once

#include "LineInfoInterface.h"
#include "LineStyle.h"
#include "ColorStateList.h"
#include "SizeType.h"
#include "LineCapType.h"
#include "Coord.h"
#include <string>
#include <memory>

class LineInfoInterfaceWrapper : public LineInfoInterface
{
    std::string identifier;
    std::vector<Coord> coordinates;
    std::unique_ptr<LineStyle> style;

public:
    LineInfoInterfaceWrapper(std::string identifier, std::vector<Coord> coords, std::unique_ptr<LineStyle> style) : identifier(identifier), coordinates(coords), style(std::move(style)) {}

    virtual std::string getIdentifier() override;

    virtual std::vector<::Coord> getCoordinates() override;

    virtual LineStyle getStyle() override;
};

class LineInfoInterfaceWrapperBuilder
{
    std::string identifier;
    std::vector<Coord> coordinates;
    std::unique_ptr<LineStyle> style;
    public:
     LineInfoInterfaceWrapperBuilder(): identifier(), coordinates(), style() {}
     void addCoordinate(Coord &coordinate) {
         this->coordinates.push_back(coordinate);
     }
     void setStyle(std::unique_ptr<LineStyle> style) {
         this->style = std::move(style);
     }
     void setIdentifier(std::string identifier) {
         this->identifier = identifier;
     }
     std::shared_ptr<LineInfoInterface> build() {
         return std::static_pointer_cast<LineInfoInterface> (std::make_shared<LineInfoInterfaceWrapper>(this->identifier, this->coordinates, std::move(this->style)));
     }
};

class LineStyleBuilder {

};

inline std::vector<float> make_default_dash()
{
     return std::vector<float>(1.0);
}