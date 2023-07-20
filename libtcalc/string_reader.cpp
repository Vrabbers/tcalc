#include "string_reader.h"

#include <utf8proc.h>

TCalc::StringReader::StringReader(std::string input)
{
    string = std::move(input);
    startIx = 0;
    endIx = 0;
    startPosition = { 1, 1 };
    endPosition = { 1, 1 };
}

std::pair<char32_t, std::size_t> TCalc::StringReader::peekNextCharAndLength() const
{
    if (endIx >= string.length())
        return std::make_pair(EOF, 1);

    auto startPtr = reinterpret_cast<const uint8_t*>(&string.c_str()[endIx]);
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

std::optional<char32_t> TCalc::StringReader::peekNextCharacter() const
{
    auto [character, size] = peekNextCharAndLength();
    if (size == 0)
        return std::nullopt;
    else 
        return character;
}

std::optional<char32_t> TCalc::StringReader::moveNextCharacter()
{
    auto [character, size] = peekNextCharAndLength();

    if (size == 0)
        return std::nullopt;
    
    endIx += size;

    if (character == U'\n')
    {
        endPosition.column = 1;
        endPosition.line++;
    }
    else 
    {
        endPosition.column++; 
    }

    return character;
}

std::size_t TCalc::StringReader::tokenLength() const
{
    return endIx - startIx;
}

TCalc::SourceToken TCalc::StringReader::flushToken()
{
    auto substring = string.substr(startIx, tokenLength());
    auto token = SourceToken(substring, startPosition, endPosition);
    discardToken();
    return token;
}

void TCalc::StringReader::discardToken()
{
    startIx = endIx;
    startPosition = endPosition;
}
