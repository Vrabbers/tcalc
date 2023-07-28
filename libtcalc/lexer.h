#pragma once

#include <string>
#include <vector>
#include "token.h"
#include "tcalc_export.h"
#include "string_reader.h"

class TCALC_EXPORT Lexer final
{
    public:
        explicit Lexer(std::string&& input, bool argSeparatorIsComma);
        Token next();

    private:
        Token flushToken(Token::Type);
        Token parseNumber(char32_t first);
        char32_t decimalSeparator() const;
        char32_t argSeparator() const;
        StringReader _sr;
        bool _argSeparatorIsComma;
};