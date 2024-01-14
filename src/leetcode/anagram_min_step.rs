pub fn min_steps(s: String, t: String) -> i32 {
    let mut f = [0i32; 128];
    s.chars().for_each(|c| f[c as usize] += 1);
    t.chars().for_each(|c| f[c as usize] -= 1);
    f.iter().map(|x| x.abs()).sum::<i32>() / 2
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use crate::leetcode::anagram_min_step::min_steps;

    #[test_case("abb", "aab", 1)]
    #[test_case("leetcode", "practice", 5)]
    #[test_case("anagram", "mangaar", 0)]
    fn test_min_steps(s: &str, t: &str, expected: i32) {
        let steps = min_steps(s.to_string(), t.to_string());
        assert_eq!(steps, expected);
    }
}
