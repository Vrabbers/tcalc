use crate::{source_pos::SourcePos, token::TokenKind};

type Number = String;

#[derive(Debug, Clone)]
pub enum Op {
    Binary(TokenKind),
    Unary(TokenKind),
    Literal(Number),
    VarRef(String),
    FnCall { name: String, arity: i32 },
}

#[derive(Debug, Clone)]
pub struct Operation {
    pub op: Op,
    pub position: SourcePos,
}

#[derive(Debug, Clone)]
pub struct Computation {
    pub ops: Vec<Operation>,
    pub position: SourcePos,
}

#[derive(Debug, Clone)]
pub enum Expression {
    Arithmetic(Computation),
    Assignment {
        var: String,
        comp: Computation,
        position: SourcePos,
    },
    Boolean {
        lhs: Computation,
        rhs: Computation,
        kind: TokenKind,
        position: SourcePos,
    },
}
