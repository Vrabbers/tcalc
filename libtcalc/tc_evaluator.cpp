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
}

using namespace std::string_view_literals;

static std::unordered_map<std::string_view, const number> initialize_constants(const mpfr_prec_t prec)
{
    return {
        {"pi"sv, number::pi(prec)},
        {"π"sv, number::pi(prec)},
        {"tau"sv, number::tau(prec)},
        {"τ"sv, number::tau(prec)},
        {"e"sv, number::e(prec)}
    };
}

static const std::unordered_map<std::string_view, std::function<eval_error_type(eval_stack&)> > basic_builtins =
{
    {
        "sqrt"sv, [](eval_stack& stack)
        {
            stack.top().sqrt(stack.top());
            return eval_error_type::none;
        }
    },
    {
        "exp"sv, [](eval_stack& stack)
        {
            stack.top().exp(stack.top());
            return eval_error_type::none;
        }
    },
    {
        "log"sv, [](eval_stack& stack)
        {
            // TODO: check if zero
            stack.top().log(stack.top());
            return eval_error_type::none;
        }
    },
    {
        "ln"sv, [](eval_stack& stack)
        {
            // TODO: check if zero
            stack.top().ln(stack.top());
            return eval_error_type::none;
        }
    }
};

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
            lhs.add(lhs, rhs);
            break;

        case token_kind::multiply:
            lhs.mul(lhs, rhs);
            break;

        case token_kind::exponentiate:
            lhs.pow(lhs, rhs);
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
            stack_top.sqrt(stack_top);
            break;

        case token_kind::minus:
            stack_top.negate(stack_top);
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
            const eval_error_type err = evaluate_binop(binop, stack.top(), rhs);

            if (err == eval_error_type::none)
                continue;

            return eval_result<number>::from_error(err, binop->position);
        }
        else if (const auto* unop = std::get_if<unary_operator>(&op))
        {
            if (!stack.has_at_least(1))
                return eval_result<number>::from_error(eval_error_type::invalid_program, unop->position);

            const eval_error_type err = evaluate_unop(unop, stack.top());

            if (err == eval_error_type::none)
                continue;

            return eval_result<number>::from_error(err, unop->position);
        }
    }

    if (stack.size() == 1)
        return eval_result{std::move(stack.top())};

    return eval_result<number>::from_error(eval_error_type::invalid_program, expr.position);
}
