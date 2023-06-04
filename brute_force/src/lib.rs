pub fn brute_force(text: &str, pattern: &str) -> Vec<usize> {
    let text_len = text.len();
    let pattern_len = pattern.len();
    let mut positions = Vec::new();

    if pattern_len == 0 {
        return positions;
    }

    if pattern_len > text_len {
        return positions;
    }

    for i in 0..(text_len - pattern_len + 1) {
        if &text[i..i + pattern_len] == pattern {
            positions.push(i);
        }
    }
    positions
}
