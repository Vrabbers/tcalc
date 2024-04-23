#pragma once
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

    const char* token_kind_name(token_kind);
}