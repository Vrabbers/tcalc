#include "tc_parser.h"

#include <cassert>
#include <stdexcept>

#include "utf8utils.h"

using namespace tcalc;

static int binary_precedence(const token_kind kind)
{
    switch (kind)
    {
        case token_kind::exponentiate:
            return 5;

        case token_kind::right_shift:
        case token_kind::left_shift:
        case token_kind::binary_and:
        case token_kind::binary_nand:
        case token_kind::binary_or:
        case token_kind::binary_nor:
        case token_kind::binary_xor:
        case token_kind::binary_xnor:
            return 3;

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
            return 4;
        default:
            return -1;
    }
}

static bool is_right_associative(const token_kind kind)
{
    return kind == token_kind::exponentiate;
}

static bool ends_expr(const token_kind kind)
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
        case token_kind::numeric_literal:
            return true;
        default:
            return unary_precendence(kind) != -1;
    }
}

static bool is_postfix_operator(const token_kind kind)
{
    return kind == token_kind::percent || kind == token_kind::factorial;
}

static bool is_superscript(const token_kind kind)
{
    switch (kind)
    {
        case token_kind::superscript_literal:
        case token_kind::superscript_minus:
        case token_kind::superscript_plus:
            return true;
        default:
            return false;
    }
}

std::vector<expression> parser::parse_all()
{
    std::vector<expression> exprs;
    do
    {
        exprs.push_back(parse_expression());
    } while(_current.kind() != token_kind::end_of_file);
    return exprs;
}

expression parser::parse_variable_assignment(const size_t lhs_start_ix, std::vector<operation>&& lhs_parse)
{
    const size_t rhs_start_ix = _current.start_index();
    // this is a variable assignment expression 😁
    std::vector<operation> rhs_parse;
    parse_arithmetic(rhs_parse);
    const size_t rhs_end_ix = _current.end_index();
    expect_end();
    auto var = std::get<variable_reference>(lhs_parse[0]).identifier;
    return assignment_expression{
        .variable = std::move(var),
        .expression = arithmetic_expression{
            .tokens = std::move(rhs_parse),
            .position = {rhs_start_ix, rhs_end_ix}
        },
        .position = {lhs_start_ix, rhs_end_ix}
    };
}

expression parser::parse_function_definition(const size_t lhs_start_ix, std::vector<operation>&& lhs_parse)
{
    const size_t rhs_start_ix = _current.start_index();
    std::vector<operation> def_parse;
    parse_arithmetic(def_parse);
    const size_t rhs_end_ix = _current.end_index();
    expect_end();
    auto fn_name = std::get<function_call>(lhs_parse.back()).identifier;
    return func_def_expression{
        .name = std::move(fn_name),
        .parameters = collect_arg_names(lhs_parse),
        .expression = arithmetic_expression{
            .tokens = std::move(def_parse),
            .position = {rhs_start_ix, rhs_end_ix}
        },
        .position = {lhs_start_ix, rhs_end_ix}
    };
}

expression parser::parse_boolean_expression(const size_t lhs_start_ix, const size_t lhs_end_ix,
                                            std::vector<operation>&& lhs_parse, const token_kind delimiter)
{
    const size_t rhs_start_ix = _current.start_index();
    std::vector<operation> rhs_parse;
    parse_arithmetic(rhs_parse);
    const size_t rhs_end_ix = _current.end_index();
    expect_end();
    return boolean_expression{
        .lhs = arithmetic_expression{
            .tokens = std::move(lhs_parse),
            .position = {lhs_start_ix, lhs_end_ix}
        },
        .rhs = arithmetic_expression{
            .tokens = std::move(rhs_parse),
            .position = {rhs_start_ix, rhs_end_ix}
        },
        .kind = delimiter,
        .position = {lhs_start_ix, rhs_end_ix}
    };
}

expression parser::parse_expression()
{
    const size_t lhs_start_ix = _current.start_index();

    std::vector<operation> lhs_parse;
    parse_arithmetic(lhs_parse); // Parse full expression or left hand side of function call or assignment
    const size_t lhs_end_ix = _current.start_index();

    if (lhs_parse.empty())
    {
        expect_end();
        return arithmetic_expression{{}, {lhs_start_ix, lhs_end_ix}};
    }

    switch (_current.kind())
    {
        case token_kind::equal:
            // can be assignment, or function def, or boolean expr
            if (lhs_parse.size() == 1 && std::holds_alternative<variable_reference>(lhs_parse[0]))
                return parse_variable_assignment(lhs_start_ix, std::move(lhs_parse));
            if (is_valid_func_def(lhs_parse))
                return parse_function_definition(lhs_start_ix, std::move(lhs_parse));
        [[fallthrough]]; // in the case that it is not assignment or function def, we try to parse as a boolean expression
        case token_kind::not_equal:
        case token_kind::less_than:
        case token_kind::less_or_equal:
        case token_kind::greater_than:
        case token_kind::greater_or_equal:
            return parse_boolean_expression(lhs_start_ix, lhs_end_ix, std::move(lhs_parse), forward().kind());
        default:
            expect_end();
            return arithmetic_expression{std::move(lhs_parse), {lhs_start_ix, lhs_end_ix}};
    }
}

// The parsing functions share a single vector around, placing elements onto the parse stack as necessary.
void parser::parse_arithmetic(std::vector<operation>& parsing, const int enclosing_precedence, const bool enclosing_right_assoc)
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
        const auto unary_op = forward();
        // read unary operand
        parse_arithmetic(parsing, unary_prec);
        parsing.emplace_back(unary_operator{unary_op.kind(), unary_op.position()}); // And place the unary operator on the stack
    }

    while (true)
    {
        auto prec = binary_precedence(_current.kind());
        token_kind op_kind;
        source_position position;

        if (prec != -1) // is binary operator
        {
            op_kind = _current.kind();
            position = _current.position();

            if (enclosing_right_assoc ? prec < enclosing_precedence : prec <= enclosing_precedence)
                return;

            forward();
        }
        else if (can_insert_implicit_multiply(_current.kind()))
        {
            // insert implicit multiply
            op_kind = token_kind::multiply;
            prec = binary_precedence(token_kind::multiply);
            position = {_current.position().start_index, _current.position().start_index};

            if (enclosing_right_assoc ? prec < enclosing_precedence : prec <= enclosing_precedence)
                return;
        }
        else if (is_superscript(_current.kind()))
        {
            auto exp_pos = _current.position().start_index;
            parse_superscript(parsing);
            parsing.emplace_back(binary_operator{token_kind::exponentiate, {exp_pos, exp_pos}});
            continue;
        }
        else if (is_postfix_operator(_current.kind()))
        {
            parsing.emplace_back(unary_operator{_current.kind(), forward().position()});
            continue;
        }
        else
        {
            return; // if not a kind that starts a term, or binary operator, stop.
        }

        // parse right-hand side, up to where the operator precedence will allow us
        parse_arithmetic(parsing, prec, is_right_associative(op_kind));
        parsing.emplace_back(binary_operator{op_kind, position}); // and put operator
    }
}

void parser::parse_primary_term(std::vector<operation>& parsing)
{
    switch(_current.kind())
    {
        case token_kind::identifier:
            if (peek().kind() != token_kind::open_parenthesis)
            {
                parsing.emplace_back(variable_reference{std::string{_current.source()}, _current.position()});
                forward();
            }
            else
            {
                parse_function(parsing);
            }
            return;
        case token_kind::numeric_literal:
            {
                auto num = number{_number_precision};
                if (_current.source().back() == 'i')
                {
                    if (_current.source().length() == 1)
                        num.set(0, 1); // string is "i", set to 0 + 1i
                    else
                        num.set_imaginary(_current.source());
                }
                else
                {
                    num.set_real(_current.source());
                }
                const auto token = forward();
                parsing.emplace_back(literal_number{std::move(num), token.position()});
                return;
            }
        case token_kind::binary_literal:
            {
                auto num = number{_number_precision};
                num.set_binary(_current.source());
                parsing.emplace_back(literal_number{std::move(num), _current.position()});
                forward();
                return;
            }
        case token_kind::hex_literal:
            {
                auto num = number{_number_precision};
                num.set_hexadecimal(_current.source());
                parsing.emplace_back(literal_number{std::move(num), _current.position()});
                forward();
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

void parser::expect_end()
{
    if (!ends_expr(_current.kind()))
        unexpected_token(_current);
    forward();
}

static std::optional<token_kind> from_superscript_op(const token_kind kind)
{
    switch (kind)
    {
        case token_kind::superscript_minus:
            return token_kind::minus;
        case token_kind::superscript_plus:
            return token_kind::plus;
        default:
            return std::nullopt;
    }
}

void parser::parse_super_num(std::vector<operation>& parsing)
{
    if (_current.kind() != token_kind::superscript_literal)
    {
        unexpected_token(_current);
        return;
    }

    const auto num_str = utf8utils::to_inline_number(_current.source());
    number num{_number_precision};
    if (num_str.back() == 'i')
    {
        if (num_str.length() == 1)
            num.set(0, 1); // string is "i", set to 0 + 1i
        else
            num.set_imaginary(num_str);
    }
    else
    {
        num.set_real(num_str);
    }
    parsing.emplace_back(literal_number{std::move(num), _current.position()});
    forward();
}

void parser::parse_super_term(std::vector<operation>& parsing)
{
    if (_current.kind() != token_kind::superscript_literal)
    {
        const auto unary_op_token = forward();
        if (const auto unary = from_superscript_op(unary_op_token.kind()))
        {
            parse_super_num(parsing);
            parsing.emplace_back(unary_operator{*unary, unary_op_token.position()});
        }
        else
        {
            unexpected_token(unary_op_token);
        }
    }
    else
    {
        parse_super_num(parsing);
    }
}

void parser::parse_superscript(std::vector<operation>& parsing)
{
    parse_super_term(parsing);

    while (true)
    {
        if (const auto binary_op = from_superscript_op(_current.kind()))
        {
            binary_operator bin_op{*binary_op, _current.position()};
            forward();
            parse_super_term(parsing);
            parsing.emplace_back(bin_op);
        }
        else
        {
            break;
        }
    }
}

void parser::parse_function(std::vector<operation>& parsing)
{
    const auto name_token = forward(); // Consume name token
    const auto position = name_token.position();
    std::string fn_name{name_token.source()};

    forward(); // Consume open parens

    if (_current.kind() == token_kind::close_parenthesis)
    {
        forward();
        parsing.emplace_back(function_call{std::move(fn_name), 0, position});
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

    parsing.emplace_back(function_call{std::move(fn_name), arity, position});
}

void parser::unexpected_token(const token& err_token)
{
    _diagnostic_bag.emplace_back(err_token.position(), diagnostic_type::unexpected_token, token_kind_name(err_token.kind()));
}