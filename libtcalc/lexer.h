#pragma once

#include <string>
#include <vector>
#include "token.h"
#include "string_reader.h"

class Lexer final
{
    public:
        explicit Lexer(std::string&& input, bool argSeparatorIsComma);
        Token next();

    private:
        StringReader _sr;
        bool _argSeparatorIsComma;
};