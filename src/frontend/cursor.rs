pub fn skip_space(code: &str) -> &str {
    code.trim_start()
}

pub fn eat_number(code: &str) -> Option<(&str, &str)> {
    if let Some(mut cursor) = eat_token(code, "0x") {
        let mut length = 2;
        while let Some((next, '0'..='9' | 'a'..='f' | 'A'..='F')) = eat_char(code) {
            length += 1;
            cursor = next;
        }

        Some((cursor, &code[0..length]))
    } else if let Some(mut cursor) = eat_token(code, "0b") {
        let mut length = 2;
        while let Some((next, '0' | '1')) = eat_char(code) {
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

pub fn eat_label(code: &str) -> Option<(&str, &str)> {
    let mut cursor = code.chars();
    let Some('_' | 'A'..='Z' | 'a'..='z') = cursor.next() else {
            return None;
        };

    let mut length: usize = 1;
    let mut end;
    loop {
        end = cursor.as_str();
        match cursor.next() {
            Some('_' | 'A'..='Z' | 'a'..='z' | '0'..='9') => {
                length += 1;
            }
            _ => {
                break;
            }
        }
    }

    Some((end, &code[0..length]))
}
