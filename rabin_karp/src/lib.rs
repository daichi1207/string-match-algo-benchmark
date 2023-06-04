pub fn rabin_karp(text: &str, pattern: &str) -> Vec<usize> {
    let p = 53;
    let m = 1e9 as usize + 9;
    let mut p_pow: Vec<usize> = vec![0; text.len()];
    p_pow[0] = 1;
    for i in 1..text.len() {
        p_pow[i] = (p_pow[i - 1] * p) % m;
    }
    let mut h_text: Vec<usize> = vec![0; text.len() + 1];
    for (i, character) in text.chars().enumerate() {
        h_text[i + 1] = (h_text[i] + (character as usize - ' ' as usize + 1) * p_pow[i]) % m;
    }
    let h_pattern = pattern.chars().enumerate().fold(0, |h, (i, character)| {
        (h + (character as usize - ' ' as usize + 1) * p_pow[i]) % m
    });
    let mut occurences: Vec<usize> = Vec::new();

    for i in 0..text.len() - pattern.len() + 1 {
        let cur_h = (h_text[i + pattern.len()] + m - h_text[i]) % m;
        if cur_h == h_pattern * p_pow[i] % m {
            occurences.push(i);
        }
    }
    occurences
}
