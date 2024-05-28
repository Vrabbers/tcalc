#ifndef TC_EVALUATOR_H
#define TC_EVALUATOR_H

#include <functional>
#include <string>
#include <unordered_map>

#include "tc_eval_result.h"
#include "tc_expression.h"
#include "tc_number.h"



namespace tcalc {
    class evaluator final
    {
    public:
        using result_type = std::variant<empty_result, number, bool>;

        explicit evaluator(mpfr_prec_t precision);
        
        [[nodiscard]]
        eval_result<result_type> evaluate(const expression& expr);
        
        [[nodiscard]]
        eval_result<number> evaluate_arithmetic(const arithmetic_expression& expr) const;
        
        [[nodiscard]]
        eval_result<bool> evaluate_boolean(const boolean_expression& expr) const;

        [[nodiscard]]
        eval_result<empty_result> evaluate_assignment(const assignment_expression& expr);

        [[nodiscard]]
        eval_result<empty_result> evaluate_fn_def(const func_def_expression& expr);

    private:
        mpfr_prec_t _precision;
        std::unordered_map<std::string_view, const number> _constants;
        std::unordered_map<std::string, number> _variables;
        std::unordered_map<std::string, func_def_expression> _user_def_fns;
    };

}

#endif // TC_EVALUATOR_H
