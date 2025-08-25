pub fn valid_word_square(words: Vec<String>) -> bool {
    let words: Vec<_> = words.into_iter().map(|s| s.into_bytes()).collect();
    let n = words.len();
    for i in 0..n {
        for j in 0..words[i].len() {
            if j >= n || words[j].len() <= i || words[j][i] != words[i][j] {
                return false;
            }
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use crate::{leetcode::valid_word_square::valid_word_square, string_vec};

    #[test]
    fn valid_square() {
        let words = string_vec!("abcd", "bnrt", "crmy", "dtye");
        assert!(valid_word_square(words));
    }

    #[test]
    fn valid_off_shape() {
        let words = string_vec!("abcd", "bnrt", "crm", "dt");
        assert!(valid_word_square(words));
    }

    #[test]
    fn invalid_square_incorrect_characters() {
        let words = string_vec!("ball", "area", "read", "lady");
        assert!(!valid_word_square(words));

        let words = string_vec!("c", "de");
        assert!(!valid_word_square(words));
    }
}
