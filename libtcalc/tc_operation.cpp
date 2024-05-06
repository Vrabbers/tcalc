#include "tc_operation.h"

#include <format>

std::string tcalc::op_to_string(const operation &op)
{
    if (const auto* bin = std::get_if<binary_operator>(&op))
        return std::format("[{}]@{}-{}", token_kind_name(bin->operation), bin->position.start_index,
                           bin->position.end_index);

    if (const auto* un = std::get_if<unary_operator>(&op))
        return std::format("[unary {}]@{}-{}", token_kind_name(un->operation), un->position.start_index,
                           un->position.end_index);

    if (const auto* num = std::get_if<literal_number>(&op))
        return std::format("({})@{}-{}", num->num.string(), num->position.start_index, num->position.end_index);

    if (const auto* var = std::get_if<variable_reference>(&op))
        return std::format("({})@{}-{}", var->identifier, var->position.start_index, var->position.end_index);

    if (const auto* fn = std::get_if<function_call>(&op))
        return std::format("[{}/{}]@{}-{}", fn->identifier, fn->arity, fn->position.start_index, fn->position.end_index);

    return {};
}
