#include <gtest/gtest.h>

#include <tc_lexer.h>
#include <tc_parser.h>
#include <tc_evaluator.h>

#define PRECISION 64

class Errors : public ::testing::TestWithParam<std::pair<std::string, tcalc::eval_error_type>>
{
public:
  tcalc::evaluator evaluator{PRECISION};
};


TEST_P(Errors, Evaluate) {
  auto [inputExpression, expectedError] = GetParam();

  tcalc::lexer lexer(inputExpression, true);
  tcalc::parser parser(std::move(lexer), PRECISION);

  auto expr = parser.parse_expression();
  ASSERT_TRUE(parser.diagnostic_bag().empty());

  auto result = evaluator.evaluate(expr);
  ASSERT_TRUE(result.is_error());
  ASSERT_EQ(result.error().type, expectedError);
}

INSTANTIATE_TEST_SUITE_P(
    DivisionByZero, Errors,
    testing::Values(
        std::pair{"1/0", tcalc::eval_error_type::divide_by_zero},
        std::pair{"2/0", tcalc::eval_error_type::divide_by_zero},
        std::pair{"4/(1-1)", tcalc::eval_error_type::divide_by_zero}
    ));

INSTANTIATE_TEST_SUITE_P(
    LogZero, Errors,
    testing::Values(
        std::pair{"log(0)", tcalc::eval_error_type::log_zero},
        std::pair{"ln(0)", tcalc::eval_error_type::log_zero},
        std::pair{"log(0,5)", tcalc::eval_error_type::log_zero}
    ));

INSTANTIATE_TEST_SUITE_P(
    UndefinedVariable, Errors,
    testing::Values(
        std::pair{"x", tcalc::eval_error_type::undefined_variable},
        std::pair{"3+x", tcalc::eval_error_type::undefined_variable}
    ));

INSTANTIATE_TEST_SUITE_P(
    UndefinedFunction, Errors,
    testing::Values(
        std::pair{"funky(3)", tcalc::eval_error_type::undefined_function},
        std::pair{"funky(3,5)", tcalc::eval_error_type::undefined_function}
    ));