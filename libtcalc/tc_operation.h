#ifndef TC_OPERATION_H
#define TC_OPERATION_H

#include <variant>
#include <string>

#include "tc_token.h"
#include "tc_number.h"

namespace tcalc
{
    struct binary_operator final
    {
        token_kind operation;
        source_position position;
    };

    struct unary_operator final
    {
        token_kind operation;
        source_position position;
    };

    struct literal_number final
    {
        number num;
        source_position position;
    };

    struct variable_reference final
    {
        std::string identifier;
        source_position position;
    };

    using fn_arity_t = int32_t;

    struct function_call final
    {
        std::string identifier;
        fn_arity_t arity;
        source_position position;
    };

    using operation = std::variant<binary_operator, unary_operator, literal_number, variable_reference, function_call>;

    std::string op_to_string(const operation& op);
}

#endif // TC_OPERATION_H
