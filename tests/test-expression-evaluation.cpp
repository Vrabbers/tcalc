#include <gtest/gtest.h>

#include "tc_lexer.h"
#include "tc_parser.h"
#include "tc_evaluator.h"

constexpr long precision = 64;

class ExpressionEvaluation : public testing::TestWithParam<std::pair<std::string, std::string>>
{
public:
    tcalc::evaluator evaluator{precision};
};

TEST_P(ExpressionEvaluation, Evaluate)
{
    auto [inputExpression, expectedExpression] = GetParam();

    tcalc::lexer lexer(inputExpression, true);
    tcalc::parser parser(std::move(lexer), precision);

    auto expr = parser.parse_expression();
    ASSERT_TRUE(parser.diagnostic_bag().empty());

    auto result = evaluator.evaluate(expr);
    ASSERT_FALSE(result.is_error());

    auto number = std::get_if<tcalc::number>(&result.value());
    ASSERT_TRUE(number);
    ASSERT_EQ(number->string(), expectedExpression);
}

INSTANTIATE_TEST_SUITE_P(
    AdditionOperations, ExpressionEvaluation,
    testing::Values(
        std::pair{"2+2", "4"},
        std::pair{"-2+2", "0"},
        std::pair{"2+-2", "0"}
    ));
INSTANTIATE_TEST_SUITE_P(
    SubtractionOperations, ExpressionEvaluation,
    testing::Values(
        std::pair{"2-2", "0"},
        std::pair{"-2-2", "-4"},
        std::pair{"2--2", "4"}
    ));
INSTANTIATE_TEST_SUITE_P(
    MultiplicationOperations, ExpressionEvaluation,
    testing::Values(
        std::pair{"2*2", "4"},
        std::pair{"2*-2", "-4"},
        std::pair{"-2*2", "-4"},
        std::pair{"-2*-2", "4"}
    ));
INSTANTIATE_TEST_SUITE_P(
    DivisonOperations, ExpressionEvaluation,
    testing::Values(
        std::pair{"2/2", "1"},
        std::pair{"1/2", "0.5"},
        std::pair{"-2/4", "-0.5"},
        std::pair{"2/-4", "-0.5"}
    ));
INSTANTIATE_TEST_SUITE_P(
    OrderOfOperations, ExpressionEvaluation,
    testing::Values(
        std::pair{"2+3^2*4", "38"}
    ));
INSTANTIATE_TEST_SUITE_P(
    TrigonometricOperations, ExpressionEvaluation,
    testing::Values(
        std::pair{"asin(sin(30))", "30"},
        std::pair{"acos(cos(30))", "30"},
        std::pair{"atan(tan(30))", "30"},
        std::pair{"asec(sec(30))", "30"},
        std::pair{"acsc(csc(30))", "30"},
        std::pair{"acot(cot(30))", "30"},
        std::pair{"asinh(sinh(30))", "30"},
        std::pair{"acosh(cosh(30))", "30"},
        std::pair{"atanh(tanh(1))", "1"},
        std::pair{"asech(sech(30))", "30"},
        std::pair{"acsch(csch(30))", "30"},
        std::pair{"acoth(coth(1))", "1"}
        ));