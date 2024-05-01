#include "parser.h"

using namespace tcalc;

static bool ends_expr(const token_kind kind)
{
    switch (kind)
    {
        case token_kind::expression_separator:
        case token_kind::end_of_file:
        case token_kind::bad:
            return true;
        default:
            return false;
    }
}

static bool ends_arith(const token_kind kind)
{
    switch (kind)
    {
        case token_kind::equal:
        case token_kind::less:
        case token_kind::less_or_equal:
        case token_kind::greater:
        case token_kind::greater_or_equal:
        case token_kind::not_equal:
        case token_kind::expression_separator:
        case token_kind::end_of_file:
        case token_kind::bad:
            return true;
        default:
            return false;
    }
}

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

expression parser::parse_expression()
{
    std::vector<operation> parse;
    parse_arithmetic(parse);
    return arithmetic_expression{parse};
}

// The parsing functions share a single vector around, placing elements onto the parse stack as necessary.
void parser::parse_arithmetic(std::vector<operation>& parsing, int enclosing_precedence, bool enclosing_right_assoc)
{
    int unary_prec = unary_precendence(_current.kind());
    if (unary_prec == -1) // not unary operator
    {
        // Must parse some sort of primary term (a function call, a parenthesized expression, etc.)
        parse_primary_term(parsing);
    }
    else
    {
        // We have a unary operator
        auto unary_op = forward().kind();
        // read unary operand
        parse_arithmetic(parsing, unary_prec);
        parsing.emplace_back(unary_operator{unary_op}); // And place the unary operator on the stack
    }

    while (true)
    {
        auto prec = binary_precedence(_current.kind());
        if (prec == -1)
            return;
        // TODO: Need to check right-associative (i.e. right-associative exponentiate)
        if (prec < enclosing_precedence)
            return;

        auto op_kind = forward().kind();
        // parse right-hand side, up to where the operator precedence will allow us
        parse_arithmetic(parsing, prec, op_kind == token_kind::exponentiate);
        parsing.emplace_back(binary_operator{op_kind}); // and put operator
    }
}

void parser::parse_primary_term(std::vector<operation>& parsing, int enclosing_precedence)
{
    switch(_current.kind())
    {
        case token_kind::identifier:
            parsing.emplace_back(variable_reference{std::string{_current.source_str()}});
            break;
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
            parsing.emplace_back(literal_number{std::move(num)});
            break;
        }
        default: 
            break;
    }
    forward();
}
