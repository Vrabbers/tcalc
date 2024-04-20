#pragma once

#include <string>
#include <memory>
#include "token.h"
#include "string_reader.h"

class tcLexer final
{
public:
    explicit tcLexer(std::string&& input, bool commaArgSeparator);
    explicit tcLexer(const char* input, bool commaArgSeparator);
    tcToken next();

private:
    tcToken flushToken(tcTokenType);
    tcToken parseNumber(char32_t first);
    tcToken parseSymbol(char32_t first);
    tcToken parseIdentifier();
    [[nodiscard]]
    char32_t decimalSeparator() const;
    [[nodiscard]]
    char32_t argSeparator() const;
    std::unique_ptr<tcStringReader> _sr;
    bool _commaArgumentSeparator;
};