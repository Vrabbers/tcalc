#include "tc_evaluator.h"

#ifdef _MSC_VER
#pragma warning(push, 0) // mpc header has warnings on MSVC /W4
#endif

#include <mpc.h>

#ifdef _MSC_VER
#pragma warning(pop)
#endif

#include "tc_eval_result.h"

#include <stdexcept>

using namespace tcalc;

namespace
{
    using namespace std::string_literals;
    using namespace std::string_view_literals;

    void convert_angle(number& number, const angle_unit from, const angle_unit to)
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

    std::unordered_map<std::string, number> initialize_constants(const mpfr_prec_t prec)
    {
        return {
            {"pi"s, number::pi(prec)},
            {"π"s, number::pi(prec)},
            {"tau"s, number::tau(prec)},
            {"τ"s, number::tau(prec)},
            {"e"s, number::e(prec)}
        };
    }

    eval_error_type builtin_sqrt(eval_stack& stack, const evaluator&)
    {
        stack.top().sqrt(stack.top());
        return eval_error_type::none;
    }

    eval_error_type builtin_exp(eval_stack& stack, const evaluator&)
    {
        stack.top().exp(stack.top());
        return eval_error_type::none;
    }

    eval_error_type builtin_log1(eval_stack& stack, const evaluator&)
    {
        if (stack.top() == 0)
            return eval_error_type::log_zero;
        stack.top().log(stack.top());
        return eval_error_type::none;
    }

    eval_error_type builtin_ln(eval_stack& stack, const evaluator&)
    {
        if (stack.top() == 0)
            return eval_error_type::log_zero;
        stack.top().ln(stack.top());
        return eval_error_type::none;
    }

    eval_error_type builtin_log2(eval_stack& stack, const evaluator&)
    {
        if (stack.top() == 0 || stack.top() == 1)
            return eval_error_type::log_base;
        auto& base = stack.pop();
        if (stack.top() == 0)
            return eval_error_type::log_zero;
        base.ln(base);
        stack.top().ln(stack.top());
        stack.top().div(stack.top(), base);
        return eval_error_type::none;
    }

    eval_error_type builtin_sin(eval_stack& stack, const evaluator& eval)
    {
        convert_angle(stack.top(), eval.trig_unit(), angle_unit::radians);
        stack.top().sin(stack.top());
        return eval_error_type::none;
    }

    eval_error_type builtin_cos(eval_stack& stack, const evaluator& eval)
    {
        convert_angle(stack.top(), eval.trig_unit(), angle_unit::radians);
        stack.top().cos(stack.top());
        return eval_error_type::none;
    }

    eval_error_type builtin_tan(eval_stack& stack, const evaluator& eval)
    {
        convert_angle(stack.top(), eval.trig_unit(), angle_unit::radians);
        number test{ eval.precision() };
        test.cos(stack.top());
        if (test == 0)
            return eval_error_type::out_of_tan_domain;
        stack.top().tan(stack.top());
        return eval_error_type::none;
    }

    std::unordered_map<std::string, std::vector<evaluator::native_fn>> basic_builtins()
    {
        return
        {
            { "sqrt"s, { { 1, &builtin_sqrt } } },
            { "exp"s, { { 1, &builtin_exp } } },
            {
                "log"s,
                {
                    { 1, &builtin_log1 },
                    { 2, &builtin_log2 }
                }
            },
            { "ln"s, { { 1, &builtin_ln } } },
            { "sin"s, { { 1, &builtin_sin } } },
            { "cos"s, { { 1, &builtin_cos } } },
            { "tan"s , { { 1, &builtin_tan } } }
        };
    }

    eval_result<evaluator::result_type> to_variant_result(eval_result<number>&& e)
    {
        if (e.is_error())
            return eval_result<evaluator::result_type>{e.error()};
        return eval_result<evaluator::result_type>{std::move(e.mut_value())};
    }

    eval_result<evaluator::result_type> to_variant_result(const eval_result<bool>& e)
    {
        if (e.is_error())
            return eval_result<evaluator::result_type>{e.error()};
        return eval_result<evaluator::result_type>{e.value()};
    }

    eval_result<evaluator::result_type> to_variant_result(eval_result<assign_result>&& e)
    {
        if (e.is_error())
            return eval_result<evaluator::result_type>{e.error()};
        return eval_result<evaluator::result_type>{std::move(e.mut_value())};
    }
} // End anonymous namespace

evaluator::evaluator(const long precision)
{
    _precision = precision;
    _constants = initialize_constants(precision);
    _native_fns = basic_builtins();
}

eval_result<evaluator::result_type> evaluator::evaluate(const expression& expr) const
{
    if (const auto* arith = std::get_if<arithmetic_expression>(&expr))
        return to_variant_result(evaluate_arithmetic(*arith));
    if (const auto* bool_exp = std::get_if<boolean_expression>(&expr))
        return to_variant_result(evaluate_boolean(*bool_exp));
    if (const auto* asgn_exp = std::get_if<assignment_expression>(&expr))
        return to_variant_result(evaluate_assignment(*asgn_exp));

    throw std::logic_error{ "unreachable" };
}

void evaluator::commit_result(const result_type& result)
{
    if (const auto* num = std::get_if<number>(&result))
        _variables.insert_or_assign("Ans", *num);
    else if (const auto* asgn = std::get_if<assign_result>(&result))
        _variables.insert_or_assign(asgn->variable, asgn->value);
}

eval_result<number> evaluator::evaluate_arithmetic(const arithmetic_expression& expr) const
{
    eval_stack stack;

    for (auto& op : expr.tokens)
    {
        if (const auto* numop = std::get_if<literal_number>(&op))
        {
            stack.push(numop->num);
        }
        else if (const auto* varref = std::get_if<variable_reference>(&op))
        {
            const auto const_it = _constants.find(varref->identifier);
            if (const_it != _constants.end())
            {
                stack.push(const_it->second);
                continue;
            }

            const auto var_it = _variables.find(varref->identifier);
            if (var_it != _variables.end())
            {
                stack.push(var_it->second);
                continue;
            }

            return eval_result<number>{eval_error_type::undefined_variable, varref->position};
        }
        else if (const auto* binop = std::get_if<binary_operator>(&op))
        {
            if (!stack.has_at_least(2))
                return eval_result<number>{eval_error_type::invalid_program, binop->position};

            const number& rhs = stack.pop();
            const eval_error_type err = evaluate_binary_operator(binop, stack.top(), rhs);

            if (err == eval_error_type::none)
                continue;

            return eval_result<number>{err, binop->position};
        }
        else if (const auto* unop = std::get_if<unary_operator>(&op))
        {
            if (!stack.has_at_least(1))
                return eval_result<number>{eval_error_type::invalid_program, unop->position};

            const auto err = evaluate_unary_operation(unop, stack.top());

            if (err == eval_error_type::none)
                continue;

            return eval_result<number>{err, unop->position};
        }
        else if (const auto* fncall = std::get_if<function_call>(&op))
        {
            if (!stack.has_at_least(fncall->arity))
                return eval_result<number>{eval_error_type::invalid_program, fncall->position};

            const auto native_it = _native_fns.find(fncall->identifier);

            if (native_it == _native_fns.end())
                return eval_result<number>{eval_error_type::undefined_function, fncall->position};

            const auto& fns_with_this_name = native_it->second;

            auto err = eval_error_type::bad_arity;

            for (const auto& [arity, fn] : fns_with_this_name)
            {
                if (arity == fncall->arity)
                {
                    err = fn(stack, *this);
                    break;
                }
            }

            if (err == eval_error_type::none)
                continue;

            return eval_result<number>{err, fncall->position};
        }
    }

    if (stack.size() == 1)
    {
        return eval_result{std::move(stack.top())};
    }

    return eval_result<number>{eval_error_type::invalid_program, expr.position};
}

eval_result<bool> evaluator::evaluate_boolean(const boolean_expression& expr) const
{
    const eval_result left = evaluate_arithmetic(expr.lhs);
    if (left.is_error())
        return eval_result<bool>{left.error()};

    const eval_result right = evaluate_arithmetic(expr.rhs);
    if (right.is_error())
        return eval_result<bool>{right.error()};

    if (expr.kind == token_kind::equality)
        return eval_result{left.value() == right.value()};
    if (expr.kind == token_kind::not_equal)
        return eval_result{left.value() != right.value()};

    if (!left.value().is_real())
        return eval_result<bool>{eval_error_type::complex_inequality, expr.lhs.position};

    if (!right.value().is_real())
        return eval_result<bool>{eval_error_type::complex_inequality, expr.rhs.position};

    switch (expr.kind)
    {
        case token_kind::greater_than:
            return eval_result{left.value() > right.value()};
        case token_kind::less_than:
            return eval_result{left.value() < right.value()};
        case token_kind::greater_or_equal:
            return eval_result{left.value() == right.value() || left.value() > right.value()};
        case token_kind::less_or_equal:
            return eval_result{left.value() == right.value() || left.value() < right.value()};
        default:
            return eval_result<bool>{eval_error_type::invalid_program, expr.position};
    }
}

eval_result<assign_result> evaluator::evaluate_assignment(const assignment_expression& expr) const
{
    eval_result res = evaluate_arithmetic(expr.expression);

    if (res.is_error())
        return eval_result<assign_result>{res.error()};

    return eval_result<assign_result>{{expr.variable, std::move(res.mut_value())}};
}

eval_error_type evaluator::evaluate_unary_operation(const unary_operator* op, number& stack_top) const
{
    switch (op->operation)
    {
        case token_kind::radical:
            stack_top.sqrt(stack_top);
            break;

        case token_kind::minus:
            stack_top.negate(stack_top);
            break;

        case token_kind::percent:
            stack_top.div(stack_top, 100);
            break;

        case token_kind::deg:
            convert_angle(stack_top, angle_unit::degrees, _trig_unit);
            break;

        case token_kind::rad:
            convert_angle(stack_top, angle_unit::radians, _trig_unit);
            break;

        case token_kind::grad:
            convert_angle(stack_top, angle_unit::gradians, _trig_unit);
            break;
                
        default:
            return eval_error_type::invalid_program;
    }

    return eval_error_type::none;
}

eval_error_type evaluator::evaluate_binary_operator(const binary_operator* op, number& lhs, const number& rhs)
{
    switch (op->operation)
    {
        case token_kind::plus:
            lhs.add(lhs, rhs);
            break;

        case token_kind::minus:
            lhs.sub(lhs, rhs);
            break;

        case token_kind::multiply:
            lhs.mul(lhs, rhs);
            break;

        case token_kind::divide:
            if (rhs == 0)
                return eval_error_type::divide_by_zero;
            lhs.div(lhs, rhs);
            break;

        case token_kind::exponentiate:
            if (lhs == 0 && rhs == 0)
                return eval_error_type::zero_pow_zero;
            lhs.pow(lhs, rhs);
            break;

        default:
            return eval_error_type::invalid_program;
    }

    return eval_error_type::none;
}
