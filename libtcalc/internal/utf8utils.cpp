#include "utf8utils.h"

#include <stdexcept>
#include <string>

static char to_inline(const char32_t super)
{
    switch (super)
    {
        case U'⁻':
            return '-';
        case U'⁺':
            return '+';
        case U'¹':
            return '1';
        case U'²':
            return '2';
        case U'³':
            return '3';
        case U'⁴':
            return '4';
        case U'⁵':
            return '5';
        case U'⁶':
            return '6';
        case U'⁷':
            return '7';
        case U'⁸':
            return '8';
        case U'⁹':
            return '9';
        case U'⁰':
            return '0';
        case U'ᵉ':
        case U'ᴱ':
            return 'e';
        case U'ⁱ':
            return 'i';
        default:
            throw std::invalid_argument{"super"};
    }
}

std::string utf8utils::to_inline_number(const std::string_view superscript_string)
{
    std::string out;
    size_t ix = 0;
    while (ix < superscript_string.size())
    {
        const auto [delta, codept] = iterate_one_from_index(superscript_string, ix);
        if (delta == 0)
            break;
        ix += delta;
        out.push_back(to_inline(codept));
    }
    return out;
}
