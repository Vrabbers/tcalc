#include "token.h"

#include <array>
#include <format>

using namespace tc;

std::string_view tc::token_kind_name(token_kind type)
{
    using namespace std::literals;
    constexpr static std::array type_names =
    {
        "bad"sv,
        "end_of_file"sv,

        "numeric_literal"sv,
        "superscript_literal"sv,
        "hex_literal"sv,
        "binary_literal"sv,
        "identifier"sv,

        "plus"sv,
        "superscript_plus"sv,
        "minus"sv,
        "superscript_minus"sv,
        "multiply"sv,
        "divide"sv,
        "exponentiate"sv,
        "left_parenthesis"sv,
        "right_parenthesis"sv,
        "radical"sv,
        "percent"sv,
        "factorial"sv,
        "left_shift"sv,
        "right_shift"sv,
        "greater"sv,
        "greater_or_equal"sv,
        "less"sv,
        "less_or_equal"sv,
        "equal"sv,
        "equality"sv,
        "not_equal"sv,
        "pi"sv,
        "tau"sv,
        "binary_nand"sv,
        "binary_nor"sv,
        "binary_xnor"sv,
        "binary_and"sv,
        "binary_or"sv,
        "binary_xor"sv,
        "binary_not"sv,

        "argument_separator"sv,
        "end_of_line"sv,
    };

    return type_names.at(static_cast<size_t>(type));
}

std::string token::format() const
{
    std::string str = std::format("(L:{}, C:{}): {} ", position().line, position().column, token_kind_name(kind()));
    if (kind() != token_kind::end_of_line && kind() != token_kind::end_of_file)
        str += std::format("\"{}\"", source_str());
    return str;
}
