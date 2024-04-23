#include "string_reader.h"

#include <stdexcept>

#include "utf8_utils.h"

std::pair<char32_t, std::ptrdiff_t> tc::string_reader::peek_with_length() const
{
    if (_end_ix >= _string.length())
        return std::make_pair(END_OF_FILE, 0);

    char32_t character;

    const auto length = utf8proc::iterate_one_from_index(_string, _end_ix, &character);

    if (length > 0)
        return std::make_pair(character, length);

    return std::make_pair(0, 0);
}

std::optional<char32_t> tc::string_reader::peek() const
{
    auto [character, size] = peek_with_length();
    if (size == 0 && character != END_OF_FILE)
        return std::nullopt;
    else 
        return character;
}

std::u32string tc::string_reader::peek_many(std::uint32_t count) const
{
    std::u32string out{};
    auto index = _end_ix;

    for (uint32_t i = 0; i < count; i++)
    {
        if (index >= _string.length())
            return out;

        char32_t character;
        const auto length = utf8proc::iterate_one_from_index(_string, index, &character);

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

std::optional<char32_t> tc::string_reader::forward()
{
    auto [character, size] = peek_with_length();

    if (size == 0 && character != END_OF_FILE)
    {
        // some error happened. try to inch forward.
        _end_ix++;
        _current = std::nullopt;
    }
    else
    {
        _end_ix += size;

        if (character == U'\n')
        {
            _end_position.column = 1;
            _end_position.line++;
        }
        else
        {
            _end_position.column++;
        }

        _current = character;
    }

    return _current;
}

std::unique_ptr<tc::source_span> tc::string_reader::flush()
{
    auto substring = _string.substr(_start_ix, token_length());
    auto token = std::make_unique<source_span>(std::move(substring), _start_position, _start_ix);
    discard_token();
    return token;
}
