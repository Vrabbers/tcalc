#include <gtest/gtest.h>

#include "tc_lexer.h"
#include "tc_parser.h"
#include "tc_evaluator.h"

constexpr long precision = 64;

class RealModeErrors : public testing::TestWithParam<std::pair<std::string, tcalc::eval_error_type>>
{
public:
    tcalc::evaluator evaluator{precision};
};


TEST_P(RealModeErrors, Evaluate)
{
    auto [inputExpression, expectedError] = GetParam();

    tcalc::lexer lexer(inputExpression, true);
    tcalc::parser parser(std::move(lexer), precision);

    auto expr = parser.parse_expression();
    ASSERT_TRUE(parser.diagnostic_bag().empty());

    evaluator.complex_mode(false);
    auto result = evaluator.evaluate(expr);
    ASSERT_TRUE(result.is_error());
    ASSERT_EQ(result.error().type, expectedError);
}

INSTANTIATE_TEST_SUITE_P(
    RealModeComplexResult, RealModeErrors,
    testing::Values(
        std::pair{"sqrt(-1)", tcalc::eval_error_type::real_mode_complex_result},
        std::pair{"root(-1, 4)", tcalc::eval_error_type::real_mode_complex_result},
        std::pair{"ln(-1) - ln(-1)", tcalc::eval_error_type::real_mode_complex_result},
        std::pair{"3+2i", tcalc::eval_error_type::real_mode_complex_result}
    ));