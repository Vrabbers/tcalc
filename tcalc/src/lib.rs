#[derive(Debug, Clone, Copy)]
pub struct SourcePos { pub start: usize, pub end: usize }

impl SourcePos {
    pub fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }
}

mod string_reader;

pub mod diagnostics;
pub mod lexer;
pub mod parser;
pub mod token;
pub mod expressions;
