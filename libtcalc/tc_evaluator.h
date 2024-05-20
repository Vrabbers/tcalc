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
        explicit evaluator(mpfr_prec_t precision);
        [[nodiscard]]
        eval_result<number> evaluate_arithmetic(const arithmetic_expression& expr) const;

    private:
        mpfr_prec_t _precision;
        std::unordered_map<std::string, const number> _constants;
        std::unordered_map<std::string, number> _variables;
    };

}

#endif // TC_EVALUATOR_H
