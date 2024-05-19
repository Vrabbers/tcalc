#ifndef TC_LEXER_H
#define TC_LEXER_H

#include <string>
#include <memory>
#include <vector>

#include "tc_diagnostic.h"
#include "tc_token.h"
#include "tc_string_reader.h"

namespace tcalc
{
    class lexer final
    {
    public:
        explicit lexer(std::string&& input, bool comma_arg_separator) :
            _sr{std::move(input)},
            _comma_argument_separator{comma_arg_separator}
        {
        }

        explicit lexer(const std::string& input, bool comma_arg_separator) :
            _sr{input},
            _comma_argument_separator{comma_arg_separator}
        {
        }

        [[nodiscard]]
        std::vector<diagnostic>& diagnostic_bag()
        {
            return _diagnostic_bag;
        }

        token next();

        [[nodiscard]]
        bool reached_end() const
        {
            return _reached_end;
        }

    private:
        token flush_token(token_kind);
        token lex_hex_number();
        token lex_decimal_number();
        token lex_binary_number();
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
        bool _reached_end{false};
    };
}

#endif // TC_LEXER_H
