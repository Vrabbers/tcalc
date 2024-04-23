#include "diagnostic.h"

#include <array>

const char* tc::diagnostic_type_name(diagnostic_type type)
{
    static std::array names =
    {
        "bad_number_literal",
        "bad_character",
        "bad_symbol"
    };

    return names.at(static_cast<std::size_t>(type));
}
