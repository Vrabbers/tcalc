use std::fmt::Display;

pub enum MathsSymbols {
    Pi,
    Sqrt,
    Multiply
}

impl Display for MathsSymbols {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            MathsSymbols::Pi => "π".to_string(),
            MathsSymbols::Sqrt => "√".to_string(),
            MathsSymbols::Multiply => "×".to_string()
        })
    }
}