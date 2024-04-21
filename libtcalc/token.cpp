#include "token.h"

#include <array>
#include <iomanip>
#include <sstream>

const char* tcTokenKindName(tcTokenKind type)
{
    static std::array typeNames =
    {
        "Bad",
        "EndOfFile",

        "NumericLiteral",
        "SuperscriptLiteral",
        "HexLiteral",
        "BinaryLiteral",
        "Identifier",

        "Plus",
        "SuperscriptPlus",
        "Minus",
        "SuperscriptMinus",
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

    return typeNames.at(static_cast<std::size_t>(type));
}

std::string tcToken::format() const
{
    std::stringstream strStr;
    strStr << "(L:" << position().line << ", C:" << position().column << "): ";
    strStr << tcTokenKindName(kind()) << " ";
    if (kind() != tcTokenKind::EndOfLine && kind() != tcTokenKind::EndOfFile)
        strStr << std::quoted(sourceStr());
    return std::move(strStr.str());
}
