#include "token.h"

tcToken::tcToken(tcTokenType type, tcSourceSpan source) : _type(type), _source(source)
{
}

tcTokenType tcToken::type() const
{
    return _type;
}

tcSourceSpan tcToken::source() const
{
    return _source;
}

const char* tcTokenTypeName(tcTokenType type)
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
