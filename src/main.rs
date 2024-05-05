use clap::{Parser, ValueEnum};
use regex::Regex;
use std::io::{self, Read};

#[derive(Debug, Clone, ValueEnum)]
enum FormatType {
    CamelCase,
    SnakeCase,
    Title,
    ToOneLine,
}

/// Simple program to reformat strings
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Format type
    #[arg(short, long, value_enum, default_value_t=FormatType::ToOneLine)]
    format: FormatType,
}

fn to_one_line(text: String) -> String {
    let text = text.trim(); // Trim the string first
    let regex = Regex::new(r"[ \t]*\n[ \t]*").unwrap(); // Compile the regex to match spaces/tabs around newlines
    let result = regex.replace_all(text, " "); // Replace each match with a single space
    result.into_owned() // Convert the result back into a String
}

fn main() -> io::Result<()> {
    let args = Args::parse();
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let text = buffer.clone();

    let result = match args.format {
        FormatType::ToOneLine => to_one_line(text),
        _ => "not implemented".to_string(),
    };

    println!("{}", result);
    Ok(())
}
