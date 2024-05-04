#pragma once
#include <string>
#include <variant>
#include <vector>

#include "tc_operation.h"
#include "tc_token.h"

namespace tcalc
{
    struct arithmetic_expression final
    {
        std::vector<operation> tokens;
    };

    struct assignment_expression final
    {
        std::string variable;
        arithmetic_expression expression;
    };

    struct func_def_expression final
    {
        std::string name;
        std::vector<std::string> parameters;
        arithmetic_expression expression;
    };

    struct boolean_expression final
    {
        arithmetic_expression lhs;
        arithmetic_expression rhs;
        token_kind kind;
    };

    using expression = std::variant<arithmetic_expression, assignment_expression, func_def_expression, boolean_expression>;
}
