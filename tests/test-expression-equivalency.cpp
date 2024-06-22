#include <gtest/gtest.h>

#include "tc_lexer.h"
#include "tc_parser.h"
#include "tc_evaluator.h"

constexpr long precision = 64;

class ExpressionEquivalency : public testing::TestWithParam<std::pair<std::string, std::string>>
{
public:
  tcalc::evaluator evaluator{precision};
};

TEST_P(ExpressionEquivalency, Evaluate)
{
  auto [inputExpression, expectedExpression] = GetParam();

  tcalc::lexer inputLexer(inputExpression, true);
  tcalc::parser inputParser(std::move(inputLexer), precision);

  auto inputExpr = inputParser.parse_expression();
  ASSERT_TRUE(inputParser.diagnostic_bag().empty());

  auto inputResult = evaluator.evaluate(inputExpr);
  ASSERT_FALSE(inputResult.is_error());

  auto inputNumber = std::get_if<tcalc::number>(&inputResult.value());
  ASSERT_TRUE(inputNumber);

  tcalc::lexer expectedLexer(expectedExpression, true);
  tcalc::parser expectedParser(std::move(expectedLexer), precision);

  auto expectedExpr = expectedParser.parse_expression();
  ASSERT_TRUE(expectedParser.diagnostic_bag().empty());

  auto expectedResult = evaluator.evaluate(expectedExpr);
  ASSERT_FALSE(expectedResult.is_error());

  auto expectedNumber = std::get_if<tcalc::number>(&expectedResult.value());
  ASSERT_TRUE(expectedNumber);

  ASSERT_EQ(inputNumber->string(), expectedNumber->string());
}

INSTANTIATE_TEST_SUITE_P(
    SecantOperations, ExpressionEquivalency,
    testing::Values(
        std::pair{"sec(5)", "1/cos(5)"}
    ));

INSTANTIATE_TEST_SUITE_P(
    CosecantOperations, ExpressionEquivalency,
    testing::Values(
        std::pair{"csc(5)", "1/sin(5)"}
        ));

INSTANTIATE_TEST_SUITE_P(
    CotangentOperations, ExpressionEquivalency,
    testing::Values(
        std::pair{"cot(5)", "1/tan(5)"},
        std::pair{"cot(5)", "cos(5)/sin(5)"}
        ));