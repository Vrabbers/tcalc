#include <gtest/gtest.h>

#include "tc_lexer.h"
#include "tc_parser.h"
#include "tc_evaluator.h"

constexpr long precision = 64;

class Errors : public testing::TestWithParam<std::pair<std::string, tcalc::eval_error_type>>
{
public:
    tcalc::evaluator evaluator{precision};
};


TEST_P(Errors, Evaluate)
{
    auto [inputExpression, expectedError] = GetParam();

    tcalc::lexer lexer(inputExpression, true);
    tcalc::parser parser(std::move(lexer), precision);

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

INSTANTIATE_TEST_SUITE_P(
    LogBase, Errors,
    testing::Values(
        std::pair{"log(1,0)", tcalc::eval_error_type::log_base},
        std::pair{"log(1,1)", tcalc::eval_error_type::log_base},
        std::pair{"log(4,1)", tcalc::eval_error_type::log_base}
    ));

INSTANTIATE_TEST_SUITE_P(
    BadArity, Errors,
    testing::Values(
        std::pair{"sin(1,0)", tcalc::eval_error_type::bad_arity},
        std::pair{"cos()", tcalc::eval_error_type::bad_arity},
        std::pair{"log(4,1,4)", tcalc::eval_error_type::bad_arity}
    ));

INSTANTIATE_TEST_SUITE_P(
    ComplexInequality, Errors,
    testing::Values(
        std::pair{"3i<5", tcalc::eval_error_type::complex_inequality},
        std::pair{"2+3i<5", tcalc::eval_error_type::complex_inequality},
        std::pair{"3i<5i", tcalc::eval_error_type::complex_inequality},
        std::pair{"3i<3+5i", tcalc::eval_error_type::complex_inequality},
        std::pair{"2+3i<3+5i", tcalc::eval_error_type::complex_inequality},
        std::pair{"3i>5", tcalc::eval_error_type::complex_inequality},
        std::pair{"2+3i>5", tcalc::eval_error_type::complex_inequality},
        std::pair{"3i>5i", tcalc::eval_error_type::complex_inequality},
        std::pair{"3i>3+5i", tcalc::eval_error_type::complex_inequality},
        std::pair{"2+3i>3+5i", tcalc::eval_error_type::complex_inequality},
        std::pair{"3i<=5", tcalc::eval_error_type::complex_inequality},
        std::pair{"2+3i<=5", tcalc::eval_error_type::complex_inequality},
        std::pair{"3i<=5i", tcalc::eval_error_type::complex_inequality},
        std::pair{"3i<=3+5i", tcalc::eval_error_type::complex_inequality},
        std::pair{"2+3i<=3+5i", tcalc::eval_error_type::complex_inequality},
        std::pair{"3i>=5", tcalc::eval_error_type::complex_inequality},
        std::pair{"2+3i>=5", tcalc::eval_error_type::complex_inequality},
        std::pair{"3i>=5i", tcalc::eval_error_type::complex_inequality},
        std::pair{"3i>=3+5i", tcalc::eval_error_type::complex_inequality},
        std::pair{"2+3i>=3+5i", tcalc::eval_error_type::complex_inequality}
    ));

INSTANTIATE_TEST_SUITE_P(
    OutOfTanDomain, Errors,
    testing::Values(
        std::pair{"tan(90deg)", tcalc::eval_error_type::out_of_tan_domain},
        std::pair{"tan((pi/2)rad)", tcalc::eval_error_type::out_of_tan_domain},
        std::pair{"tan(100grad)", tcalc::eval_error_type::out_of_tan_domain},
        std::pair{"tan(270deg)", tcalc::eval_error_type::out_of_tan_domain},
        std::pair{"tan(-90deg)", tcalc::eval_error_type::out_of_tan_domain},
        std::pair{"tan((23 pi/2)rad)", tcalc::eval_error_type::out_of_tan_domain}
    ));

INSTANTIATE_TEST_SUITE_P(
    ZeroPowZero, Errors,
    testing::Values(
        std::pair{"0^0", tcalc::eval_error_type::zero_pow_zero}
    ));

INSTANTIATE_TEST_SUITE_P(
    AssignToConstant, Errors,
    testing::Values(
        std::pair{"e=2", tcalc::eval_error_type::assign_to_constant}
    ));

INSTANTIATE_TEST_SUITE_P(
    ZeroRoot, Errors,
    testing::Values(
        std::pair{"root(5, 0)", tcalc::eval_error_type::zero_root}
    ));