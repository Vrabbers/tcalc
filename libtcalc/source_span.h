#pragma once

#include <string>
#include "tcalc_export.h"
#include "source_position.h"

class tcSourceSpan final
{
    public:
        tcSourceSpan(std::string&&, SourcePosition start);
        TCALC_EXPORT const char* string() const;
        TCALC_EXPORT SourcePosition position() const;
        TCALC_EXPORT ~tcSourceSpan() = default;
    private:
        std::string _string;
        SourcePosition _position;
};
