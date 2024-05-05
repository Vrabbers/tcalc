#include "tc_string_reader.h"

#include <optional>

#include "tc_utf8_utils.h"

std::pair<char32_t, size_t> tcalc::string_reader::peek_with_length() const
{
    if (_end_ix >= _string.length())
        return {end_of_file, 0};

    const auto [length, character] = utf8proc::iterate_one_from_index(_string, _end_ix);

    if (length > 0)
        return {character, length};

    return {0, 0};
}

std::optional<char32_t> tcalc::string_reader::peek() const
{
    const auto [character, size] = peek_with_length();

    if (size == 0 && character != end_of_file)
        return std::nullopt;

    return character;
}

std::u32string tcalc::string_reader::peek_many(int32_t count) const
{
    std::u32string out;
    auto index = _end_ix;

    for (int32_t i = 0; i < count; i++)
    {
        if (index >= _string.length())
            return out;

        const auto [length, character] = utf8proc::iterate_one_from_index(_string, index);

        if (length > 0)
        {
            out += character;
            index += length;
        }
        else
        {
            out += character;
            index += length + 1;
        }
    }
    return out;
}

std::optional<char32_t> tcalc::string_reader::forward()
{
    const auto [character, size] = peek_with_length();

    if (size == 0 && character != end_of_file)
    {
        // some error happened. try to inch forward.
        _end_ix++;
        _current = std::nullopt;
    }
    else
    {
        _end_ix += size;
        _current = character;
    }

    return _current;
}

std::pair<tcalc::source_position, std::string_view> tcalc::string_reader::flush()
{
    auto substr = std::string_view{_string}.substr(_start_ix, token_length());
    const source_position position{_start_ix, _end_ix};
    discard_token();
    return {position, substr};
}
