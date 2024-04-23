#include "token.h"

#include <array>
#include <format>

const char* tc::token_kind_name(token_kind type)
{
    static std::array type_names =
    {
        "bad",
        "end_of_file",

        "numeric_literal",
        "superscript_literal",
        "hex_literal",
        "binary_literal",
        "identifier",

        "plus",
        "superscript_plus",
        "minus",
        "superscript_minus",
        "multiply",
        "divide",
        "exponentiate",
        "left_parenthesis",
        "right_parenthesis",
        "radical",
        "percent",
        "factorial",
        "left_shift",
        "right_shift",
        "greater",
        "greater_or_equal",
        "less",
        "less_or_equal",
        "equal",
        "equality",
        "not_equal",
        "pi",
        "tau",
        "binary_nand",
        "binary_nor",
        "binary_xnor",
        "binary_and",
        "binary_or",
        "binary_xor",
        "binary_not",

        "argument_separator",
        "end_of_line",
    };

    return type_names.at(static_cast<std::size_t>(type));
}

std::string tc::token::format() const
{
    std::string str = std::format("(L:{}, C:{}): {} ", position().line, position().column, token_kind_name(kind()));
    if (kind() != token_kind::end_of_line && kind() != token_kind::end_of_file)
        str += std::format("\"{}\"", source_str());
    return str;
}
