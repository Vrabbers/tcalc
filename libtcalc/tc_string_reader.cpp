#include "tc_string_reader.h"

#include <optional>

#include "utf8utils.h"

using namespace tcalc;

std::pair<int, char32_t> string_reader::peek_with_length() const
{
    if (_end_ix >= _string.length())
        return {0, end_of_file};

    const auto [length, character] = utf8utils::iterate_one_from_index(_string, _end_ix);

    if (length > 0)
        return {length, character};

    return {0, 0};
}

std::optional<char32_t> string_reader::peek() const
{
    const auto [size, character] = peek_with_length();

    if (size == 0 && character != end_of_file)
        return std::nullopt;

    return character;
}

std::u32string string_reader::peek_many(const int32_t count) const
{
    std::u32string out;
    auto index = _end_ix;

    for (int32_t i = 0; i < count; i++)
    {
        if (index >= _string.length())
            return out;

        const auto [length, character] = utf8utils::iterate_one_from_index(_string, index);

        out += character;

        if (length > 0)
            index += length;
        else
            index += length + 1;
    }
    return out;
}

std::optional<char32_t> string_reader::forward()
{
    const auto [size, character] = peek_with_length();

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

std::pair<source_position, std::string_view> string_reader::flush()
{
    auto substr = std::string_view{_string}.substr(_start_ix, token_length());
    const source_position position{_start_ix, _end_ix};
    discard_token();
    return {position, substr};
}
