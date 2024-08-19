/// uppercase the first title case of the string
///
/// if the first title case is ascii alphabetic it will back the uppercase String
/// else back the original String
pub fn uppercase_title(s: &str) -> Option<String> {
    s.char_indices()
        .next() //get the first char
        .and_then(|(i, c)| {
            if c.is_ascii_alphabetic() {
                // but first char back
                Some(c.to_uppercase().collect::<String>() + &s[i + 1..])
            } else {
                None
            }
        })
}

/// consume original String to surround String
///
/// format: `surround_left_sign`xxx`surround_right_sign`
pub fn surround(s: String, l: &str, r: &str) -> String {
    format!("{}{}{}", l, s, r)
}

/// convert camel to snake case
/// 1. View => view
/// 2. ViewName => view_name
pub fn camel_to_snake(s: &str) -> String {
    let chars: Vec<char> = s.chars().collect();
    let mut result = String::new();

    for (i, &c) in chars.iter().enumerate() {
        if c.is_uppercase() && i != 0 {
            result.push('_');
        }
        result.push(c.to_lowercase().next().unwrap());
    }

    result
}

pub fn snake_to_camel(s: &str) -> String {
    if s.eq("checkbox") {
        return "CheckBox".to_string();
    }

    if s.contains("_") {
        return s
            .split('_')
            .map(|part| {
                let mut c = part.chars();
                match c.next() {
                    None => String::new(),
                    Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
                }
            })
            .collect();
    } else {
        if let Some(s) = uppercase_title(s) {
            return s;
        } else {
            return s.to_string();
        }
    }
}

#[cfg(test)]
mod test_utils {
    use super::snake_to_camel;

    #[test]
    fn snake() {
        let name = "checkbox";
        let handled = snake_to_camel(name);
        dbg!(handled);
    }
}
