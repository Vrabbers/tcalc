#include "token.h"

Token::Token(Type type, SourceSpan source) : _source(source), _type(type) { }

Token::Type Token::type() const
{
    return _type;
}

SourceSpan Token::source() const
{
    return _source;
}
