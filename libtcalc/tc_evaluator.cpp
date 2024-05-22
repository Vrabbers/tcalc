#include "tc_evaluator.h"

#include "tc_eval_result.h"

using namespace tcalc;

namespace
{
    struct eval_stack
    {
        std::vector<number> _stack;
        unsigned int _top = -1;

        [[nodiscard]]
        number& top()
        {
            return _stack.at(_top);
        }

        [[nodiscard]]
        bool has_at_least(const unsigned int amt) const
        {
            return amt <= _top + 1;
        }

        [[nodiscard]]
        int size() const
        {
            return _top + 1;
        }

        const number& pop()
        {
            // We keep old entries to avoid creating and destroying perfectly good number entries.
            const auto& num = _stack.at(_top);
            _top--;
            return num;
        }

        void push(const number& num)
        {
            _top++;
            if (_stack.size() == _top)
            {
                number num2{num};
                _stack.push_back(std::move(num2));
            }
            else
            {
                _stack.at(_top).set(num);
            }
        }
    };

    using namespace std::string_view_literals;

    std::unordered_map<std::string_view, const number> initialize_constants(const mpfr_prec_t prec)
    {
        return {
            {"pi"sv, number::pi(prec)},
            {"π"sv, number::pi(prec)},
            {"tau"sv, number::tau(prec)},
            {"τ"sv, number::tau(prec)},
            {"e"sv, number::e(prec)}
        };
    }

    struct arity_fn_pair
    {
        fn_arity_t arity;
        std::function<eval_error_type(eval_stack&)> fn;
    };

    using builtin_fn_vec = std::vector<arity_fn_pair>;
    const std::unordered_map<std::string_view, builtin_fn_vec> basic_builtins =
    {
        {
            "sqrt"sv, 
            {
                { 1, 
                [](eval_stack& stack)
                    {
                        stack.top().sqrt();
                        return eval_error_type::none;
                    }
                }
            }
        },
        {
            "exp"sv, 
            {
                { 1, 
                [](eval_stack& stack)
                    {
                        stack.top().exp();
                        return eval_error_type::none;
                    }
                }
            }
        },
        {
            "log"sv, 
            {
                { 1, 
                [](eval_stack& stack)
                    {
                        if (stack.top() == 0)
                            return eval_error_type::log_zero;
                        stack.top().log();
                        return eval_error_type::none;
                    }
                },
                { 2,
                [](eval_stack& stack)
                    {
                        if(stack.top() == 0 || stack.top() == 1)
                            return eval_error_type::log_base;
                        auto& base = stack.top(); // Take mutable ref
                        stack.pop();
                        if (stack.top() == 0)
                            return eval_error_type::log_zero;
                        base.ln();
                        stack.top().ln();
                        stack.top().div(base);
                        return eval_error_type::none;
                    }
                }
            }
        },
        {
            "ln"sv, 
            {
                { 1, 
                [](eval_stack& stack)
                    {
                        if (stack.top() == 0)
                            return eval_error_type::log_zero;
                        stack.top().ln();
                        return eval_error_type::none;
                    }
                }
            }
        },
        {
            "sin"sv, 
            {
                { 1, 
                [](eval_stack& stack)
                    {
                        stack.top().sin();
                        return eval_error_type::none;
                    }
                }
            }
        },
        {
            "cos"sv, 
            {
                { 1, 
                [](eval_stack& stack)
                    {
                        stack.top().cos();
                        return eval_error_type::none;
                    }
                }
            }
        },
        {
            "tan"sv, 
            {
                { 1, 
                [](eval_stack& stack)
                    {
                        stack.top().tan();
                        return eval_error_type::none;
                    }
                }
            }
        },
    };
}

evaluator::evaluator(const mpfr_prec_t precision)
{
    _precision = precision;
    _constants = initialize_constants(precision);
}


static eval_error_type evaluate_binop(const binary_operator* op, number& lhs, const number& rhs)
{
    switch (op->operation)
    {
        case token_kind::plus:
            lhs.add(rhs);
            break;

        case token_kind::minus:
            lhs.sub(rhs);
            break;

        case token_kind::multiply:
            lhs.mul(rhs);
            break;

        case token_kind::divide:
            if (rhs == 0)
                return eval_error_type::divide_by_zero;
            lhs.div(rhs);
            break;

        case token_kind::exponentiate:
            lhs.pow(rhs);
            break;

        default:
            return eval_error_type::invalid_program;
    }

    return eval_error_type::none;
}

static eval_error_type evaluate_unop(const unary_operator* op, number& stack_top)
{
    switch(op->operation)
    {
        case token_kind::radical:
            stack_top.sqrt();
            break;

        case token_kind::minus:
            stack_top.negate();
            break;

        default:
            return eval_error_type::invalid_program;
    }

    return eval_error_type::none;
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
            auto const_it = _constants.find(varref->identifier);
            if (const_it != _constants.end())
                stack.push(const_it->second);
            else
                return eval_result<number>::from_error(eval_error_type::undefined_variable, varref->position);
        }
        else if (const auto* binop = std::get_if<binary_operator>(&op))
        {
            if (!stack.has_at_least(2))
                return eval_result<number>::from_error(eval_error_type::invalid_program, binop->position);

            const number& rhs = stack.pop();
            const auto err = evaluate_binop(binop, stack.top(), rhs);

            if (err == eval_error_type::none)
                continue;

            return eval_result<number>::from_error(err, binop->position);
        }
        else if (const auto* unop = std::get_if<unary_operator>(&op))
        {
            if (!stack.has_at_least(1))
                return eval_result<number>::from_error(eval_error_type::invalid_program, unop->position);

            const auto err = evaluate_unop(unop, stack.top());

            if (err == eval_error_type::none)
                continue;

            return eval_result<number>::from_error(err, unop->position);
        }
        else if (const auto* fncall = std::get_if<function_call>(&op))
        {
            if (!stack.has_at_least(fncall->arity))
                return eval_result<number>::from_error(eval_error_type::invalid_program, fncall->position);

            const auto builtin_it = basic_builtins.find(fncall->identifier);

            if (builtin_it == basic_builtins.end())
                return eval_result<number>::from_error(eval_error_type::undefined_function, fncall->position);

            const auto fns_with_this_name = builtin_it->second;

            auto err = eval_error_type::bad_arity;

            for (const auto& fn_arity : fns_with_this_name)
            {
                if (fn_arity.arity == fncall->arity)
                {
                    err = fn_arity.fn(stack);
                    break;
                }
            }

            if (err == eval_error_type::none)
                continue;
            
            return eval_result<number>::from_error(err, fncall->position);
        }
    }

    if (stack.size() == 1)
        return eval_result{std::move(stack.top())};

    return eval_result<number>::from_error(eval_error_type::invalid_program, expr.position);
}

eval_result<bool> evaluator::evaluate_boolean(const boolean_expression &expr) const
{
    const eval_result left = evaluate_arithmetic(expr.lhs);
    if (left.is_error())
        return eval_result<bool>::from_error(left.error());

    const eval_result right = evaluate_arithmetic(expr.rhs);
    if (right.is_error())
        return eval_result<bool>::from_error(right.error());

    if (expr.kind == token_kind::equal)
        return eval_result{left.value() == right.value()};
    if (expr.kind == token_kind::not_equal)
        return eval_result{left.value() != right.value()};
        
    if (!left.value().is_real())
        return eval_result<bool>::from_error(eval_error_type::complex_inequality, expr.lhs.position);

    if (!right.value().is_real())
        return eval_result<bool>::from_error(eval_error_type::complex_inequality, expr.rhs.position);
    
    switch(expr.kind)
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
            return eval_result<bool>::from_error(eval_error_type::invalid_program, expr.position);
    }
}

eval_result<empty_result> tcalc::evaluator::evaluate_assignment(const assignment_expression &expr)
{
    return eval_result<empty_result>();
}

eval_result<empty_result> tcalc::evaluator::evaluate_fn_def(const func_def_expression &expr)
{
    return eval_result<empty_result>();
}
