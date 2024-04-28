#pragma once
#include <span>

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
                _peek = std::move(_lexer.next());
            return *_peek;
        }

        token _current;
        std::optional<token> _peek;
        lexer _lexer;
        std::vector<diagnostic>& _diagnostic_bag;
    };
}
