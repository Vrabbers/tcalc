use std::mem;

use crate::SourcePos;
use crate::token::{Token, TokenKind};
use crate::lexer::Lexer;
use crate::expressions::{Expr, Expression, Op, Operation};

#[derive(Debug, Clone)]
pub struct Parser {
    current: Token,
    peek: Option<Token>,
    lexer: Lexer,
}

fn binary_precedence(kind: TokenKind) -> i32 {
    match kind {
        TokenKind::Exponentiate => 5,

        TokenKind::RightShift
        | TokenKind::LeftShift
        | TokenKind::BinaryAnd
        | TokenKind::BinaryNand
        | TokenKind::BinaryOr
        | TokenKind::BinaryNor
        | TokenKind::BinaryXor
        | TokenKind::BinaryXnor => 3,

        TokenKind::Multiply | TokenKind::Divide => 2,

        TokenKind::Minus | TokenKind::Plus => 1,

        _ => -1,
    }
}

fn unary_precedence(kind: TokenKind) -> i32 {
    match kind {
        TokenKind::BinaryNot
        | TokenKind::Minus
        | TokenKind::Plus
        | TokenKind::Radical
        | TokenKind::CubeRoot
        | TokenKind::FourthRoot
        | TokenKind::SuperscriptLiteral => 4,

        _ => -1,
    }
}

fn is_right_assoc(kind: TokenKind) -> bool {
    kind == TokenKind::Exponentiate
}

fn ends_expr(kind: TokenKind) -> bool {
    matches!(kind, TokenKind::ExpressionSeparator | TokenKind::EndOfFile)
}

fn can_insert_implicit_multiply(kind: TokenKind) -> bool {
    match kind {
        TokenKind::Identifier | TokenKind::OpenParenthesis | TokenKind::NumericLiteral => true,
        TokenKind::SuperscriptLiteral => false,
        _ => unary_precedence(kind) != -1,
    }
}

fn is_postfix_operator(kind: TokenKind) -> bool {
    matches!(kind, TokenKind::Rad
        | TokenKind::Deg
        | TokenKind::Grad
        | TokenKind::Percent
        | TokenKind::Factorial)
}

fn is_superscript(kind: TokenKind) -> bool {
    matches!(kind, TokenKind::SuperscriptLiteral
        | TokenKind::SuperscriptMinus
        | TokenKind::SuperscriptPlus)
}

impl Parser {
    pub fn parse_all(&mut self) -> Vec<Expression> {
        let mut expr = Vec::new();
        loop {
            expr.push(self.parse_expression());

            if self.at_end() {
                break;
            }
        }
        expr
    }

    fn parse_expression(&mut self) -> Expression {
        let lhs_start = self.current.position.start;
        let mut lhs_parse = Vec::new();
        self.parse_arithmetic(&mut lhs_parse);
        let lhs_end = self.current.position.start;

        if lhs_parse.is_empty() {
            self.expect_end();
            return Expression {
                expr: Expr::Arithmetic(lhs_parse),
                position: SourcePos::new(lhs_start, lhs_end),
            };
        }

        match self.current.kind {
            TokenKind::Equal
                if lhs_parse.len() == 1 && matches!(lhs_parse[0].op, Op::VarRef(_)) =>
                self.parse_variable_assignment(lhs_start, lhs_parse),

            TokenKind::Equal
            | TokenKind::Equality
            | TokenKind::NotEqual
            | TokenKind::LessThan
            | TokenKind::LessOrEqual
            | TokenKind::GreaterThan
            | TokenKind::GreaterOrEqual => {
                let boolean_kind = self.forward().kind;
                self.parse_boolean_expression(lhs_start, lhs_end, lhs_parse, boolean_kind)
            },

            _ => todo!()
        }
    }

    pub fn at_end(&self) -> bool {
        self.current.kind == TokenKind::EndOfFile
    }

    fn parse_arithmetic(&mut self, lhs_parse: &mut Vec<Operation>) {
        todo!()
    }

    fn expect_end(&mut self) {
        todo!()
    }
    
    fn parse_variable_assignment(&mut self, lhs_start: usize, lhs_parse: Vec<Operation>) -> Expression {
        todo!()
    }
    
    fn forward(&mut self) -> Token {
        let mut val  = self.peek.take().unwrap_or_else(|| self.lexer.next_token());
        mem::swap(&mut self.current, &mut val);
        val
    }

    fn peek(&mut self) -> &Token {
        if self.peek.is_none() {
            self.peek = Some(self.lexer.next_token())
        }
        self.peek.as_ref().unwrap()
    }
    
    fn parse_boolean_expression(&mut self, lhs_start: usize, lhs_end: usize, lhs_parse: Vec<Operation>, kind: TokenKind) -> Expression {
        todo!()
    }
}
