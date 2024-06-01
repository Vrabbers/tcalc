#ifndef TC_PARSER_H
#define TC_PARSER_H

#include "tc_expression.h"
#include "tc_lexer.h"

namespace tcalc
{
    class parser final
    {
    public:
        explicit parser(lexer&& lexer, const long number_precision) :
            _current{lexer.next()},
            _lexer{std::move(lexer)},
            _diagnostic_bag{_lexer.diagnostic_bag()},
            _number_precision{number_precision}
        {
        }

        std::vector<expression> parse_all();
        expression parse_expression();

        [[nodiscard]]
        bool at_end() const
        {
            return _current.kind() == token_kind::end_of_file;
        }

        [[nodiscard]]
        const std::vector<diagnostic>& diagnostic_bag() const
        {
            return _diagnostic_bag;
        }

    private:
        token forward()
        {
            auto old_current = _current;
            if (_peek.has_value())
                _current = *_peek;
            else
                _current = _lexer.next();
            _peek = std::nullopt;
            return old_current;
        }

        const token& peek()
        {
            if (!_peek.has_value())
                _peek = _lexer.next();
            return *_peek;
        }

        void parse_arithmetic(std::vector<operation> &parsing, int enclosing_precedence = -1, bool enclosing_right_assoc = false);
        void parse_function(std::vector<operation>& parsing);
        void unexpected_token(const token& err_token);
        void parse_primary_term(std::vector<operation> &parsing);
        void expect_end();
        void parse_super_num(std::vector<operation>& parsing);
        void parse_super_term(std::vector<operation>& parsing);
        void parse_superscript(std::vector<operation>& parsing);
        expression parse_variable_assignment(size_t lhs_start_ix, std::vector<operation>&& lhs_parse);
        expression parse_boolean_expression(size_t lhs_start_ix, size_t lhs_end_ix, std::vector<operation>&& lhs_parse,
                                            token_kind delimiter);
        token _current;
        std::optional<token> _peek;
        lexer _lexer;
        std::vector<diagnostic>& _diagnostic_bag;
        long _number_precision;
    };

    template<class ExprT> requires std::convertible_to<ExprT, expression>
    std::pair<parser, std::optional<ExprT> > parse_single(lexer&& lexer, const long number_precision)
    {
        parser p{std::move(lexer), number_precision};
        const expression expr = p.parse_expression();

        if (const ExprT* t_exp = std::get_if<ExprT>(&expr))
            return {p, *t_exp};

        return {p, std::nullopt};
    }
}

#endif // TC_PARSER_H
