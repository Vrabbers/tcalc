#pragma once

#include <string>
#include <vector>
#include "token.h"
#include "tcalc_export.h"
#include "string_reader.h"

class tcLexer final
{
    public:
        explicit tcLexer(std::string&& input, bool argSeparatorIsComma);
        TCALC_EXPORT explicit tcLexer(const char* input, bool argSeparatorIsComma);
        TCALC_EXPORT tcToken next();
        TCALC_EXPORT ~tcLexer() = default;

    private:
        tcToken flushToken(tcTokenType);
        tcToken parseNumber(char32_t first);
        char32_t decimalSeparator() const;
        char32_t argSeparator() const;
        tcStringReader _sr;
        bool _argSeparatorIsComma;
};