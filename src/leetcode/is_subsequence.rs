pub fn is_subsequence(s: String, t: String) -> bool {
    if s.len() > t.len() || s.len() == 0 {
        return false;
    }

    let (mut s_idx, mut t_idx) = (0, 0);
    let s = s.as_bytes();
    let t = t.as_bytes();

    while t_idx < t.len() {
        if s[s_idx] == t[t_idx] {
            s_idx += 1;
            if s_idx == s.len() {
                return true;
            }
        }
        t_idx += 1;
    }

    false
}
