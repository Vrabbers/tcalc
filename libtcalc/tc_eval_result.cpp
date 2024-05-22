#include "tc_eval_result.h"

std::string_view tcalc::eval_error_type_name(eval_error_type t)
{
    using namespace std::string_view_literals;
    switch (t)
    {
        case eval_error_type::none:
            return "none"sv;
        case eval_error_type::invalid_program:
            return "invalid_program"sv;
        case eval_error_type::divide_by_zero:
            return "divide_by_zero"sv;
        case eval_error_type::log_zero:
            return "log_zero"sv;
        case eval_error_type::undefined_variable:
            return "undefined_variable"sv;
        default:
            return {};
    }
}
