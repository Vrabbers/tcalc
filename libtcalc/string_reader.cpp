#include "string_reader.h"

#include <utf8proc.h>

TCalc::StringReader::StringReader(std::unique_ptr<std::string> input)
{
    string = std::move(input);
    startIx = 0;
    endIx = 0;
    startPosition = { 0, 0 };
    endPosition = { 0, 0 };
}

std::pair<char32_t, std::size_t> TCalc::StringReader::peekNextCharAndLength() const
{
    if (endIx >= string->length())
        return std::make_pair(EOF, 1);

    auto startPtr = reinterpret_cast<const uint8_t*>(string->c_str());
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
    return std::optional<char32_t>();
}

std::size_t TCalc::StringReader::tokenLength() const
{
    return std::size_t();
}

std::string TCalc::StringReader::flushToken()
{
    return std::string();
}

void TCalc::StringReader::discardToken()
{
}
