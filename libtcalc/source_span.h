#pragma once

#include <string>
#include "tcalc_export.h"
#include "source_position.h"

class TCALC_EXPORT SourceSpan final
{
    public:
        SourceSpan(std::string_view, SourcePosition start);
        std::string_view string() const;
        SourcePosition position() const;
    private:
        std::string_view _string;
        SourcePosition _position;
};
