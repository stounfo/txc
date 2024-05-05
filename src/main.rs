use clap::{Parser, ValueEnum};
use regex::Regex;
use std::io::{self, Read};

fn to_one_line(text: String) -> String {
    // Regex to replace spaces/tabs before and after newlines
    let re_newline_spaces = Regex::new(r"\s*\n\s*").unwrap();
    let text = re_newline_spaces.replace_all(&text, " ");

    // Regex to replace multiple spaces/tabs with a single space
    let re_multiple_spaces = Regex::new(r"[ \t]+").unwrap();
    let text = re_multiple_spaces.replace_all(&text, " ");

    // Trim the text to remove leading and trailing spaces
    text.trim().to_string()
}

#[derive(Debug, Clone, ValueEnum)]
enum FormatType {
    CamelCase,
    SnakeCase,
    Title,
    OneLine,
}

/// Simple program to format strings
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Type to format
    #[arg(short, long, value_enum, default_value_t=FormatType::OneLine)]
    format_to: FormatType,
}

fn main() -> io::Result<()> {
    let args = Args::parse();
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let text = buffer.clone();

    let result = match args.format_to {
        FormatType::OneLine => to_one_line(text),
        _ => "not implemented".to_string(),
    };

    println!("{}", result);
    Ok(())
}
