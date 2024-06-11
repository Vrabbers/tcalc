#ifndef TC_EVALUATOR_H
#define TC_EVALUATOR_H

#include <functional>
#include <map>
#include <string>

#include "tc_eval_result.h"
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
        using result_type = std::variant<assign_result, number, bool>;

        using stack = std::vector<number>;

        struct native_fn
        {
            fn_arity_t arity;
            std::function<eval_error_type(stack&, const evaluator&)> fn;
        };

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
        bool complex_mode() const
        {
            return _complex_mode;
        }

        void complex_mode(const bool mode)
        {
            _complex_mode = mode;
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
        eval_error_type evaluate_unary_operation(const unary_operator* op, stack& stack) const;
        eval_error_type evaluate_binary_operator(const binary_operator* op, stack& stack) const;

        long _precision;
        bool _complex_mode = true;
        angle_unit _trig_unit = angle_unit::degrees;
        std::map<std::string, number> _constants;
        std::map<std::string, number> _variables;
        std::map<std::string, std::vector<native_fn>> _native_fns;
    };
}

#endif // TC_EVALUATOR_H
