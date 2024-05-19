#ifndef TC_EXPRESSION_H
#define TC_EXPRESSION_H

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
        source_position position;
    };

    struct assignment_expression final
    {
        std::string variable;
        arithmetic_expression expression;
        source_position position;
    };

    struct func_def_expression final
    {
        std::string name;
        std::vector<std::string> parameters;
        arithmetic_expression expression;
        source_position position;
    };

    struct boolean_expression final
    {
        arithmetic_expression lhs;
        arithmetic_expression rhs;
        token_kind kind;
        source_position position;
    };

    using expression = std::variant<arithmetic_expression, assignment_expression, func_def_expression, boolean_expression>;
}
#endif // TC_EXPRESSION_H
