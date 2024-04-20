#pragma once

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
    Tau,
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

const char* tcTokenTypeName(tcTokenType);