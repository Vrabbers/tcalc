use crate::SourcePosition;

#[derive(Debug, Clone, Copy)]
pub enum DiagnosticType {
    InvalidNumberLiteral,
    InvalidSymbol,
    UnexpectedToken,
}

#[derive(Debug, Clone)]
pub struct Diagnostic {
    position: SourcePosition,
    diagnostic: DiagnosticType,
    arguments: Vec<String>,
}

impl Diagnostic {
    pub fn new_args(
        position: SourcePosition,
        diagnostic: DiagnosticType,
        arguments: Vec<String>,
    ) -> Self {
        Self {
            position,
            diagnostic,
            arguments,
        }
    }

    pub fn new(position: SourcePosition, diagnostic: DiagnosticType) -> Self {
        Self {
            position,
            diagnostic,
            arguments: Vec::with_capacity(0),
        }
    }

    pub fn position(&self) -> SourcePosition {
        self.position
    }

    pub fn diagnostic(&self) -> DiagnosticType {
        self.diagnostic
    }

    pub fn arguments(&self) -> &[String] {
        &self.arguments
    }
}
