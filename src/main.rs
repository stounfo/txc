use clap::{Parser, ValueEnum};
use regex::Regex;
use std::io::{self, Read};
use titlecase::titlecase;
mod case_converter;

#[derive(Debug, Clone, ValueEnum, PartialEq)]
enum FormatType {
    Text,
    SingleLine,
    SliceOfLine,
    Word,
    SnakeCase,
    KebabCase,
    CamelCase,
    PascalCase,
    Title,
    Unknown,
}

/// Simple program to convert strings.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None, arg_required_else_help(true))]
struct Args {
    /// Type of format to convert from.
    #[arg(short, long, value_enum, default_value_t=FormatType::Unknown)]
    from: FormatType,
    /// Type of format to convert to.
    #[arg(short, long, value_enum, default_value_t=FormatType::Unknown)]
    to: FormatType,
}

fn guess_format(text: &str) -> FormatType {
    if text.ends_with('\n') && text.matches('\n').count() == 1 {
        FormatType::SingleLine
    } else if text.contains('\n') {
        FormatType::Text
    } else if text.contains(' ') {
        FormatType::SliceOfLine
    } else if text.contains('_') {
        FormatType::SnakeCase
    } else if text.contains('-') {
        FormatType::KebabCase
    } else if text.chars().any(|c| c.is_uppercase()) {
        if text.chars().next().unwrap().is_uppercase() {
            FormatType::PascalCase
        } else {
            FormatType::CamelCase
        }
    } else {
        FormatType::Word
    }
}

fn convert_to_single_line(to_convert: String, from: FormatType) -> String {
    match from {
        FormatType::Text => {
            // Regex to replace spaces/tabs before and after newlines
            let re_newline_spaces = Regex::new(r"\s*\n\s*").unwrap();
            let to_convert = re_newline_spaces.replace_all(&to_convert, " ");

            // Regex to replace multiple spaces/tabs with a single space
            let re_multiple_spaces = Regex::new(r"[ \t]+").unwrap();
            let to_convert = re_multiple_spaces.replace_all(&to_convert, " ");

            // Trim the text to remove leading and trailing spaces
            to_convert.trim().to_string() + "\n"
        }
        _ => to_convert,
    }
}

fn convert_to_pascal_case(to_convert: String, from: FormatType) -> String {
    match from {
        FormatType::SnakeCase => case_converter::snake_to_pascal(&to_convert),
        FormatType::KebabCase => case_converter::kebab_to_pascal(&to_convert),
        FormatType::CamelCase => case_converter::camel_to_pascal(&to_convert),
        _ => to_convert,
    }
}

fn convert_to_camel_case(to_convert: String, from: FormatType) -> String {
    match from {
        FormatType::SnakeCase => case_converter::snake_to_camel(&to_convert),
        FormatType::KebabCase => case_converter::kebab_to_camel(&to_convert),
        FormatType::PascalCase => case_converter::pascal_to_camel(&to_convert),
        _ => to_convert,
    }
}

fn convert_to_kebab_case(to_convert: String, from: FormatType) -> String {
    match from {
        FormatType::SnakeCase => case_converter::snake_to_kebab(&to_convert),
        FormatType::CamelCase => case_converter::camel_to_kebab(&to_convert),
        FormatType::PascalCase => case_converter::pascal_to_kebab(&to_convert),
        _ => to_convert,
    }
}

fn convert_to_snake_case(to_convert: String, from: FormatType) -> String {
    match from {
        FormatType::KebabCase => case_converter::kebab_to_snake(&to_convert),
        FormatType::CamelCase => case_converter::camel_to_snake(&to_convert),
        FormatType::PascalCase => case_converter::pascal_to_snake(&to_convert),
        _ => to_convert,
    }
}

fn convert_to_word(to_convert: String, from: FormatType) -> String {
    match from {
        FormatType::Text | FormatType::SingleLine | FormatType::SliceOfLine => {
            let re = Regex::new(r"\s+").unwrap();
            re.replace_all(&to_convert, "").to_string()
        }
        _ => to_convert,
    }
}

fn convert_to_title(to_convert: String, from: FormatType) -> String {
    match from {
        FormatType::SingleLine => titlecase(&to_convert) + "\n",
        FormatType::SliceOfLine => titlecase(&to_convert),
        _ => to_convert,
    }
}

fn convert(to_convert: String, from: FormatType, to: FormatType) -> String {
    match to {
        FormatType::Text => to_convert,
        FormatType::SingleLine => convert_to_single_line(to_convert, from),
        FormatType::SliceOfLine => to_convert,
        FormatType::Word => convert_to_word(to_convert, from),
        FormatType::SnakeCase => convert_to_snake_case(to_convert, from),
        FormatType::KebabCase => convert_to_kebab_case(to_convert, from),
        FormatType::CamelCase => convert_to_camel_case(to_convert, from),
        FormatType::PascalCase => convert_to_pascal_case(to_convert, from),
        FormatType::Title => convert_to_title(to_convert, from),
        FormatType::Unknown => to_convert,
    }
}

fn main() -> io::Result<()> {
    let args = Args::parse();
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let to_convert = buffer.clone();

    let from = if args.from == FormatType::Unknown {
        guess_format(&to_convert)
    } else {
        args.from
    };

    let result = convert(to_convert, from, args.to);

    print!("{}", result);
    Ok(())
}
