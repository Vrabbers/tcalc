#include "parser.h"

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

expression parser::parse_expression()
{
    std::vector<operation> lhs_parse;
    parse_arithmetic(lhs_parse); // Parse full expression or left hand side of function call or assignment
    const auto token = forward();
    switch (token.kind())
    {
        case token_kind::expression_separator:
        case token_kind::end_of_file:
            return arithmetic_expression{lhs_parse};
        default:
            unexpected_token(token);
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
        auto prec = binary_precedence(_current.kind());
        
        // If not a binary operator
        if (prec == -1)
            return;

        if (enclosing_right_assoc ? prec < enclosing_precedence : prec <= enclosing_precedence)
            return;

        const auto op_kind = forward().kind();
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
            if (peek().kind() != token_kind::left_parenthesis)
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
        case token_kind::left_parenthesis:
            forward();
            parse_arithmetic(parsing);
            // like a lot of calculators, silenty ignore missing close parens
            if (_current.kind() == token_kind::right_parenthesis)
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
    if (forward().kind() == token_kind::right_parenthesis)
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

    if (_current.kind() == token_kind::right_parenthesis)
        forward();

    parsing.emplace_back(function_call{std::move(fn_name), arity});
}

void parser::unexpected_token(const token& err_token)
{
    _diagnostic_bag.emplace_back(err_token.source(), diagnostic_type::unexpected_token, token_kind_name(err_token.kind()));
}