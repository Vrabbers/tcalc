#include "builtins.h"

using namespace tcalc;

void tcalc::convert_angle(number& number, const angle_unit from, const angle_unit to)
{
    if (from == to)
        return;
    switch (from)
    {
        case angle_unit::degrees:
            number.div(number, 180);
        if (to == angle_unit::radians)
            number.mul(number, number::pi(number.precision()));
        else // to == gradians
            number.mul(number, 200);
        break;
        case angle_unit::radians:
            number.div(number, number::pi(number.precision()));
        if (to == angle_unit::degrees)
            number.mul(number, 180);
        else // to == gradians
            number.mul(number, 200);
        break;
        case angle_unit::gradians:
            number.div(number, 200);
        if (to == angle_unit::radians)
            number.mul(number, number::pi(number.precision()));
        else // to == degrees
            number.mul(number, 180);
        break;
    }
}


eval_error_type tcalc::builtin_sqrt(evaluator::stack& stack, const evaluator&)
{
    stack.back().sqrt(stack.back());
    return eval_error_type::none;
}

eval_error_type tcalc::builtin_cbrt(evaluator::stack& stack, const evaluator&)
{
    stack.back().nth_root(stack.back(), 3);
    return eval_error_type::none;
}

eval_error_type tcalc::builtin_root(evaluator::stack& stack, const evaluator&)
{
    if (stack.back() == 0)
        return eval_error_type::zero_root;
    const auto n = std::move(stack.back());
    stack.pop_back();
    stack.back().nth_root(stack.back(), n);
    return eval_error_type::none;
}

eval_error_type tcalc::builtin_log1(evaluator::stack& stack, const evaluator&)
{
    if (stack.back() == 0)
        return eval_error_type::log_zero;
    stack.back().log(stack.back());
    return eval_error_type::none;
}

eval_error_type tcalc::builtin_ln(evaluator::stack& stack, const evaluator&)
{
    if (stack.back() == 0)
        return eval_error_type::log_zero;
    stack.back().ln(stack.back());
    return eval_error_type::none;
}

eval_error_type tcalc::builtin_log2(evaluator::stack& stack, const evaluator&)
{
    if (stack.back() == 0 || stack.back() == 1)
        return eval_error_type::log_base;
    auto base = std::move(stack.back());
    stack.pop_back();
    if (stack.back() == 0)
        return eval_error_type::log_zero;
    base.ln(base);
    stack.back().ln(stack.back());
    stack.back().div(stack.back(), base);
    return eval_error_type::none;
}

eval_error_type tcalc::builtin_tan(evaluator::stack& stack, const evaluator& eval)
{
    if (stack.back().is_real())
        convert_angle(stack.back(), eval.trig_unit(), angle_unit::radians);
    number test{eval.precision()};
    test.cos(stack.back());
    if (test == 0)
        return eval_error_type::out_of_tan_domain;
    stack.back().tan(stack.back());
    return eval_error_type::none;
}

