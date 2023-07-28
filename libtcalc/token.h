#pragma once

#include "source_span.h"
#include "tcalc_export.h"

class TCALC_EXPORT Token final
{
    public:
        enum class Type
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

        explicit Token(Type type, SourceSpan source);
        Type type() const;
        SourceSpan source() const;
        static const char* typeName(Token::Type);
    private:
        Type _type;
        SourceSpan _source;
};
