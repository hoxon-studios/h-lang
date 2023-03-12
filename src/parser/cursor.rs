pub fn skip_space(code: &str) -> &str {
    code.trim_start()
}

pub fn eat_number(code: &str) -> Option<(&str, &str)> {
    if let Some(mut cursor) = eat_token(code, "0x") {
        let mut length = 2;
        while let Some((next, '0'..='9' | 'a'..='f' | 'A'..='F')) = eat_char(cursor) {
            length += 1;
            cursor = next;
        }

        Some((cursor, &code[0..length]))
    } else if let Some(mut cursor) = eat_token(code, "0b") {
        let mut length = 2;
        while let Some((next, '0' | '1')) = eat_char(cursor) {
            length += 1;
            cursor = next;
        }

        Some((cursor, &code[0..length]))
    } else if let Some((mut cursor, '-' | '0'..='9')) = eat_char(code) {
        let mut length = 1;
        while let Some((next, '0'..='9')) = eat_char(cursor) {
            length += 1;
            cursor = next;
        }

        Some((cursor, &code[0..length]))
    } else {
        None
    }
}

pub fn eat_char(code: &str) -> Option<(&str, char)> {
    let mut cursor = code.chars();
    let Some(c) = cursor.next() else {
        return None;
    };

    Some((cursor.as_str(), c))
}

pub fn eat_token<'a>(code: &'a str, token: &'static str) -> Option<&'a str> {
    if code.starts_with(token) {
        Some(&code[token.len()..])
    } else {
        None
    }
}

pub fn eat_id(code: &str) -> Option<(&str, &str)> {
    let mut cursor = code.chars();
    let mut is_start = true;

    let mut length: usize = 0;
    let mut end;
    loop {
        end = cursor.as_str();
        if is_start {
            let Some('_' | 'A'..='Z' | 'a'..='z') = cursor.next() else {
                return None;
            };
            is_start = false;
            length += 1;
        } else {
            match cursor.next() {
                Some('_' | 'A'..='Z' | 'a'..='z' | '0'..='9') => {
                    length += 1;
                }
                _ => {
                    break;
                }
            }
        }
    }

    Some((end, &code[0..length]))
}

pub fn eat_string(code: &str) -> Option<(&str, &str)> {
    let code = skip_space(code);
    let mut chars = code.chars();
    if let Some('"') = chars.next() {
        let result = chars.as_str();
        let mut length = 0;
        loop {
            if let Some(next) = chars.next() {
                if next == '\\' {
                    if let None = chars.next() {
                        return None;
                    } else {
                        length += 2;
                    }
                } else if next == '"' {
                    return Some((chars.as_str(), &result[0..length]));
                } else {
                    length += 1;
                }
            } else {
                return None;
            }
        }
    } else {
        None
    }
}
