#pragma once

#include "expression.h"
#include "lexer.h"

namespace tcalc
{
    class parser final
    {
    public:
        explicit parser(lexer&& lexer) : _current{lexer.next()}, _lexer{std::move(lexer)}, _diagnostic_bag{_lexer.diagnostic_bag()}
        {
        }

        expression parse_expression();

        const std::vector<diagnostic>& diagnostic_bag()
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
        void parse_primary_term(std::vector<operation> &parsing, int enclosing_precedence = -1);

        token _current;
        std::optional<token> _peek;
        lexer _lexer;
        std::vector<diagnostic>& _diagnostic_bag;
    };
}
