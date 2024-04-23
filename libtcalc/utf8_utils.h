#pragma once

#include <cstdint>
#include <utility>
#include <string>

#include <utf8proc.h>

// Wrappers around utf8proc functions to make other code more readable
namespace utf8proc
{
    inline std::string encode_char(const char32_t character)
    {
        char buf[4] = { };
        const auto count = utf8proc_encode_char(static_cast<int32_t>(character), reinterpret_cast<uint8_t*>(buf));
        return {buf, static_cast<size_t>(count)};
    }

    // WARNING! Make sure index < string.size()!
    inline std::pair<ptrdiff_t, char32_t> iterate_one_from_index(const std::string& string, const size_t index)
    {
        char32_t character;
        auto start_ptr = reinterpret_cast<const uint8_t*>(&string.c_str()[index]);
        auto len = utf8proc_iterate(start_ptr, -1, reinterpret_cast<int32_t*>(&character));
        return std::make_pair(len, character);
    }

    inline utf8proc_category_t category(const char32_t character)
    {
        return utf8proc_category(static_cast<int32_t>(character));
    }
}
