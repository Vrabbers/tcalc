use unicode_categories::UnicodeCategories;

use crate::{
    diagnostics::{Diagnostic, DiagnosticType},
    string_reader::StringReader,
    token::{Token, TokenKind},
};

#[derive(Debug, Clone)]
pub struct Lexer {
    sr: StringReader,
    pub(crate) comma_is_arg_separator: bool,
    reached_end: bool,
    pub(crate) diagnostic_bag: Vec<Diagnostic>,
}

fn is_whitespace(c: char) -> bool {
    c.is_separator_space()
}

fn is_letter(c: char) -> bool {
    c.is_letter() || c.is_punctuation_connector() || c.is_number_other()
}

fn is_hex_digit(c: char) -> bool {
    c.is_ascii_digit() || ('a'..='f').contains(&c) || ('A'..='F').contains(&c)
}

fn is_superscript_digit(c: char) -> bool {
    matches!(c, '²' | '³' | '¹' | '⁰' | '⁴' | '⁵' | '⁶' | '⁷' | '⁸' | '⁹')
}

#[derive(Clone, Copy)]
enum DigitStyle {
    Decimal,
    Superscript,
}

const fn plus(s: DigitStyle) -> char {
    match s {
        DigitStyle::Decimal => '+',
        DigitStyle::Superscript => '⁺',
    }
}

const fn minus(s: DigitStyle) -> char {
    match s {
        DigitStyle::Decimal => '-',
        DigitStyle::Superscript => '⁻',
    }
}

fn is_digit(c: char, s: DigitStyle) -> bool {
    match s {
        DigitStyle::Decimal => c.is_ascii_digit(),
        DigitStyle::Superscript => is_superscript_digit(c),
    }
}

fn start_reading_exponent(sr: &mut StringReader, style: DigitStyle) -> bool {
    let next3 = sr.peek_many(3);

    if next3.len() >= 2 {
        if next3[1] == plus(style) || next3[1] == minus(style) {
            if next3.len() == 3 && is_digit(next3[2], style) {
                sr.forward_many(3);
                true
            } else {
                false
            }
        } else {
            sr.forward_many(2);
            true
        }
    } else {
        false
    }
}

impl Iterator for Lexer {
    type Item = Token;
    fn next(&mut self) -> Option<Self::Item> {
        if self.reached_end {
            None
        } else {
            Some(self.next_token())
        }
    }
}

impl Lexer {
    pub fn new(str: String, comma_is_arg_separator: bool) -> Self {
        Self {
            sr: StringReader::new(str),
            comma_is_arg_separator,
            reached_end: false,
            diagnostic_bag: Vec::new(),
        }
    }

    pub fn diagnostic_bag(&self) -> &[Diagnostic] {
        &self.diagnostic_bag
    }

    pub fn diagnostic_bag_mut(&mut self) -> &mut Vec<Diagnostic> {
        &mut self.diagnostic_bag
    }

    pub fn reached_end(&self) -> bool {
        self.reached_end
    }

    pub fn next_token(&mut self) -> Token {
        while self.sr.peek().map(is_whitespace).unwrap_or_default() {
            self.sr.forward();
        }

        self.sr.discard_token();

        if let Some(c) = self.sr.forward() {
            if c.is_ascii_digit() {
                self.lex_number()
            } else if is_superscript_digit(c) {
                self.lex_superscript_number()
            } else if is_letter(c) {
                self.lex_word()
            } else {
                self.lex_symbol()
            }
        } else {
            self.reached_end = true;
            self.flush_token(TokenKind::EndOfFile)
        }
    }

    fn decimal_sep(&self) -> char {
        if self.comma_is_arg_separator {
            '.'
        } else {
            ','
        }
    }

    fn arg_sep(&self) -> char {
        if self.comma_is_arg_separator {
            ','
        } else {
            ';'
        }
    }

    fn flush_token(&mut self, kind: TokenKind) -> Token {
        let (position, source) = self.sr.flush();
        Token {
            source,
            position,
            kind,
        }
    }

    fn lex_number(&mut self) -> Token {
        let first = self.sr.current;
        let next = self.sr.peek();
        match (first, next) {
            (Some('0'), Some('b')) => self.lex_binary_number(),
            (Some('0'), Some('x')) => self.lex_hex_number(),
            _ => self.lex_decimal_number(),
        }
    }

    fn lex_binary_number(&mut self) -> Token {
        self.sr.forward();
        while self
            .sr
            .peek()
            .map(|c| matches!(c, '0' | '1' | '_' | '\''))
            .unwrap_or_default()
        {
            self.sr.forward();
        }

        self.flush_token(TokenKind::BinaryLiteral)
    }

    fn lex_hex_number(&mut self) -> Token {
        self.sr.forward();
        while self
            .sr
            .peek()
            .map(|c| is_hex_digit(c) || matches!(c, '_' | '\''))
            .unwrap_or_default()
        {
            self.sr.forward();
        }

        self.flush_token(TokenKind::HexLiteral)
    }

    fn lex_decimal_number(&mut self) -> Token {
        let mut reading_decimal = false;
        let mut reading_exponent = false;
        loop {
            let next = self.sr.peek();

            if next.map(|c| c.is_ascii_digit()).unwrap_or_default()
                || matches!(next, Some('\'') | Some('_'))
            {
                self.sr.forward();
            } else if next == Some(self.decimal_sep()) {
                self.sr.forward();
                if !reading_decimal && !reading_exponent {
                    reading_decimal = true;
                } else {
                    let token = self.flush_token(TokenKind::Bad);
                    self.diagnostic_bag.push(Diagnostic::new(
                        token.position,
                        DiagnosticType::InvalidNumberLiteral,
                    ));
                    return token;
                }
            } else if matches!(next, Some('e') | Some('E')) {
                if !reading_exponent {
                    reading_exponent = start_reading_exponent(&mut self.sr, DigitStyle::Decimal);
                    if reading_exponent {
                        continue;
                    }
                }
                return self.flush_token(TokenKind::NumericLiteral);
            } else {
                if next == Some('i') {
                    self.sr.forward();
                }
                return self.flush_token(TokenKind::NumericLiteral);
            }
        }
    }

    fn lex_superscript_number(&mut self) -> Token {
        let mut reading_exponent = false;
        loop {
            let next = self.sr.peek();
            if next.map(is_superscript_digit).unwrap_or_default() {
                self.sr.forward();
            } else if matches!(next, Some('ᵉ') | Some('ᴱ')) {
                if !reading_exponent {
                    reading_exponent =
                        start_reading_exponent(&mut self.sr, DigitStyle::Superscript);
                    if reading_exponent {
                        continue;
                    }
                }
            } else {
                if next == Some('ⁱ') {
                    self.sr.forward();
                }

                return self.flush_token(TokenKind::SuperscriptLiteral);
            }
        }
    }

    fn lex_word(&mut self) -> Token {
        let mut peek = self.sr.peek();

        while peek
            .map(|c| (is_letter(c) || c.is_ascii_digit()) && !is_superscript_digit(c))
            .unwrap_or_default()
        {
            self.sr.forward();
            peek = self.sr.peek();
        }

        let (position, source) = self.sr.flush();

        let kind = match source.as_str() {
            "NAND" => TokenKind::BinaryNand,
            "NOR" => TokenKind::BinaryNor,
            "XNOR" => TokenKind::BinaryXnor,
            "AND" => TokenKind::BinaryAnd,
            "OR" => TokenKind::BinaryOr,
            "XOR" => TokenKind::BinaryXor,
            "NOT" => TokenKind::BinaryNot,
            "i" => TokenKind::NumericLiteral,
            "ⁱ" => TokenKind::SuperscriptLiteral,
            "deg" => TokenKind::Deg,
            "rad" => TokenKind::Rad,
            "grad" => TokenKind::Grad,
            _ => TokenKind::Identifier,
        };

        Token {
            kind,
            source,
            position,
        }
    }

    fn lex_symbol(&mut self) -> Token {
        let peek = self.sr.peek();
        match self.sr.current.unwrap() {
            '+' => self.flush_token(TokenKind::Plus),
            '-' => self.flush_token(TokenKind::Minus),
            '⁻' => self.flush_token(TokenKind::SuperscriptMinus),
            '*' | '×' | '⋅' => self.flush_token(TokenKind::Multiply),
            '/' | '÷' => self.flush_token(TokenKind::Divide),
            '^' => self.flush_token(TokenKind::Exponentiate),
            '(' => self.flush_token(TokenKind::OpenParenthesis),
            ')' => self.flush_token(TokenKind::CloseParenthesis),
            '√' => self.flush_token(TokenKind::Radical),
            '∛' => self.flush_token(TokenKind::CubeRoot),
            '∜' => self.flush_token(TokenKind::FourthRoot),
            '%' => self.flush_token(TokenKind::Percent),
            '≥' => self.flush_token(TokenKind::GreaterOrEqual),
            '≤' => self.flush_token(TokenKind::LessOrEqual),
            '≠' => self.flush_token(TokenKind::NotEqual),
            '\n' | ':' => self.flush_token(TokenKind::ExpressionSeparator),
            '°' => self.flush_token(TokenKind::Deg),

            '!' if peek == Some('=') => {
                self.sr.forward();
                self.flush_token(TokenKind::NotEqual)
            }
            '!' => self.flush_token(TokenKind::Factorial),

            '>' if peek == Some('>') => {
                self.sr.forward();
                self.flush_token(TokenKind::RightShift)
            }
            '>' if peek == Some('=') => {
                self.sr.forward();
                self.flush_token(TokenKind::GreaterOrEqual)
            }
            '>' => self.flush_token(TokenKind::GreaterThan),

            '<' if peek == Some('<') => {
                self.sr.forward();
                self.flush_token(TokenKind::LeftShift)
            }
            '<' if peek == Some('=') => {
                self.sr.forward();
                self.flush_token(TokenKind::LessOrEqual)
            }
            '<' => self.flush_token(TokenKind::LessThan),

            '=' if peek == Some('=') => {
                self.sr.forward();
                self.flush_token(TokenKind::Equality)
            }
            '=' => self.flush_token(TokenKind::Equal),

            x if x == self.arg_sep() => self.flush_token(TokenKind::ArgumentSeparator),
            _ => {
                let token = self.flush_token(TokenKind::Bad);
                self.diagnostic_bag.push(Diagnostic::new(
                    token.position,
                    DiagnosticType::InvalidSymbol,
                ));
                token
            }
        }
    }
}
