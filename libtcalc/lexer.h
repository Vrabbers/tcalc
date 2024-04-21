#pragma once

#include <string>
#include <memory>
#include "token.h"
#include "string_reader.h"

class tcLexer final
{
public:
    explicit tcLexer(std::string&& input, bool commaArgSeparator) : _sr(std::make_unique<tcStringReader>(std::move(input))),
                                                                    _commaArgumentSeparator(commaArgSeparator)
    {
    }

    explicit tcLexer(const char* input, bool commaArgSeparator) : _sr(std::make_unique<tcStringReader>(input)),
                                                                  _commaArgumentSeparator(commaArgSeparator)
    {
    }

    tcToken next();

private:
    tcToken flushToken(tcTokenKind);
    tcToken parseNumber(char32_t first);
    tcToken parseSuperscriptNumber();
    tcToken parseSymbol(char32_t first);
    tcToken parseIdentifier();

    [[nodiscard]]
    inline char32_t decimalSeparator() const;
    [[nodiscard]]
    inline char32_t argSeparator() const;

    std::unique_ptr<tcStringReader> _sr;
    bool _commaArgumentSeparator;
};
