#ifndef BUILTINS_H
#define BUILTINS_H

#include "../tc_evaluator.h"

namespace tcalc
{
    void convert_angle(number& number, angle_unit from, angle_unit to);

    using number_member_fn = void (number::*)(const number&);

    template<number_member_fn Fn>
    eval_error_type builtin1(evaluator::stack& stack, const evaluator&)
    {
        (stack.back().*Fn)(stack.back());
        return eval_error_type::none;
    }

    template <number_member_fn Fn>
    eval_error_type builtin1_angle_argument(evaluator::stack& stack, const evaluator& eval)
    {
        if (stack.back().is_real())
            convert_angle(stack.back(), eval.trig_unit(), angle_unit::radians);
        (stack.back().*Fn)(stack.back());
        return eval_error_type::none;
    }

    template <number_member_fn Fn>
    eval_error_type builtin1_angle_result(evaluator::stack& stack, const evaluator& eval)
    {
        (stack.back().*Fn)(stack.back());
        if (stack.back().is_real())
            convert_angle(stack.back(), angle_unit::radians, eval.trig_unit());
        return eval_error_type::none;
    }

    eval_error_type builtin_sqrt(evaluator::stack&, const evaluator&);
    eval_error_type builtin_cbrt(evaluator::stack&, const evaluator&);
    eval_error_type builtin_root(evaluator::stack&, const evaluator&);
    eval_error_type builtin_log1(evaluator::stack&, const evaluator&);
    eval_error_type builtin_ln(evaluator::stack&, const evaluator&);
    eval_error_type builtin_log2(evaluator::stack& , const evaluator&);
    eval_error_type builtin_tan(evaluator::stack&, const evaluator&);
    eval_error_type builtin_sec(evaluator::stack&, const evaluator&);
    eval_error_type builtin_csc(evaluator::stack&, const evaluator&);
    eval_error_type builtin_cot(evaluator::stack&, const evaluator&);
    eval_error_type builtin_asec(evaluator::stack&, const evaluator&);
    eval_error_type builtin_acsc(evaluator::stack&, const evaluator&);
    eval_error_type builtin_acot(evaluator::stack&, const evaluator&);
    eval_error_type builtin_sech(evaluator::stack&, const evaluator&);
    eval_error_type builtin_csch(evaluator::stack&, const evaluator&);
    eval_error_type builtin_coth(evaluator::stack&, const evaluator&);
    eval_error_type builtin_asech(evaluator::stack&, const evaluator&);
    eval_error_type builtin_acsch(evaluator::stack&, const evaluator&);
    eval_error_type builtin_acoth(evaluator::stack&, const evaluator&);
}

#endif //BUILTINS_H
