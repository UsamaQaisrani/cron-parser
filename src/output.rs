use crate::{fields::FIELDS, parser::CronExpression};

pub fn format_expression(expr: &CronExpression) -> String {
    let mut display_str: String = String::new();
    for (field, value) in FIELDS.iter().zip(&expr.fields) {
        let value_str = value
            .iter()
            .map(|n| n.to_string())
            .collect::<Vec<String>>()
            .join(" ");

        display_str += &format!("{:<14}", field.name);
        display_str += &value_str;
        display_str.push('\n');
    }
    display_str += &format!("{:<14}", "command");
    display_str += &expr.command;
    display_str.push('\n');

    display_str
}
