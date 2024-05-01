#include "operation.h"

#include <format>

std::string tcalc::op_to_string(const operation &op)
{
    if (std::holds_alternative<binary_operator>(op))
        return std::format("[2ry {}]", token_kind_name(std::get<binary_operator>(op).operation));
    if (std::holds_alternative<unary_operator>(op))
        return std::format("[1ry {}]", token_kind_name(std::get<unary_operator>(op).operation));
    if (std::holds_alternative<literal_number>(op))
        return std::format("[{}]", std::get<literal_number>(op).num.string());
    if (std::holds_alternative<variable_reference>(op))
        return std::format("[\"{}\"]", std::get<variable_reference>(op).identifier);
    if (std::holds_alternative<function_call>(op))
        return std::format("[{}/{}]", std::get<function_call>(op).identifier, std::get<function_call>(op).arity);
    return {};
}
