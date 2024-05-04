#pragma once

#include <memory>

#include "tc_source_span.h"

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
        percent,
        factorial,
        left_shift,
        right_shift,
        greater_than,
        greater_or_equal,
        less_than,
        less_or_equal,
        equal,
        not_equal,
        binary_nand,
        binary_nor,
        binary_xnor,
        binary_and,
        binary_or,
        binary_xor,
        binary_not,

        argument_separator,
        expression_separator,
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
        size_t start_index() const
        {
            return _source_span.start_index();
        }

        [[nodiscard]]
        size_t end_index() const
        {
            return _source_span.end_index();
        }

        [[nodiscard]]
        std::string format() const;
    private:
        token_kind _kind;
        source_span _source_span;
    };
}
