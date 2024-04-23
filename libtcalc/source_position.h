#pragma once

#include <cstdint>

namespace tc
{
    struct source_position
    {
        std::uint32_t line;
        std::uint32_t column;
    };
}
