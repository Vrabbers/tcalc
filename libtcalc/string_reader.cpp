#include "string_reader.h"

#include <utf8proc.h>

StringReader::StringReader(std::string&& input) : _string(input)
{
    _startIx = 0;
    _endIx = 0;
    _startPosition = { 1, 1 };
    _endPosition = { 1, 1 };
}

std::pair<char32_t, std::size_t> StringReader::peekNextCharAndLength() const
{
    if (_endIx >= _string.length())
        return std::make_pair(EndOfFile, 0);

    auto startPtr = reinterpret_cast<const uint8_t*>(&_string.c_str()[_endIx]);
    char32_t character;

    auto length = utf8proc_iterate(startPtr, -1, reinterpret_cast<int32_t*>(&character));

    if (length > 0)
    {
        return std::make_pair(character, length);
    }
    else
    {
        return std::make_pair(0, 0);
    }
}

std::optional<char32_t> StringReader::peekNextCharacter() const
{
    auto [character, size] = peekNextCharAndLength();
    if (size == 0 && character != EndOfFile)
        return std::nullopt;
    else 
        return character;
}

std::optional<char32_t> StringReader::moveNextCharacter()
{
    auto [character, size] = peekNextCharAndLength();

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

std::size_t StringReader::tokenLength() const
{
    return _endIx - _startIx;
}

SourceSpan StringReader::flush()
{
    auto substring = _string.substr(_startIx, tokenLength());
    auto token = SourceSpan(substring, _startPosition);
    discardToken();
    return token;
}

void StringReader::discardToken()
{
    _startIx = _endIx;
    _startPosition = _endPosition;
}
