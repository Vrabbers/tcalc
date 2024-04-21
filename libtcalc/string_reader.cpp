#include "string_reader.h"

#include <stdexcept>
#include <vector>

#include "utf8_utils.h"

std::pair<char32_t, std::ptrdiff_t> tcStringReader::peekAndLength() const
{
    if (_endIx >= _string.length())
        return std::make_pair(EndOfFile, 0);

    char32_t character;

    const auto length = utf8proc::iterateOnceFromIndex(_string, _endIx, &character);

    if (length > 0)
        return std::make_pair(character, length);

    return std::make_pair(0, 0);
}

std::optional<char32_t> tcStringReader::peek() const
{
    auto [character, size] = peekAndLength();
    if (size == 0 && character != EndOfFile)
        return std::nullopt;
    else 
        return character;
}

std::u32string tcStringReader::peekMany(std::uint32_t count) const
{
    std::u32string out{};
    auto index = _endIx;

    for (uint32_t i = 0; i < count; i++)
    {
        if (index >= _string.length())
            return out;

        char32_t character;
        const auto length = utf8proc::iterateOnceFromIndex(_string, index, &character);

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

std::optional<char32_t> tcStringReader::forward()
{
    auto [character, size] = peekAndLength();

    if (size == 0 && character != EndOfFile)
    {
        // some error happened. try to inch forward.
        _endIx++;
        _current = std::nullopt;
    }
    else
    {
        _endIx += size;

        if (character == U'\n')
        {
            _endPosition.column = 1;
            _endPosition.line++;
        }
        else
        {
            _endPosition.column++;
        }

        _current = character;
    }

    return _current;
}

tcSourceSpan tcStringReader::flush()
{
    auto substring = _string.substr(_startIx, tokenLength());
    auto token = tcSourceSpan(std::move(substring), _startPosition);
    discardToken();
    return token;
}

void tcStringReader::discardToken()
{
    _startIx = _endIx;
    _startPosition = _endPosition;
}
