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

fn parse_step(text: &str, field: &FieldDef) -> Result<Vec<u32>, ParseError> {
    if let Some((left, right)) = text.split_once('/') {
        let step = right
            .parse::<u32>()
            .map_err(|_| ParseError::InvalidNumber {
                field: field.name,
                input: right.to_string(),
            })?;
        if step == 0 {
            return Err(ParseError::InvalidStep {
                field: field.name,
                input: right.to_string(),
            });
        }
        if !left.contains('-') && left != "*" {
            let start = parse_value(left, field)?;
            Ok((start..=field.max).step_by(step as usize).collect())
        } else {
            let curr_range = parse_range(left, field)?;
            Ok(curr_range.into_iter().step_by(step as usize).collect())
        }
    } else {
        let range = parse_range(text, field)?;
        Ok(range)
    }
}

fn parse_field(text: &str, field: &FieldDef) -> Result<Vec<u32>, ParseError> {
    let parts: Vec<&str> = text.split(',').collect();
    let mut fields: Vec<u32> = Vec::new();

    for part in parts {
        let mut res = parse_step(part, field)?;
        fields.append(&mut res);
    }

    fields.sort();
    fields.dedup();

    Ok(fields)
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
        assert_eq!(result.first(), Some(&0));
        assert_eq!(result.last(), Some(&59));
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

    #[test]
    fn test_parse_step_wildcard_with_step() {
        let input = "*/15";
        let expected = [0, 15, 30, 45];
        let output = parse_step(input, &FIELDS[0]).unwrap();
        assert_eq!(output, expected);
    }

    #[test]
    fn test_parse_step_starts_at_field_min() {
        let input = "*/10";
        let expected = [1, 11, 21, 31];
        let output = parse_step(input, &FIELDS[2]).unwrap();
        assert_eq!(output, expected);
    }

    #[test]
    fn test_parse_step_range_with_step() {
        let input = "1-10/3";
        let expected = [1, 4, 7, 10];
        let output = parse_step(input, &FIELDS[0]).unwrap();
        assert_eq!(output, expected);
    }

    #[test]
    fn test_parse_step_single_value_expands_to_max() {
        let input = "5/15";
        let expected = [5, 20, 35, 50];
        let output = parse_step(input, &FIELDS[0]).unwrap();
        assert_eq!(output, expected);
    }

    #[test]
    fn test_parse_step_range_base_not_expanded() {
        let input = "5-5/15";
        let expected = [5];
        let output = parse_step(input, &FIELDS[0]).unwrap();
        assert_eq!(output, expected);
    }

    #[test]
    fn test_parse_step_propagates_invalid_range() {
        let input = "10-5/2";
        let expected = Err(ParseError::InvalidRange {
            field: FIELDS[0].name,
            input: "10-5".to_string(),
        });
        let output = parse_step(input, &FIELDS[0]);
        assert_eq!(output, expected);
    }

    #[test]
    fn test_parse_step_zero_step_returns_error() {
        let input = "*/0";
        let expected = Err(ParseError::InvalidStep {
            field: FIELDS[0].name,
            input: "0".to_string(),
        });
        let output = parse_step(input, &FIELDS[0]);
        assert_eq!(output, expected);
    }

    #[test]
    fn test_parse_step_non_numeric_step_returns_error() {
        let input = "*/abc";
        let expected = Err(ParseError::InvalidNumber {
            field: FIELDS[0].name,
            input: "abc".to_string(),
        });
        let output = parse_step(input, &FIELDS[0]);
        assert_eq!(output, expected);
    }

    #[test]
    fn test_parse_step_single_value_zero_step_returns_error() {
        let input = "5/0";
        let expected = Err(ParseError::InvalidStep {
            field: FIELDS[0].name,
            input: "0".to_string(),
        });
        let output = parse_step(input, &FIELDS[0]);
        assert_eq!(output, expected);
    }

    #[test]
    fn test_parse_field_single_value() {
        let input = "5";
        let expected = [5];

        let output = parse_field(input, &FIELDS[0]).unwrap();
        assert_eq!(output, expected);
    }

    #[test]
    fn test_parse_field_list_of_value_and_range() {
        let input = "1,5-7";
        let expected = [1, 5, 6, 7];

        let output = parse_field(input, &FIELDS[0]).unwrap();
        assert_eq!(output, expected);
    }

    #[test]
    fn test_parse_field_removes_duplicates() {
        let input = "1-5,3";
        let expected = [1, 2, 3, 4, 5];

        let output = parse_field(input, &FIELDS[0]).unwrap();
        assert_eq!(output, expected);
    }

    #[test]
    fn test_parse_field_sorts_values() {
        let input = "30,0,15";
        let expected = [0, 15, 30];

        let output = parse_field(input, &FIELDS[0]).unwrap();
        assert_eq!(output, expected);
    }

    #[test]
    fn test_parse_field_list_with_step() {
        let input = "*/20,0";
        let expected = [0, 20, 40];

        let output = parse_field(input, &FIELDS[0]).unwrap();
        assert_eq!(output, expected);
    }

    #[test]
    fn test_parse_field_propagates_error() {
        let input = "1,abc";
        let expected = Err(ParseError::InvalidNumber {
            field: FIELDS[0].name,
            input: "abc".to_string(),
        });

        let output = parse_field(input, &FIELDS[0]);
        assert_eq!(output, expected);
    }

    #[test]
    fn test_parse_field_empty_piece_returns_error() {
        let input = "1,,2";
        let expected = Err(ParseError::InvalidNumber {
            field: FIELDS[0].name,
            input: "".to_string(),
        });

        let output = parse_field(input, &FIELDS[0]);
        assert_eq!(output, expected);
    }
}
