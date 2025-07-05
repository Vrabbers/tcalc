use std::mem;

use crate::{
    SourcePos,
    diagnostics::{Diagnostic, DiagnosticType},
    expressions::{Computation, Expression, Op, Operation},
    lexer::Lexer,
    token::{Token, TokenKind},
};

#[derive(Debug, Clone)]
pub struct Parser {
    current: Token,
    lexer: Lexer,
}

const fn binary_precedence(kind: TokenKind) -> i32 {
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

const fn unary_precedence(kind: TokenKind) -> i32 {
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
    matches!(
        kind,
        TokenKind::Rad
            | TokenKind::Deg
            | TokenKind::Grad
            | TokenKind::Percent
            | TokenKind::Factorial
    )
}

fn is_superscript(kind: TokenKind) -> bool {
    matches!(
        kind,
        TokenKind::SuperscriptLiteral | TokenKind::SuperscriptMinus | TokenKind::SuperscriptPlus
    )
}

fn enclosing_precedence_test(prec: i32, enclosing_prec: i32, enclosing_right_assoc: bool) -> bool {
    (enclosing_right_assoc && prec < enclosing_prec)
        || (!enclosing_right_assoc && prec <= enclosing_prec)
}

fn to_decimal(superscript: char) -> char {
    match superscript {
        '⁻' => '-',
        '⁺' => '+',
        '¹' => '1',
        '²' => '2',
        '³' => '3',
        '⁴' => '4',
        '⁵' => '5',
        '⁶' => '6',
        '⁷' => '7',
        '⁸' => '8',
        '⁹' => '9',
        '⁰' => '0',
        'ᵉ' | 'ᴱ' => 'e',
        'ⁱ' => 'i',
        _ => panic!("invalid super character"),
    }
}

fn from_superscript_op(kind: TokenKind) -> Option<TokenKind> {
    match kind {
        TokenKind::SuperscriptPlus => Some(TokenKind::Plus),
        TokenKind::SuperscriptMinus => Some(TokenKind::Minus),
        _ => None,
    }
}

impl Iterator for Parser {
    type Item = Expression;

    fn next(&mut self) -> Option<Self::Item> {
        if self.at_end() {
            None
        } else {
            Some(self.parse_expression())
        }
    }
}

impl Parser {
    pub fn new(mut lexer: Lexer) -> Self {
        Self {
            current: lexer.next_token(),
            lexer,
        }
    }

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
            return Expression::Arithmetic(Computation {
                ops: lhs_parse,
                position: SourcePos::new(lhs_start, lhs_end),
            });
        }

        if self.current.kind == TokenKind::Equal && lhs_parse.len() == 1 {
            if let Op::VarRef(var) = lhs_parse.pop().unwrap().op {
                return self.parse_variable_assignment(lhs_start, var);
            }
        }

        match self.current.kind {
            TokenKind::Equal
            | TokenKind::Equality
            | TokenKind::NotEqual
            | TokenKind::LessThan
            | TokenKind::LessOrEqual
            | TokenKind::GreaterThan
            | TokenKind::GreaterOrEqual => {
                let boolean_kind = self.forward().kind;
                self.parse_boolean_expression(lhs_start, lhs_end, lhs_parse, boolean_kind)
            }

            _ => {
                self.expect_end();
                Expression::Arithmetic(Computation {
                    ops: lhs_parse,
                    position: SourcePos::new(lhs_start, lhs_end),
                })
            }
        }
    }

    pub fn at_end(&self) -> bool {
        self.current.kind == TokenKind::EndOfFile
    }

    fn parse_arithmetic(&mut self, parse: &mut Vec<Operation>) {
        self.parse_arithmetic_prec_assoc(parse, -1, false);
    }

    fn parse_arithmetic_prec(&mut self, parse: &mut Vec<Operation>, enclosing_prec: i32) {
        self.parse_arithmetic_prec_assoc(parse, enclosing_prec, false);
    }

    fn parse_arithmetic_prec_assoc(
        &mut self,
        parse: &mut Vec<Operation>,
        enclosing_prec: i32,
        enclosing_right_assoc: bool,
    ) {
        let unary_prec = unary_precedence(self.current.kind);
        if unary_prec == -1 {
            self.parse_primary_term(parse);
        } else {
            let unary_op = self.forward();
            if unary_op.kind == TokenKind::SuperscriptLiteral {
                // Expect to read nth root expression of form ⁿ√x

                if self.current.kind != TokenKind::Radical {
                    self.unexpected_token(unary_op.position, unary_op.kind);
                    return;
                }

                let radical = self.forward();
                let num_str = unary_op.source.chars().map(to_decimal).collect::<String>();
                if num_str.find('i').is_some() {
                    self.diagnostic_bag_mut().push(Diagnostic::new(
                        unary_op.position,
                        DiagnosticType::InvalidNumberLiteral,
                    ));
                    return;
                }
                parse.push(Operation {
                    op: Op::Literal(num_str),
                    position: unary_op.position,
                });
                self.parse_arithmetic_prec(parse, unary_prec);
                let position = SourcePos::new(unary_op.position.start, radical.position.end);
                parse.push(Operation {
                    op: Op::Binary(TokenKind::Radical),
                    position,
                });
            } else {
                self.parse_arithmetic_prec(parse, unary_prec);
                parse.push(Operation {
                    op: Op::Unary(unary_op.kind),
                    position: unary_op.position,
                });
            }
        }

        loop {
            let mut prec = binary_precedence(self.current.kind);
            let (op_kind, position) = if prec != -1 {
                if enclosing_precedence_test(prec, enclosing_prec, enclosing_right_assoc) {
                    return;
                }
                let t = self.forward();
                (t.kind, t.position)
            } else if can_insert_implicit_multiply(self.current.kind) {
                prec = binary_precedence(TokenKind::Multiply);
                let pos = SourcePos::new(self.current.position.start, self.current.position.start);

                if enclosing_precedence_test(prec, enclosing_prec, enclosing_right_assoc) {
                    return;
                }

                (TokenKind::Multiply, pos)
            } else if is_superscript(self.current.kind) {
                let exp_pos = self.current.position;
                self.parse_superscript(parse);
                parse.push(Operation {
                    op: Op::Binary(TokenKind::Exponentiate),
                    position: exp_pos,
                });
                continue;
            } else if is_postfix_operator(self.current.kind) {
                parse.push(Operation {
                    op: Op::Unary(self.current.kind),
                    position: self.forward().position,
                });
                continue;
            } else {
                return;
            };

            self.parse_arithmetic_prec_assoc(parse, prec, is_right_assoc(op_kind));
            parse.push(Operation {
                op: Op::Binary(op_kind),
                position,
            });
        }
    }

    fn parse_primary_term(&mut self, parse: &mut Vec<Operation>) {
        let token = self.forward();
        match token.kind {
            TokenKind::Identifier => {
                if self.current.kind != TokenKind::OpenParenthesis {
                    parse.push(Operation {
                        op: Op::VarRef(token.source),
                        position: token.position,
                    });
                } else {
                    self.parse_function(parse, token);
                }
            }
            TokenKind::NumericLiteral | TokenKind::BinaryLiteral | TokenKind::HexLiteral => {
                parse.push(Operation {
                    op: Op::Literal(token.source),
                    position: token.position,
                });
            }
            TokenKind::OpenParenthesis => {
                self.parse_arithmetic(parse);

                // Silently ignore missing closing parenthesis
                if self.current.kind == TokenKind::CloseParenthesis {
                    self.forward();
                }
            }
            _ => {
                self.unexpected_token(token.position, token.kind);
            }
        }
    }

    fn parse_superscript(&mut self, parse: &mut Vec<Operation>) {
        self.parse_super_term(parse);

        while let Some(binary_op_kind) = from_superscript_op(self.current.kind) {
            let bin_op = Operation {
                op: Op::Binary(binary_op_kind),
                position: self.current.position,
            };
            self.forward();
            self.parse_super_term(parse);
            parse.push(bin_op);
        }
    }

    fn parse_super_term(&mut self, parse: &mut Vec<Operation>) {
        if self.current.kind != TokenKind::SuperscriptLiteral {
            let unary_op_token = self.forward();
            if let Some(kind) = from_superscript_op(unary_op_token.kind) {
                self.parse_super_num(parse);
                parse.push(Operation {
                    op: Op::Unary(kind),
                    position: unary_op_token.position,
                });
            } else {
                self.unexpected_token(unary_op_token.position, unary_op_token.kind);
            }
        } else {
            self.parse_super_num(parse);
        }
    }

    fn parse_super_num(&mut self, parse: &mut Vec<Operation>) {
        let token = self.forward();
        if token.kind != TokenKind::SuperscriptLiteral {
            self.unexpected_token(token.position, token.kind);
            return;
        }

        let num_str = token.source.chars().map(to_decimal).collect::<String>();
        parse.push(Operation {
            op: Op::Literal(num_str),
            position: token.position,
        });
    }

    fn parse_boolean_expression(
        &mut self,
        lhs_start: usize,
        lhs_end: usize,
        lhs_parse: Vec<Operation>,
        kind: TokenKind,
    ) -> Expression {
        let rhs_start = self.current.position.start;
        let mut rhs_parse = Vec::new();
        self.parse_arithmetic(&mut rhs_parse);
        let rhs_end = self.current.position.end;
        self.expect_end();

        Expression::Boolean {
            lhs: Computation {
                ops: lhs_parse,
                position: SourcePos::new(lhs_start, lhs_end),
            },
            rhs: Computation {
                ops: rhs_parse,
                position: SourcePos::new(rhs_start, rhs_end),
            },
            kind,
            position: SourcePos::new(lhs_start, rhs_end),
        }
    }

    fn parse_function(&mut self, parse: &mut Vec<Operation>, name: Token) {
        self.forward(); // Consume open parenthesis

        if self.current.kind == TokenKind::CloseParenthesis {
            self.forward();
            parse.push(Operation {
                op: Op::FnCall {
                    name: name.source,
                    arity: 0,
                },
                position: name.position,
            });
            return;
        }

        let mut arity: i32 = 0;
        loop {
            self.parse_arithmetic(parse);
            arity += 1;

            if self.current.kind == TokenKind::ArgumentSeparator {
                self.forward();
            } else {
                break;
            }
        }

        if self.current.kind == TokenKind::CloseParenthesis {
            self.forward();
        }

        parse.push(Operation {
            op: Op::FnCall {
                name: name.source,
                arity,
            },
            position: name.position,
        });
    }

    fn expect_end(&mut self) {
        if !ends_expr(self.current.kind) {
            self.unexpected_token(self.current.position, self.current.kind);
        }
        self.forward();
    }

    fn parse_variable_assignment(&mut self, lhs_start: usize, var: String) -> Expression {
        self.forward();
        let rhs_start = self.current.position.start;
        let mut rhs_parse = Vec::new();
        self.parse_arithmetic(&mut rhs_parse);
        let rhs_end = self.current.position.end;
        self.expect_end();
        Expression::Assignment {
            var,
            comp: Computation {
                ops: rhs_parse,
                position: SourcePos::new(rhs_start, rhs_end),
            },
            position: SourcePos::new(lhs_start, rhs_end),
        }
    }

    // This function actually returns the old current value, which is useful.
    fn forward(&mut self) -> Token {
        let new_current = self.lexer.next_token();
        mem::replace(&mut self.current, new_current)
    }

    fn unexpected_token(&mut self, err_pos: SourcePos, err_kind: TokenKind) {
        self.diagnostic_bag_mut().push(Diagnostic::new(
            err_pos,
            DiagnosticType::UnexpectedToken(err_kind),
        ));
    }

    pub fn diagnostic_bag(&self) -> &[Diagnostic] {
        self.lexer.diagnostic_bag()
    }

    fn diagnostic_bag_mut(&mut self) -> &mut Vec<Diagnostic> {
        self.lexer.diagnostic_bag_mut()
    }
}
