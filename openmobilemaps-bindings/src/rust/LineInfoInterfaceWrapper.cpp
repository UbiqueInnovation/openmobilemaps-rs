#include "LineInfoInterfaceWrapper.h"
#include "cxxgen3.h"

std::string LineInfoInterfaceWrapper::getIdentifier()
{
    return this->identifier;
}
std::vector<Coord> LineInfoInterfaceWrapper::getCoordinates()
{
    
    std::vector<Coord> coords;
    for (auto innerCoord : this->coordinates)
    {
        coords.push_back(innerCoord);
    }
    return coords;
}
LineStyle LineInfoInterfaceWrapper::getStyle()
{
    return *this->style;
}