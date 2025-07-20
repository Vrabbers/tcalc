use std::fmt::Display;
use crate::maths_symbols::MathsSymbols;

pub enum AngleUnit {
    Degrees,
    Radians,
    Gradians
}

impl AngleUnit {
    pub fn symbol(&self) -> &'static str {
        match self {
            AngleUnit::Degrees => {"deg"}
            AngleUnit::Radians => {""}
            AngleUnit::Gradians => {"grad"}
        }
    }

    /// Human-readable string to convert an argument from Radians
    pub fn conversion_string(&self) -> String {
        match self {
            AngleUnit::Degrees => {format!("{}180/{}", MathsSymbols::Multiply, MathsSymbols::Pi)}
            AngleUnit::Radians => {"".to_string()}
            AngleUnit::Gradians => {"gradians".to_string()}
        }.to_string()
    }
}

impl Display for AngleUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.symbol())
    }
}