//! Shared utility code and types.

#[inline]
pub fn is_token_id(s: &str) -> bool {
    return s.chars().all(|c| {
        if c.is_alphabetic() {
            c.is_uppercase()
        } else {
            true
        }
    })
}

/// Returns a 1-indexed line/column pair from a text offset.
pub fn get_position(text: &str, byte_offset: usize) -> (usize, usize) {
    let text = &text[..byte_offset];
    let mut line = 1;
    let mut col = 1;

    for ch in text.chars() {
        if ch == '\n' {
            line += 1;
            col = 1;
        } else {
            col += 1;
        }
    }
    (line, col)
}
