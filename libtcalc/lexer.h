#pragma once

#include <string>
#include <memory>

#include "diagnostic.h"
#include "token.h"
#include "string_reader.h"

namespace tc
{
    class lexer final
    {
    public:
        explicit lexer(std::string&& input, bool commaArgSeparator) : _sr(std::make_unique<string_reader>(std::move(input))),
            _comma_argument_separator(commaArgSeparator)
        {
        }

        explicit lexer(const char* input, bool commaArgSeparator) : _sr(std::make_unique<string_reader>(input)),
            _comma_argument_separator(commaArgSeparator)
        {
        }

        [[nodiscard]]
        const std::multiset<diagnostic>& diagnostic_bag() const
        {
            return *_diagnostic_bag;
        }

        token next();

    private:
        token flush_token(token_kind);
        token lex_number();
        token lex_superscript_number();
        token lex_symbol();
        token lex_word();

        [[nodiscard]]
        inline char32_t decimal_separator() const;
        [[nodiscard]]
        inline char32_t arg_separator() const;

        std::unique_ptr<std::multiset<diagnostic>> _diagnostic_bag = std::make_unique<std::multiset<diagnostic>>();
        std::unique_ptr<string_reader> _sr;
        bool _comma_argument_separator;
    };
}
