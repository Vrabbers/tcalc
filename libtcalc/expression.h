#pragma once
#include <string>
#include <variant>
#include <vector>

#include "token.h"

namespace tcalc
{
    struct arithmetic_expression final
    {
        std::vector<token> tokens;
    };

    struct assignment_expression final
    {
        std::string variable;
        arithmetic_expression expression;
    };

    struct function_expression final
    {
        std::string variable;
        std::vector<std::string> parameters;
        arithmetic_expression expression;
    };

    struct boolean_expression final
    {
        enum class kind
        {
            equal,
            not_equal,
            less_than,
            less_equal,
            greater_than,
            greater_equal,
        };

        arithmetic_expression lhs;
        arithmetic_expression rhs;
        kind kind = kind::equal;
    };

    using expression = std::variant<arithmetic_expression, assignment_expression, function_expression, boolean_expression>;
}
