use std::collections::HashSet;

pub fn is_fascinating(n: i32) -> bool {
    let s = format!("{}{}{}", n, n * 2, n * 3);
    let digits = s.chars().collect::<HashSet<_>>();
    s.len() == 9 && digits.len() == 9 && !digits.contains(&'0')
}

#[cfg(test)]
mod tests {
    use crate::leetcode::is_fascinating::is_fascinating;

    #[test]
    fn fascinating() {
        assert!(is_fascinating(192));
    }

    #[test]
    fn not_fascinating() {
        assert!(!is_fascinating(100));
        assert!(!is_fascinating(783));
    }
}
