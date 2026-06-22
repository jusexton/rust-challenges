pub fn max_number_of_balloons(text: String) -> i32 {
    let freq = text.chars().fold([0i32; 26], |mut acc, c| {
        acc[c as usize - 'a' as usize] += 1;
        acc
    });

    let idx = |c: char| c as usize - 'a' as usize;
    [
        freq[idx('b')],
        freq[idx('a')],
        freq[idx('l')] / 2,
        freq[idx('o')] / 2,
        freq[idx('n')],
    ]
    .into_iter()
    .min()
    .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_number_of_balloons() {
        assert_eq!(max_number_of_balloons("nlaebolko".to_string()), 1);
        assert_eq!(max_number_of_balloons("loonbalxballpoon".to_string()), 2);
        assert_eq!(max_number_of_balloons("leetcode".to_string()), 0);
    }
}
