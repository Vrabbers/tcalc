#ifndef TC_TOKEN_H
#define TC_TOKEN_H

#include <memory>
#include <string>

#include "tc_source_position.h"

namespace tcalc
{
    enum class token_kind
    {
        bad,
        end_of_file,

        numeric_literal,
        superscript_literal,
        hex_literal,
        binary_literal,
        identifier,

        plus,
        superscript_plus,
        minus,
        superscript_minus,
        multiply,
        divide,
        exponentiate,
        open_parenthesis,
        close_parenthesis,
        radical,
        cube_root,
        fourth_root,
        percent,
        factorial,
        left_shift,
        right_shift,
        greater_than,
        greater_or_equal,
        less_than,
        less_or_equal,
        equal,
        equality,
        not_equal,
        binary_nand,
        binary_nor,
        binary_xnor,
        binary_and,
        binary_or,
        binary_xor,
        binary_not,
        deg,
        rad,
        grad,

        argument_separator,
        expression_separator,
    };

    std::string_view token_kind_name(token_kind kind);

    class token final
    {
    public:
        token(const token_kind kind, std::string&& str, const source_position pos) :
            _source{std::move(str)},
            _position{pos},
            _kind{kind}
        {
        }

        [[nodiscard]]
        std::string format() const;

        [[nodiscard]]
        token_kind kind() const
        {
            return _kind;
        }

        [[nodiscard]]
        const source_position& position() const
        {
            return _position;
        }

        [[nodiscard]]
        std::string_view source() const
        {
            return _source;
        }

        [[nodiscard]]
        size_t start_index() const
        {
            return _position.start_index;
        }

        [[nodiscard]]
        size_t end_index() const
        {
            return _position.end_index;
        }
    private:
        std::string _source;
        source_position _position;
        token_kind _kind;
    };
}

#endif // TC_TOKEN_H
