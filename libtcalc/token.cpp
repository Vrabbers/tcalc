#include "token.h"

tcToken::tcToken(): _type(tcTokenType::Bad)
{
}

tcToken::tcToken(const tcTokenType type, tcSourceSpan&& source) : _type(type),
                                                                  _source(std::make_unique<tcSourceSpan>(source))
{
}

tcTokenType tcToken::type() const
{
    return _type;
}

const tcSourceSpan& tcToken::source() const
{
    return *_source;
}

tcToken::~tcToken() = default;

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
        "Tau",
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
