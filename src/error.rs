use std::fmt::Display;

#[derive(PartialEq, Debug)]
pub enum ParseError {
    OutOfRange {
        field: &'static str,
        value: u32,
        min: u32,
        max: u32,
    },
    InvalidNumber {
        field: &'static str,
        input: String,
    },
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::OutOfRange {
                field,
                value,
                min,
                max,
            } => {
                write!(
                    f,
                    "Field: {}, Input: {} Out of valid range {}-{}",
                    field, value, min, max,
                )
            }
            ParseError::InvalidNumber { field, input } => {
                write!(f, "Field: {}, Expected a Number, Got: {}", field, input)
            }
        }
    }
}
