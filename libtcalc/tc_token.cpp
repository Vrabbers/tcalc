#include "tc_token.h"

#include <array>
#include <format>

using namespace tcalc;

std::string_view tcalc::token_kind_name(const token_kind kind)
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
        "open_parenthesis"sv,
        "close_parenthesis"sv,
        "radical"sv,
        "percent"sv,
        "factorial"sv,
        "left_shift"sv,
        "right_shift"sv,
        "greater_than"sv,
        "greater_or_equal"sv,
        "less_than"sv,
        "less_or_equal"sv,
        "equal"sv,
        "equality"sv,
        "not_equal"sv,
        "binary_nand"sv,
        "binary_nor"sv,
        "binary_xnor"sv,
        "binary_and"sv,
        "binary_or"sv,
        "binary_xor"sv,
        "binary_not"sv,
        "deg"sv,
        "rad"sv,
        "grad"sv,

        "argument_separator"sv,
        "expression_separator"sv,
    };

    return type_names.at(static_cast<size_t>(kind));
}

std::string token::format() const
{
    std::string str = std::format("({}-{}): {} ", start_index(), end_index(), token_kind_name(kind()));
    if (kind() != token_kind::expression_separator && kind() != token_kind::end_of_file)
        str.append(std::format("\"{}\"", source()));
    return str;
}
