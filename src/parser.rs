use crate::{error::ParseError, fields::FieldDef};
fn parse_value(text: &str, field: &FieldDef) -> Result<u32, ParseError> {
    match text.parse::<u32>() {
        Ok(num) => {
            if num >= field.min && num <= field.max {
                Ok(num)
            } else {
                Err(ParseError::OutOfRange {
                    field: field.name,
                    value: text.to_string(),
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

fn parse_range(text: &str, field: &FieldDef) -> Result<Vec<u32>, ParseError> {
    if text == "*" {
        Ok((field.min..=field.max).collect())
    } else if let Some((start, end)) = text.split_once('-') {
        let start = parse_value(start, field)?;
        let end = parse_value(end, field)?;
        if start > end {
            Err(ParseError::InvalidRange {
                field: field.name,
                input: text.to_string(),
            })
        } else {
            Ok((start..=end).collect())
        }
    } else {
        let value = parse_value(text, field)?;
        Ok(vec![value])
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
                value: "60".to_string(),
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
                value: "0".to_string(),
                min: FIELDS[3].min,
                max: FIELDS[3].max
            })
        );
    }

    #[test]
    fn test_parse_range_minutes_asterisk_success() {
        let input = "*";

        let result = parse_range(input, &FIELDS[0]).unwrap();
        assert_eq!(result.len(), 60);
        assert!(result.first() == Some(&0));
        assert!(result.last() == Some(&59));
    }

    #[test]
    fn test_parse_range_month_asterisk_success() {
        let input = "*";

        let result = parse_range(input, &FIELDS[3]).unwrap();
        assert_eq!(result.len(), 12);
        assert_eq!(result, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]);
    }

    #[test]
    fn test_parse_range_minutes_range_success() {
        let input = "1-5";

        let result = parse_range(input, &FIELDS[0]).unwrap();
        assert_eq!(result, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_parse_range_minutes_range_same_start_and_end() {
        let input = "5-5";

        let result = parse_range(input, &FIELDS[0]).unwrap();
        assert_eq!(result, vec![5]);
    }

    #[test]
    fn test_parse_invalid_range_returns_error() {
        let input = "10-5";

        let result = parse_range(input, &FIELDS[0]);
        assert_eq!(
            result,
            Err(ParseError::InvalidRange {
                field: FIELDS[0].name,
                input: input.to_string()
            })
        );
    }

    #[test]
    fn test_parse_range_out_of_range_returns_error() {
        let input = "1-60";

        let result = parse_range(input, &FIELDS[0]);
        assert_eq!(
            result,
            Err(ParseError::OutOfRange {
                field: FIELDS[0].name,
                value: "60".to_string(),
                min: FIELDS[0].min,
                max: FIELDS[0].max
            })
        );
    }

    #[test]
    fn test_parse_range_single_number_success() {
        let input = "7";

        let result = parse_range(input, &FIELDS[0]).unwrap();
        assert_eq!(result, vec![7]);
    }
}
