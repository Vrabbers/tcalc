#include "diagnostic.h"

#include <array>

std::string_view tc::diagnostic_type_name(diagnostic_type type)
{
    using namespace std::literals;
    constexpr static std::array names =
    {
        "bad_character"sv,
        "invalid_number_literal"sv,
        "invalid_symbol"sv,
    };

    return names.at(static_cast<size_t>(type));
}
