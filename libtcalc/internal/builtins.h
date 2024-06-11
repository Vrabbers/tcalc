#ifndef BUILTINS_H
#define BUILTINS_H

#include "../tc_evaluator.h"

namespace tcalc
{
    template <void (number::* Fn)(const number&)>
    eval_error_type builtin1(evaluator::stack& stack, const evaluator&)
    {
        (stack.back().*Fn)(stack.back());
        return eval_error_type::none;
    }

    void convert_angle(number& number, angle_unit from, angle_unit to);
    eval_error_type builtin_sqrt(evaluator::stack&, const evaluator&);
    eval_error_type builtin_cbrt(evaluator::stack&, const evaluator&);
    eval_error_type builtin_root(evaluator::stack&, const evaluator&);
    eval_error_type builtin_log1(evaluator::stack&, const evaluator&);
    eval_error_type builtin_ln(evaluator::stack&, const evaluator&);
    eval_error_type builtin_log2(evaluator::stack& , const evaluator&);
    eval_error_type builtin_sin(evaluator::stack&, const evaluator&);
    eval_error_type builtin_cos(evaluator::stack&, const evaluator&);
    eval_error_type builtin_tan(evaluator::stack&, const evaluator&);
}

#endif //BUILTINS_H
