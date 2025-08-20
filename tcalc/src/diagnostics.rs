use crate::{SourcePos, token::TokenKind};

#[derive(Debug, Clone, Copy)]
pub enum DiagnosticType {
    InvalidNumberLiteral,
    InvalidSymbol,
    UnexpectedToken(TokenKind),
}

#[derive(Debug, Clone)]
pub struct Diagnostic {
    position: SourcePos,
    diagnostic: DiagnosticType,
    arguments: Vec<String>,
}

impl Diagnostic {
    pub fn new_args(
        position: SourcePos,
        diagnostic: DiagnosticType,
        arguments: Vec<String>,
    ) -> Self {
        Self {
            position,
            diagnostic,
            arguments,
        }
    }

    pub fn new(position: SourcePos, diagnostic: DiagnosticType) -> Self {
        Self {
            position,
            diagnostic,
            arguments: Vec::with_capacity(0),
        }
    }

    pub fn position(&self) -> SourcePos {
        self.position
    }

    pub fn diagnostic(&self) -> DiagnosticType {
        self.diagnostic
    }

    pub fn arguments(&self) -> &[String] {
        &self.arguments
    }
}
