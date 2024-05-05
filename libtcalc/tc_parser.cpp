#include "tc_parser.h"

#include <stdexcept>

using namespace tcalc;

static int binary_precedence(const token_kind kind)
{
    switch (kind)
    {
        case token_kind::exponentiate:
            return 4;
        case token_kind::multiply:
        case token_kind::divide:
            return 2;
        case token_kind::minus:
        case token_kind::plus:
            return 1;
        default:
            return -1;
    }
}

static int unary_precendence(const token_kind kind)
{
    switch (kind)
    {
        case token_kind::binary_not:
        case token_kind::minus:
        case token_kind::plus:
        case token_kind::radical:
            return 3;
        default:
            return -1;
    }
}

static bool is_right_associative(const token_kind kind)
{
    return kind == token_kind::exponentiate;
}

static bool ends_arith(const token_kind kind)
{
    switch (kind)
    {
        case token_kind::expression_separator:
        case token_kind::end_of_file:
            return true;
        default:
            return false;
    }
}

static bool is_valid_func_def(const std::vector<operation>& parse)
{
    if (!std::holds_alternative<function_call>(parse.back()))
        return false;

    if (std::get<function_call>(parse.back()).arity != static_cast<int32_t>(parse.size() - 1)) // 1 token for each ident + 1 for the fn call
        return false;

    for (int32_t i = 0; i < static_cast<int32_t>(parse.size() - 1); i++)
    {
        if (!std::holds_alternative<variable_reference>(parse[i]))
            return false;
    }
    return true;
}

static std::vector<std::string> collect_arg_names(std::vector<operation>& parse)
{
    std::vector<std::string> idents;
    for (int32_t i = 0; i < static_cast<int32_t>(parse.size() - 1); i++)
    {
        idents.emplace_back(std::move(std::get<variable_reference>(parse[i]).identifier));
    }
    return idents;
}

static bool can_insert_implicit_multiply(const token_kind kind)
{
    switch (kind)
    {
        case token_kind::identifier:
        case token_kind::open_parenthesis:
            return true;
        default:
            return unary_precendence(kind) != -1;
    }
}

expression parser::parse_expression()
{
    auto expect_end = [&]
    {
        if (!ends_arith(_current.kind()))
            unexpected_token(_current);
        forward();
    };
    std::vector<operation> lhs_parse;
    parse_arithmetic(lhs_parse); // Parse full expression or left hand side of function call or assignment
    const auto delimiter = forward();
    switch (delimiter.kind())
    {
        case token_kind::expression_separator:
        case token_kind::end_of_file:
            return arithmetic_expression{std::move(lhs_parse)};
        case token_kind::equal:
            // can be assignment, or function def, or boolean expr
            if (lhs_parse.size() == 1 && std::holds_alternative<variable_reference>(lhs_parse[0]))
            {
                // this is a variable assignment expression 😁
                std::vector<operation> rhs_parse;
                parse_arithmetic(rhs_parse);
                expect_end();
                auto var = std::get<variable_reference>(lhs_parse[0]).identifier;
                return assignment_expression{std::move(var), {std::move(rhs_parse)}};
            }
            if (is_valid_func_def(lhs_parse))
            {
                std::vector<operation> def_parse;
                parse_arithmetic(def_parse);
                expect_end();
                auto fn_name = std::get<function_call>(lhs_parse.back()).identifier;
                return func_def_expression{std::move(fn_name), collect_arg_names(lhs_parse), {std::move(def_parse)}};
            }
        [[fallthrough]]; // in the case that it is not assignment or function def, we try to parse as a boolean expression
        case token_kind::not_equal:
        case token_kind::less_than:
        case token_kind::less_or_equal:
        case token_kind::greater_than:
        case token_kind::greater_or_equal:
            {
                std::vector<operation> rhs_parse;
                parse_arithmetic(rhs_parse);
                expect_end();
                return boolean_expression{{std::move(lhs_parse)}, {std::move(rhs_parse)}, delimiter.kind()};
            }
        default:
            unexpected_token(delimiter);
            return arithmetic_expression{lhs_parse};
    }
}

// The parsing functions share a single vector around, placing elements onto the parse stack as necessary.
void parser::parse_arithmetic(std::vector<operation>& parsing, int enclosing_precedence, bool enclosing_right_assoc)
{
    const int unary_prec = unary_precendence(_current.kind());
    if (unary_prec == -1) // not unary operator
    {
        // Must parse some sort of primary term (a function call, a parenthesized expression, etc.)
        parse_primary_term(parsing);
    }
    else
    {
        // We have a unary operator
        const auto unary_op = forward().kind();
        // read unary operand
        parse_arithmetic(parsing, unary_prec);
        parsing.emplace_back(unary_operator{unary_op}); // And place the unary operator on the stack
    }

    while (true)
    {
        const auto prec = binary_precedence(_current.kind());
        token_kind op_kind;
        if (prec != -1) // is binary operator
        {
            if (enclosing_right_assoc ? prec < enclosing_precedence : prec <= enclosing_precedence)
                return;

            op_kind = forward().kind();
        }
        else if (can_insert_implicit_multiply(_current.kind()))
        {
            // insert implicit multiply
            op_kind = token_kind::multiply;
        }
        else
        {
            return; // if not a kind that starts a term, or binary operator, stop.
        }
        // parse right-hand side, up to where the operator precedence will allow us
        parse_arithmetic(parsing, prec, is_right_associative(op_kind));
        parsing.emplace_back(binary_operator{op_kind}); // and put operator
    }
}

void parser::parse_primary_term(std::vector<operation>& parsing)
{
    switch(_current.kind())
    {
        case token_kind::identifier:
            if (peek().kind() != token_kind::open_parenthesis)
                parsing.emplace_back(variable_reference{std::string{forward().source_str()}});
            else
                parse_function(parsing);
            return;
        case token_kind::numeric_literal:
            {
                auto num = number{64}; // TODO: not magic constant
                if (_current.source_str().back() == 'i')
                {
                    if (_current.source_str().length() == 1)
                        num.set(0, 1); // string is "i", set to 0 + 1i
                    else
                        num.set_imaginary(_current.source_str());
                }
                else
                {
                    num.set_real(_current.source_str());
                }
                forward();
                parsing.emplace_back(literal_number{std::move(num)});
                return;
            }
        case token_kind::open_parenthesis:
            forward();
            parse_arithmetic(parsing);
            // like a lot of calculators, silenty ignore missing close parens
            if (_current.kind() == token_kind::close_parenthesis)
                forward();
            return;

        default:
            unexpected_token(forward());
            return;
    }
}

void parser::parse_function(std::vector<operation>& parsing)
{
    std::string fn_name{forward().source_str()};
    if (forward().kind() == token_kind::close_parenthesis)
    {
        parsing.emplace_back(function_call{std::move(fn_name), 0});
        return;
    }
    int arity = 0;
    while (true)
    {
        parse_arithmetic(parsing);
        arity++;

        if (_current.kind() == token_kind::argument_separator)
            forward();
        else
            break;
    }

    if (_current.kind() == token_kind::close_parenthesis)
        forward();

    parsing.emplace_back(function_call{std::move(fn_name), arity});
}

void parser::unexpected_token(const token& err_token)
{
    _diagnostic_bag.emplace_back(err_token.source(), diagnostic_type::unexpected_token, token_kind_name(err_token.kind()));
}