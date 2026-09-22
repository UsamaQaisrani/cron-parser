use crate::{error::ParseError, fields::FieldDef};
fn parse_value(text: &str, field: &FieldDef) -> Result<u32, ParseError> {
    match text.parse::<u32>() {
        Ok(num) => {
            if num >= field.min && num <= field.max {
                Ok(num)
            } else {
                Err(ParseError::OutOfRange {
                    field: field.name,
                    value: num,
                    min: field.min,
                    max: field.max,
                })
            }
        }
        Err(_) => Err(ParseError::InvalidNumber {
            field: field.name,
            input: text.to_string(),
        }),
    }
}

#[cfg(test)]
mod tests {
    use crate::fields::FIELDS;

    use super::*;

    #[test]
    fn test_valid_parse_input() {
        let input = "5";

        let result = parse_value(input, &FIELDS[0]).unwrap();
        assert_eq!(result, 5);
    }

    #[test]
    fn test_parse_value_returns_range_error() {
        let input = "60";

        let result = parse_value(input, &FIELDS[0]);
        assert_eq!(
            result,
            Err(ParseError::OutOfRange {
                field: "minute",
                value: 60,
                min: 0,
                max: 59
            })
        );
    }

    #[test]
    fn test_parse_value_returns_invalid_number_error() {
        let input = "abc";

        let result = parse_value(input, &FIELDS[0]);
        assert_eq!(
            result,
            Err(ParseError::InvalidNumber {
                field: "minute",
                input: "abc".to_string(),
            })
        );
    }

    #[test]
    fn test_parse_value_minutes_upper_edge_case_success() {
        let input = "59";
        let result = parse_value(input, &FIELDS[0]).unwrap();

        assert_eq!(result, 59);
    }

    #[test]
    fn test_parse_value_minutes_lower_edge_case_success() {
        let input = "0";
        let result = parse_value(input, &FIELDS[0]).unwrap();

        assert_eq!(result, 0);
    }

    #[test]
    fn test_parse_value_month_edge_case_returns_range_error() {
        let input = "0";
        let result = parse_value(input, &FIELDS[3]);

        assert_eq!(
            result,
            Err(ParseError::OutOfRange {
                field: "month",
                value: 0,
                min: FIELDS[3].min,
                max: FIELDS[3].max
            })
        );
    }
}
