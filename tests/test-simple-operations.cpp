#include <gtest/gtest.h>

#include <tc_lexer.h>
#include <tc_parser.h>
#include <tc_evaluator.h>

#define PRECISION 64

class SimpleOperations : public ::testing::TestWithParam<std::pair<std::string, std::string>>
{
public:
    tcalc::evaluator evaluator{PRECISION};
};

TEST_P(SimpleOperations, Evaluate) {
    auto [inputExpression, expectedExpression] = GetParam();

    tcalc::lexer lexer(inputExpression, false);
    tcalc::parser parser(std::move(lexer), PRECISION);

    auto expr = parser.parse_expression();
    ASSERT_TRUE(parser.diagnostic_bag().empty());

    auto result = evaluator.evaluate(expr);
    ASSERT_FALSE(result.is_error());

    auto number = std::get_if<tcalc::number>(&result.value());
    ASSERT_TRUE(number);
    ASSERT_EQ(number->string(), expectedExpression);
}

INSTANTIATE_TEST_SUITE_P(
    AdditionOperations, SimpleOperations,
    testing::Values(
        std::pair{"2+2", "4"},
        std::pair{"-2+2", "0"},
        std::pair{"2+-2", "0"}
    ));
INSTANTIATE_TEST_SUITE_P(
    SubtractionOperations, SimpleOperations,
    testing::Values(
        std::pair{"2-2", "0"},
        std::pair{"-2-2", "-4"},
        std::pair{"2--2", "4"}
    ));
INSTANTIATE_TEST_SUITE_P(
    MultiplicationOperations, SimpleOperations,
    testing::Values(
        std::pair{"2*2", "4"},
        std::pair{"2*-2", "-4"},
        std::pair{"-2*2", "-4"},
        std::pair{"-2*-2", "4"}
    ));
INSTANTIATE_TEST_SUITE_P(
    DivisonOperations, SimpleOperations,
    testing::Values(
        std::pair{"2/2", "1"},
        std::pair{"1/2", "0.5"},
        std::pair{"-2/4", "-0.5"},
        std::pair{"2/-4", "-0.5"}
    ));