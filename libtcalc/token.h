#pragma once

#include <memory>

#include "source_span.h"

namespace tc
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
        left_parenthesis,
        right_parenthesis,
        radical,
        percent,
        factorial,
        left_shift,
        right_shift,
        greater,
        greater_or_equal,
        less,
        less_or_equal,
        equal,
        equality,
        not_equal,
        pi,
        tau,
        binary_nand,
        binary_nor,
        binary_xnor,
        binary_and,
        binary_or,
        binary_xor,
        binary_not,

        argument_separator,
        end_of_line,
    };

    std::string_view token_kind_name(token_kind kind);

    class token final
    {
    public:
        token(const token_kind kind, source_span&& source) : _kind{kind}, _source_span{std::move(source)}
        {
        }

        [[nodiscard]]
        token_kind kind() const
        {
            return _kind;
        }

        [[nodiscard]]
        const source_span& source() const
        {
            return _source_span;
        }

        [[nodiscard]]
        std::string_view source_str() const
        {
            return _source_span.source_str();
        }

        [[nodiscard]]
        source_position position() const
        {
            return _source_span.position();
        }

        [[nodiscard]]
        std::string format() const;
    private:
        token_kind _kind;
        source_span _source_span;
    };
}
