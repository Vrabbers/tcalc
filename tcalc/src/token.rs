use std::fmt;

use crate::source_pos::SourcePos;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenKind {
    Bad,
    EndOfFile,

    NumericLiteral,
    SuperscriptLiteral,
    HexLiteral,
    BinaryLiteral,
    Identifier,

    Plus,
    SuperscriptPlus,
    Minus,
    SuperscriptMinus,
    Multiply,
    Divide,
    Exponentiate,
    OpenParenthesis,
    CloseParenthesis,
    Radical,
    CubeRoot,
    FourthRoot,
    Percent,
    Factorial,
    LeftShift,
    RightShift,
    GreaterThan,
    GreaterOrEqual,
    LessThan,
    LessOrEqual,
    Equal,
    Equality,
    NotEqual,
    BinaryNand,
    BinaryNor,
    BinaryXnor,
    BinaryAnd,
    BinaryOr,
    BinaryXor,
    BinaryNot,

    Deg,
    Rad,
    Grad,

    ArgumentSeparator,
    ExpressionSeparator,
}

impl fmt::Display for TokenKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

#[derive(Debug, Clone)]
pub struct Token {
    pub source: String,
    pub position: SourcePos,
    pub kind: TokenKind,
}
