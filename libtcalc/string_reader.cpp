#include "string_reader.h"

#include "utf8proc.h"

std::pair<char32_t, std::int32_t> tcStringReader::peekAndLength() const
{
    if (_endIx >= _string.length())
        return std::make_pair(EndOfFile, 0);

    const auto startPtr = reinterpret_cast<const uint8_t*>(&_string.c_str()[_endIx]);
    char32_t character;

    const auto length =
        static_cast<std::int32_t>(utf8proc_iterate(startPtr, -1, reinterpret_cast<int32_t*>(&character)));

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

std::optional<char32_t> tcStringReader::forward()
{
    auto [character, size] = peekAndLength();

    if (size == 0 && character != EndOfFile)
    {
        // some error happened. try to inch forward.
        _endIx++;
        return std::nullopt;
    }

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

    return character;
}

std::size_t tcStringReader::tokenLength() const
{
    return _endIx - _startIx;
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
