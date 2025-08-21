use crate::{source_pos::SourcePos, token::TokenKind};

type Number = String;

#[derive(Debug, Clone)]
pub enum OperationType {
    Binary(TokenKind),
    Unary(TokenKind),
    Literal(Number),
    VarRef(String),
    FnCall { name: String, arity: i32 },
}

#[derive(Debug, Clone)]
pub struct Operation {
    pub op_type: OperationType,
    pub position: SourcePos,
}

#[derive(Debug, Clone)]
pub struct Expression {
    pub operations: Vec<Operation>,
    pub position: SourcePos,
}

#[derive(Debug, Clone)]
pub enum Statement {
    Arithmetic(Expression),
    Assignment {
        var: String,
        comp: Expression,
        position: SourcePos,
    },
    Boolean {
        lhs: Expression,
        rhs: Expression,
        kind: TokenKind,
        position: SourcePos,
    },
}
