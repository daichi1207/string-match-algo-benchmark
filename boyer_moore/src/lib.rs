pub fn boyer_moore(haystack: &str, needle: &str) -> Vec<usize> {
    let mut last_occurrence: [Option<usize>; 256] = [None; 256];
    let h_chars: Vec<char> = haystack.chars().collect();
    let n_chars: Vec<char> = needle.chars().collect();
    let h_len = h_chars.len();
    let n_len = n_chars.len();

    for (i, &c) in n_chars.iter().enumerate() {
        last_occurrence[c as usize] = Some(i);
    }

    let mut positions = vec![];
    let mut i = 0;

    while i <= h_len - n_len {
        let mut j = n_len;
        while j > 0 {
            j -= 1;
            if h_chars[i + j] != n_chars[j] {
                break;
            }
        }
        if j > 0 {
            let lo = last_occurrence[h_chars[i + j] as usize];
            if let Some(k) = lo {
                if k < j {
                    i += j - k;
                } else {
                    i += 1;
                }
            } else {
                i += j + 1;
            }
        } else {
            positions.push(i);
            i += 1;
        }
    }
    positions
}
