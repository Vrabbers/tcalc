#include "lexer.h"

Lexer::Lexer(std::string&& input, bool argSeparatorIsComma)
    : _sr(std::move(input)), _argSeparatorIsComma(argSeparatorIsComma)
{
}

Token Lexer::next()
{
    return Token(Token::Type::Bad, SourceSpan(std::string_view(), SourcePosition(0, 0)));
}
