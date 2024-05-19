#ifndef UTF8UTILS_H
#define UTF8UTILS_H

#include <cstdint>
#include <utility>
#include <string>

#include <utf8proc.h>

// Wrappers around utf8proc functions to make other code more readable
namespace utf8utils
{
    inline std::pair<ptrdiff_t, char32_t> iterate_one_from_index(const std::string& string, const size_t index)
    {
        char32_t character = U'\0';
        const auto start_ptr = reinterpret_cast<const uint8_t*>(&string.at(index));
        const size_t len = utf8proc_iterate(start_ptr, -1, reinterpret_cast<int32_t*>(&character));
        return {len, character};
    }

    inline std::pair<ptrdiff_t, char32_t> iterate_one_from_index(const std::string_view string, const size_t index)
    {
        char32_t character = U'\0';
        const auto start_ptr = reinterpret_cast<const uint8_t*>(&string.at(index));
        const size_t len = utf8proc_iterate(start_ptr, -1, reinterpret_cast<int32_t*>(&character));
        return {len, character};
    }

    inline utf8proc_category_t category(const char32_t character)
    {
        return utf8proc_category(static_cast<int32_t>(character));
    }

    std::string to_inline_number(std::string_view superscript_string);
}

#endif // UTF8UTILS_H
