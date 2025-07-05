use crate::{token::TokenKind, SourcePos};

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
    pub position: SourcePos
}

#[derive(Debug, Clone)]
pub enum Expr {
    Arithmetic(Vec<Operation>),
    Assignment { var: String, comp: Vec<Operation> },
    Boolean { lhs: Vec<Operation>, rhs: Vec<Operation>, kind: TokenKind }
}

#[derive(Debug, Clone)]
pub struct Expression {
    pub expr: Expr,
    pub position: SourcePos,
}