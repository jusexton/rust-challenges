pub fn is_subsequence(s: String, t: String) -> bool {
    let (mut i, mut j) = (0, 0);
    let s = s.as_bytes();
    let t = t.as_bytes();
    while i < s.len() && j < t.len() {
        if s[i] == t[j] {
            i += 1;
        }
        j += 1;
    }
    i == s.len()
}
