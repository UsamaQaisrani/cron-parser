use clap::Parser;
use cron_parser::{output::format_expression, parser::parse_expression};

#[derive(Parser)]
struct Args {
    expr: String,
}

fn main() {
    let args = Args::parse();
    match parse_expression(&args.expr) {
        Ok(output) => {
            let display_str = format_expression(&output);
            print!("{}", display_str);
            std::process::exit(0);
        }
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    }
}
