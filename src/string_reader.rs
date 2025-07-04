use crate::SourcePosition;

#[derive(Debug, Clone)]
pub struct StringReader {
    start: usize,
    end: usize,
    str: String,
    pub current: Option<char>,
}

impl StringReader {
    pub fn new(str: String) -> Self {
        Self {
            start: 0,
            end: 0,
            str,
            current: None,
        }
    }

    pub fn peek(&self) -> Option<char> {
        self.str[self.end..].chars().next()
    }

    pub fn peek_many(&self, count: usize) -> Vec<char> {
        self.str[self.end..].chars().take(count).collect()
    }

    pub fn forward(&mut self) -> Option<char> {
        if let Some((l, c)) = self.str[self.start..].char_indices().next() {
            self.end += l + 1;
            self.current = Some(c);
        } else {
            self.current = None;
        }
        self.current
    }

    pub fn forward_many(&mut self, count: i32) {
        for _ in 0..count {
            _ = self.forward();
        }
    }

    pub fn flush(&mut self) -> (SourcePosition, String) {
        let substr = String::from(&self.str[self.start..self.end]);
        let pos = SourcePosition(self.start, self.end);
        self.discard_token();
        (pos, substr)
    }

    pub fn discard_token(&mut self) {
        self.start = self.end;
    }
}
