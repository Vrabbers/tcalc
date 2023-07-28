#include "token.h"

Token::Token(Type type, SourceSpan source) : _type(type), _source(source)
{
}

Token::Type Token::type() const
{
    return _type;
}

SourceSpan Token::source() const
{
    return _source;
}

const char* Token::typeName(Token::Type type)
{
    static const char* typeNames[] = 
    {
        "Bad",
        "EndOfFile",

        "NumericLiteral",
        "SuperscriptLiteral",
        "HexLiteral",
        "BinaryLiteral",
        "Identifier",

        "Add",
        "SuperscriptAdd",
        "Subtract",
        "SuperscriptSubtract",
        "Multiply",
        "Divide",
        "Exponentiate",
        "LeftParens",
        "RightParens",
        "Radical",
        "Percent",
        "Factorial",
        "LeftShift",
        "RightShift",
        "Greater",
        "GreaterOrEqual",
        "Less",
        "LessOrEqual",
        "Equal",
        "Equality",
        "NotEqual",
        "Pi",
        "Nand",
        "Nor",
        "Xnor",
        "And",
        "Or",
        "Xor",
        "Not",

        "ArgumentSeparator",
        "EndOfLine"
    };

    return typeNames[static_cast<std::size_t>(type)];
}
