#[derive(Debug, Clone, Copy)]
pub struct SourcePos {
    pub start: usize,
    pub end: usize,
}

impl SourcePos {
    pub fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }
}
