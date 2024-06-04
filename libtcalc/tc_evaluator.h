#ifndef TC_EVALUATOR_H
#define TC_EVALUATOR_H

#include <functional>
#include <string>
#include <unordered_map>

#include "tc_eval_result.h"
#include "tc_eval_stack.h"
#include "tc_expression.h"
#include "tc_number.h"

namespace tcalc
{
    enum class angle_unit
    {
        radians,
        degrees,
        gradians
    };

    class evaluator final
    {
    public:
        struct native_fn
        {
            fn_arity_t arity;
            std::function<eval_error_type(eval_stack&, const evaluator&)> fn;
        };

        using result_type = std::variant<assign_result, number, bool>;

        explicit evaluator(long precision);

        [[nodiscard]]
        angle_unit trig_unit() const
        {
            return _trig_unit;
        }

        void trig_unit(const angle_unit unit)
        {
            _trig_unit = unit;
        }

        [[nodiscard]]
        long precision() const
        {
            return _precision;
        }

        [[nodiscard]]
        eval_result<result_type> evaluate(const expression& expr) const;

        void commit_result(const result_type& result);
        
        [[nodiscard]]
        eval_result<number> evaluate_arithmetic(const arithmetic_expression& expr) const;
        
        [[nodiscard]]
        eval_result<bool> evaluate_boolean(const boolean_expression& expr) const;

        [[nodiscard]]
        eval_result<assign_result> evaluate_assignment(const assignment_expression& expr) const;

    private:
        eval_error_type evaluate_unary_operation(const unary_operator* op, number& stack_top) const;
        static eval_error_type evaluate_binary_operator(const binary_operator* op, number& lhs, const number& rhs);

        long _precision;
        angle_unit _trig_unit = angle_unit::degrees;
        std::unordered_map<std::string, number> _constants;
        std::unordered_map<std::string, number> _variables;
        std::unordered_map<std::string, std::vector<native_fn>> _native_fns;
    };
}

#endif // TC_EVALUATOR_H
