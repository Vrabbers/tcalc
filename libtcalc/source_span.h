#pragma once

#include <string>
#include "source_position.h"

class tcSourceSpan final
{
public:
    tcSourceSpan(std::string&&, tcSourcePosition start);
    [[nodiscard]]
    std::string_view string() const;
    [[nodiscard]]
    tcSourcePosition position() const;
private:
    std::string _string;
    tcSourcePosition _position;
};
