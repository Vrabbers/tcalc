#pragma once

#include <variant>
#include <string>

#include "tc_token.h"
#include "tc_number.h"
namespace tcalc
{
    struct binary_operator final
    {
        token_kind operation;
    };

    struct unary_operator final
    {
        token_kind operation;
    };

    struct literal_number final
    {
        number num;
    };

    struct variable_reference final
    {
        std::string identifier;
    };

    struct function_call final
    {
        std::string identifier;
        int32_t arity;
    };

    using operation = std::variant<binary_operator, unary_operator, literal_number, variable_reference, function_call>;

    std::string op_to_string(const operation& op);
}