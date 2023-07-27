#pragma once

#include <string>
#include "source_position.h"

class SourceSpan final
{
    public:
        SourceSpan(std::string, SourcePosition start);
        const std::string& string() const;
        SourcePosition position() const;
    private:
        std::string _string;
        SourcePosition _position;
};
