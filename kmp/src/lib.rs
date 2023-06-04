pub fn knuth_morris_pratt(haystack: &str, needle: &str) -> Vec<usize> {
    let h_chars: Vec<char> = haystack.chars().collect();
    let n_chars: Vec<char> = needle.chars().collect();
    let h_len = h_chars.len();
    let n_len = n_chars.len();

    let mut lps = vec![0; n_len];
    let mut len: usize = 0;
    let mut i = 1;

    while i < n_len {
        if n_chars[i] == n_chars[len] {
            len += 1;
            lps[i] = len;
            i += 1;
        } else {
            if len != 0 {
                len = lps[len - 1];
            } else {
                lps[i] = 0;
                i += 1;
            }
        }
    }

    let mut positions = vec![];
    let (mut i, mut j) = (0, 0);
    while i < h_len {
        if n_chars[j] == h_chars[i] {
            i += 1;
            j += 1;
        }
        if j == n_len {
            positions.push(i - j);
            j = lps[j - 1];
        } else if i < h_len && n_chars[j] != h_chars[i] {
            if j != 0 {
                j = lps[j - 1];
            } else {
                i += 1;
            }
        }
    }
    positions
}
