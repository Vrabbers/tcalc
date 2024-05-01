#pragma once

#include <variant>
#include <string>

#include "token.h"
#include "number.h"
namespace tcalc
{
    struct binary_operator
    {
        token_kind operation;
    };

    struct unary_operator
    {
        token_kind operation;
    };

    struct literal_number
    {
        number num;
    };

    struct variable_reference
    {
        std::string identifier;
    };

    struct function_call
    {
        std::string identifier;
        int32_t arity;
    };

    using operation = std::variant<binary_operator, unary_operator, literal_number, variable_reference, function_call>;

    std::string op_to_string(const operation& op);
}