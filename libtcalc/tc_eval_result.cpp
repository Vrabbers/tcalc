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
        case eval_error_type::undefined_function:
            return "undefined_function"sv;
        case eval_error_type::bad_arity:
            return "bad_arity"sv;
        case eval_error_type::log_base:
            return "log_base"sv;
        case eval_error_type::complex_inequality:
            return "complex_inequality"sv;
        case eval_error_type::out_of_tan_domain:
            return "out_of_tan_domanin"sv;
        case eval_error_type::zero_pow_zero:
            return "zero_pow_zero"sv;
        default:
            return {};
    }
}
