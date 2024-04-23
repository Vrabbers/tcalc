#include "diagnostic.h"

#include <array>

const char* tc::diagnostic_type_name(diagnostic_type type)
{
    static std::array names =
    {
        "bad_character",
        "invalid_number_literal",
        "invalid_symbol"
    };

    return names.at(static_cast<size_t>(type));
}
