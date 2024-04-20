#pragma once

#include <string>
#include "tcalc_export.h"
#include "source_position.h"

class tcSourceSpan final
{
public:
    tcSourceSpan(std::string&&, tcSourcePosition start);
    [[nodiscard]]
    TCALC_EXPORT const char* string() const;
    [[nodiscard]]
    TCALC_EXPORT tcSourcePosition position() const;
    TCALC_EXPORT ~tcSourceSpan();
private:
    std::string _string;
    tcSourcePosition _position;
};
