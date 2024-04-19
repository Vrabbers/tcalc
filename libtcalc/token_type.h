#pragma once
#include "tcalc_export.h"

enum class tcTokenType
{
    Bad,
    EndOfFile,

    NumericLiteral,
    SuperscriptLiteral,
    HexLiteral,
    BinaryLiteral,
    Identifier,

    Add,
    SuperscriptAdd,
    Subtract,
    SuperscriptSubtract,
    Multiply,
    Divide,
    Exponentiate,
    LeftParens,
    RightParens,
    Radical,
    Percent,
    Factorial,
    LeftShift,
    RightShift,
    Greater,
    GreaterOrEqual,
    Less,
    LessOrEqual,
    Equal,
    Equality,
    NotEqual,
    Pi,
    Nand,
    Nor,
    Xnor,
    And,
    Or,
    Xor,
    Not,

    ArgumentSeparator,
    EndOfLine
};

TCALC_EXPORT const char* tcTokenTypeName(tcTokenType);