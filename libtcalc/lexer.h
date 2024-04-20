#pragma once

#include <string>
#include <vector>
#include <memory>
#include "token.h"
#include "tcalc_export.h"
#include "string_reader.h"

class tcLexer final
{
    public:
        explicit tcLexer(std::string&& input, bool commaArgSeparator);
        TCALC_EXPORT explicit tcLexer(const char* input, bool commaArgSeparator);
        TCALC_EXPORT tcToken next();
        TCALC_EXPORT ~tcLexer();

    private:
        tcToken flushToken(tcTokenType);
        tcToken parseNumber(char32_t first);
        tcToken parseSymbol(char32_t first);
        [[nodiscard]]
        char32_t decimalSeparator() const;
        [[nodiscard]]
        char32_t argSeparator() const;
        std::unique_ptr<tcStringReader> _sr;
        bool _commaArgumentSeparator;
};