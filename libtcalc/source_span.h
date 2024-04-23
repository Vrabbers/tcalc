#pragma once

#include <string>
#include "source_position.h"

class tcSourceSpan final
{
public:
    tcSourceSpan(std::string&& str, const tcSourcePosition start, const std::size_t srcIx) : _string(str), _position(start), _sourceIndex(srcIx)
    {
    }

    [[nodiscard]]
    std::string_view sourceStr() const
    {
        return _string;
    }

    [[nodiscard]]
    tcSourcePosition position() const
    {
        return _position;
    }

    [[nodiscard]]
    std::size_t sourceIndex() const
    {
        return _sourceIndex;
    }


private:
    std::string _string;
    tcSourcePosition _position;
    std::size_t _sourceIndex;
};
