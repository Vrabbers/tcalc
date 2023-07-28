#pragma once

#include <string>
#include "tcalc_export.h"
#include "source_position.h"

class TCALC_EXPORT SourceSpan final
{
    public:
        SourceSpan(std::string&&, SourcePosition start);
        const std::string& string() const;
        SourcePosition position() const;
    private:
        std::string _string;
        SourcePosition _position;
};
