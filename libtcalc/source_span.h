#pragma once

#include <string>
#include "source_position.h"

class tcSourceSpan final
{
public:
    tcSourceSpan(std::string&&, tcSourcePosition start);
    [[nodiscard]]
    const char* string() const;
    [[nodiscard]]
    tcSourcePosition position() const;
    ~tcSourceSpan();
private:
    std::string _string;
    tcSourcePosition _position;
};
