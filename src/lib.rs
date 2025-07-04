#[derive(Debug, Clone, Copy)]
pub struct SourcePosition(pub usize, pub usize);

mod string_reader;

pub mod diagnostics;
pub mod lexer;
pub mod parser;
pub mod token;
