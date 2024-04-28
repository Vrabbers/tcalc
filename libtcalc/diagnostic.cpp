#include "diagnostic.h"

#include "token.h"

std::string_view tcalc::diagnostic_type_name(const diagnostic_type type)
{
    using namespace std::literals;
    switch (type)
    {
        case diagnostic_type::bad_character:
            return "bad_character"sv;
        case diagnostic_type::invalid_number_literal:
            return "invalid_number_literal"sv;
        case diagnostic_type::invalid_symbol:
            return "invalid_symbol"sv;
        default:
            return {};
    }
}
