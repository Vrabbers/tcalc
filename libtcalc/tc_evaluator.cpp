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

static std::unordered_map<std::string, const number> initialize_constants(const mpfr_prec_t prec)
{
    return {
        {"pi", number::pi(prec)},
        {"π", number::pi(prec)},
        {"tau", number::tau(prec)},
        {"τ", number::tau(prec)},
        {"e", number::e(prec)}
    };
}

using namespace std::string_view_literals;

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


eval_result<number> evaluator::evaluate_arithmetic(const arithmetic_expression& expr) const
{
    eval_stack stack;

    for (auto& op : expr.tokens)
    {
        if (const literal_number* numop = std::get_if<literal_number>(&op))
        {
            stack.push(numop->num);
        }
        else if (const binary_operator* binop = std::get_if<binary_operator>(&op))
        {
            if (!stack.has_at_least(2))
                return eval_result<number>::from_error(eval_error_type::invalid_program, binop->position);
            switch (binop->operation)
            {
                case token_kind::plus:
                    {
                        const auto& rhs = stack.pop();
                        stack.top().add(stack.top(), rhs);
                        break;
                    }

                default:
                    return eval_result<number>::from_error(eval_error_type::invalid_program, binop->position);
            }
        }
    }

    if (stack.size() == 1)
        return eval_result{std::move(stack.top())};

    return eval_result<number>::from_error(eval_error_type::invalid_program, expr.position);
}
