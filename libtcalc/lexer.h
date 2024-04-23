#pragma once

#include <string>
#include <memory>
#include <vector>

#include "diagnostic.h"
#include "token.h"
#include "string_reader.h"

namespace tc
{
    class lexer final
    {
    public:
        explicit lexer(std::string&& input, bool comma_arg_separator) :
            _sr{std::move(input)}, _comma_argument_separator{comma_arg_separator}
        {
        }

        explicit lexer(const std::string& input, bool comma_arg_separator) :
            _sr{input}, _comma_argument_separator{comma_arg_separator}
        {
        }

        [[nodiscard]]
        const std::vector<diagnostic>& diagnostic_bag() const
        {
            return _diagnostic_bag;
        }

        token next();

    private:
        token flush_token(token_kind);
        token lex_number();
        token lex_superscript_number();
        token lex_symbol();
        token lex_word();

        [[nodiscard]]
        char32_t decimal_separator() const
        {
            return _comma_argument_separator ? U'.' : U',';
        }

        [[nodiscard]]
        char32_t arg_separator() const
        {
            return _comma_argument_separator ? U',' : U';';
        }

        std::vector<diagnostic> _diagnostic_bag{};
        string_reader _sr;
        bool _comma_argument_separator;
    };
}
