pub fn snake_to_camel(s: &str) -> String {
    s.split('_')
        .enumerate()
        .map(|(i, word)| {
            if i == 0 {
                word.to_string()
            } else {
                word[..1].to_uppercase() + &word[1..]
            }
        })
        .collect()
}

pub fn snake_to_kebab(s: &str) -> String {
    s.replace('_', "-")
}

pub fn snake_to_pascal(s: &str) -> String {
    s.split('_')
        .map(|word| word[..1].to_uppercase() + &word[1..])
        .collect()
}

pub fn camel_to_snake(s: &str) -> String {
    let mut result = String::new();
    for (i, char) in s.chars().enumerate() {
        if char.is_uppercase() && i != 0 {
            result.push('_');
        }
        result.push(char.to_ascii_lowercase());
    }
    result
}

pub fn camel_to_kebab(s: &str) -> String {
    camel_to_snake(s).replace('_', "-")
}

pub fn camel_to_pascal(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

pub fn kebab_to_snake(s: &str) -> String {
    s.replace('-', "_")
}

pub fn kebab_to_camel(s: &str) -> String {
    s.split('-')
        .enumerate()
        .map(|(i, word)| {
            if i == 0 {
                word.to_string()
            } else {
                word[..1].to_uppercase() + &word[1..]
            }
        })
        .collect()
}

pub fn kebab_to_pascal(s: &str) -> String {
    s.split('-')
        .map(|word| word[..1].to_uppercase() + &word[1..])
        .collect()
}

pub fn pascal_to_snake(s: &str) -> String {
    let mut result = String::new();
    for (i, char) in s.chars().enumerate() {
        if char.is_uppercase() && i != 0 {
            result.push('_');
        }
        result.push(char.to_ascii_lowercase());
    }
    result
}

pub fn pascal_to_camel(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_lowercase().collect::<String>() + c.as_str(),
    }
}

pub fn pascal_to_kebab(s: &str) -> String {
    pascal_to_snake(s).replace('_', "-")
}

pub fn slice_of_line_to_pascal(s: &str) -> String {
    s.split_whitespace()
        .filter(|word| !word.is_empty())
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                None => String::new(),
                Some(first) => {
                    first.to_uppercase().collect::<String>()
                        + &chars.as_str().to_lowercase()
                }
            }
        })
        .collect::<String>()
}

pub fn slice_of_line_to_camel(s: &str) -> String {
    let mut iter = s
        .split_whitespace()
        .filter(|word| !word.is_empty())
        .map(|word| word.to_lowercase());

    let first_word = match iter.next() {
        None => return String::new(),
        Some(w) => w,
    };

    let mut result = first_word;
    for word in iter {
        let mut chars = word.chars();
        match chars.next() {
            None => {}
            Some(first) => {
                result.push_str(&first.to_uppercase().to_string());
                result.push_str(chars.as_str());
            }
        }
    }
    result
}

pub fn slice_of_line_to_snake(s: &str) -> String {
    s.split_whitespace()
        .filter(|word| !word.is_empty())
        .map(|word| word.to_lowercase())
        .collect::<Vec<String>>()
        .join("_")
}

pub fn slice_of_line_to_kebab(s: &str) -> String {
    s.split_whitespace()
        .filter(|word| !word.is_empty())
        .map(|word| word.to_lowercase())
        .collect::<Vec<String>>()
        .join("-")
}
