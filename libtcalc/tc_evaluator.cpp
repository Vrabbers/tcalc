#include "tc_evaluator.h"

#include <stdexcept>

#ifdef _MSC_VER
#pragma warning(push, 0) // mpc header has warnings on MSVC /W4
#endif

#include <mpc.h>

#ifdef _MSC_VER
#pragma warning(pop)
#endif

#include "tc_eval_result.h"
#include "internal/builtins.h"

using namespace tcalc;

// TODO: Make sure root implementations are correct & match for all cases.

namespace
{
    using namespace std::string_literals;
    using namespace std::string_view_literals;

    std::map<std::string, number> initialize_constants(const mpfr_prec_t prec)
    {
        return {
            {"pi"s, number::pi(prec)},
            {"π"s, number::pi(prec)},
            {"tau"s, number::tau(prec)},
            {"τ"s, number::tau(prec)},
            {"e"s, number::e(prec)}
        };
    }

    std::map<std::string, std::vector<evaluator::native_fn>> basic_builtins()
    {
        return
        {
            {"sqrt"s, {{1, &builtin_sqrt}}},
            {"cbrt"s, {{1, &builtin_cbrt}}},
            {"root"s, {{2, &builtin_root}}},
            {"exp"s, {{1, &builtin_exp}}},
            {
                "log"s,
                {
                    {1, &builtin_log1},
                    {2, &builtin_log2}
                }
            },
            {"ln"s, {{1, &builtin_ln}}},
            {"sin"s, {{1, &builtin_sin}}},
            {"cos"s, {{1, &builtin_cos}}},
            {"tan"s, {{1, &builtin_tan}}},
            {"abs"s, {{1, &builtin_abs}}},
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

    eval_error_type check_finite(const number& num, const bool complex_mode)
    {
        if (!complex_mode && !num.is_real())
            return eval_error_type::real_mode_complex_result;
        if (num.is_infinity())
            return eval_error_type::overflow;
        if (num.is_nan())
            return eval_error_type::nan_error;
        return eval_error_type::none;
    }

} // End anonymous namespace

evaluator::evaluator(const long precision) :
    _precision{precision},
    _constants{initialize_constants(_precision)},
    _native_fns{basic_builtins()}
{
}

eval_result<evaluator::result_type> evaluator::evaluate(const expression& expr) const
{
    if (const auto* arith = std::get_if<arithmetic_expression>(&expr))
        return to_variant_result(evaluate_arithmetic(*arith));
    if (const auto* bool_exp = std::get_if<boolean_expression>(&expr))
        return to_variant_result(evaluate_boolean(*bool_exp));
    if (const auto* asgn_exp = std::get_if<assignment_expression>(&expr))
        return to_variant_result(evaluate_assignment(*asgn_exp));

    throw std::logic_error{"unreachable"};
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
    stack stack;

    for (auto& op : expr.tokens)
    {
        if (const auto* numop = std::get_if<literal_number>(&op))
        {
            stack.push_back(numop->num);
            const eval_error_type err = check_finite(stack.back(), _complex_mode);
            if (err != eval_error_type::none)
                return eval_result<number>{err, numop->position};
        }
        else if (const auto* varref = std::get_if<variable_reference>(&op))
        {
            const auto const_it = _constants.find(varref->identifier);
            if (const_it != _constants.end())
            {
                stack.push_back(const_it->second);
                continue;
            }

            const auto var_it = _variables.find(varref->identifier);
            if (var_it != _variables.end())
            {
                stack.push_back(var_it->second);
                continue;
            }

            return eval_result<number>{eval_error_type::undefined_variable, varref->position};
        }
        else if (const auto* binop = std::get_if<binary_operator>(&op))
        {
            if (stack.size() < 2)
                return eval_result<number>{eval_error_type::invalid_program, binop->position};

            eval_error_type err = evaluate_binary_operator(binop, stack);
            if (err == eval_error_type::none)
            {
                err = check_finite(stack.back(), _complex_mode);
                if (err == eval_error_type::none)
                    continue;
            }

            return eval_result<number>{err, binop->position};
        }
        else if (const auto* unop = std::get_if<unary_operator>(&op))
        {
            if (stack.empty())
                return eval_result<number>{eval_error_type::invalid_program, unop->position};

            eval_error_type err = evaluate_unary_operation(unop, stack);

            if (err == eval_error_type::none)
            {
                err = check_finite(stack.back(), _complex_mode);
                if (err == eval_error_type::none)
                    continue;
            }

            return eval_result<number>{err, unop->position};
        }
        else if (const auto* fncall = std::get_if<function_call>(&op))
        {
            if (static_cast<fn_arity_t>(stack.size()) < fncall->arity)
                return eval_result<number>{eval_error_type::invalid_program, fncall->position};

            const auto native_it = _native_fns.find(fncall->identifier);

            if (native_it == _native_fns.end())
                return eval_result<number>{eval_error_type::undefined_function, fncall->position};

            const auto& fns_with_this_name = native_it->second;

            eval_error_type err = eval_error_type::bad_arity;

            for (const auto& [arity, fn] : fns_with_this_name)
            {
                if (arity == fncall->arity)
                {
                    err = fn(stack, *this);
                    break;
                }
            }

            if (err == eval_error_type::none)
            {
                err = check_finite(stack.back(), _complex_mode);
                if (err == eval_error_type::none)
                    continue;
            }

            return eval_result<number>{err, fncall->position};
        }
    }

    if (stack.size() == 1)
        return eval_result{std::move(stack.back())};

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
    if (_constants.contains(expr.variable))
        return eval_result<assign_result>{eval_error_type::assign_to_constant, expr.position};

    eval_result res = evaluate_arithmetic(expr.expression);

    if (res.is_error())
        return eval_result<assign_result>{res.error()};

    return eval_result<assign_result>{{expr.variable, std::move(res.mut_value())}};
}

eval_error_type evaluator::evaluate_unary_operation(const unary_operator* op, stack& stack) const
{
    switch (op->operation)
    {
        case token_kind::radical:
            stack.back().sqrt(stack.back());
            break;

        case token_kind::cube_root:
            stack.back().nth_root(stack.back(), 3);
            break;

        case token_kind::fourth_root:
            stack.back().nth_root(stack.back(), 4);
            break;

        case token_kind::minus:
            stack.back().negate(stack.back());
            break;

        case token_kind::percent:
            stack.back().div(stack.back(), 100);
            break;

        case token_kind::deg:
            convert_angle(stack.back(), angle_unit::degrees, _trig_unit);
            break;

        case token_kind::rad:
            convert_angle(stack.back(), angle_unit::radians, _trig_unit);
            break;

        case token_kind::grad:
            convert_angle(stack.back(), angle_unit::gradians, _trig_unit);
            break;
                
        default:
            return eval_error_type::invalid_program;
    }

    return eval_error_type::none;
}

template <void (number::* Fn)(const number&, const number&)>
void basic_binop(evaluator::stack& stack)
{
    const number rhs = std::move(stack.back());
    stack.pop_back();
    (stack.back().*Fn)(stack.back(), rhs);
}

eval_error_type evaluator::evaluate_binary_operator(const binary_operator* op, stack& stack) const
{
    if (op->operation == token_kind::radical)
    {
        if (stack.back() == 0)
            return eval_error_type::zero_root;
        if (stack.back() == 2)
            return builtin_sqrt(stack, *this);

        return builtin_root(stack, *this);
    }

    const number rhs = std::move(stack.back());
    stack.pop_back();
    number& lhs = stack.back();

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
