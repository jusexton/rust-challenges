use regex::Regex;

fn valid_word_abbreviation(word: String, abbr: String) -> bool {
    let re = Regex::new("[0-9]+|[a-z]").unwrap();
    let mut idx = 0;
    for m in re.find_iter(&abbr) {
        let match_str = m.as_str();
        if match_str.starts_with("0") {
            return false;
        }

        match match_str.parse::<usize>() {
            Ok(skip) => idx += skip,
            Err(_) => match idx < word.len() && match_str == &word[idx..idx + 1] {
                true => idx += 1,
                false => return false,
            },
        }
    }

    idx == word.len()
}

#[cfg(test)]
mod tests {
    use crate::leetcode::valid_word_abbr::valid_word_abbreviation;

    #[test]
    fn should_identify_valid_word_abbreviation() {
        assert!(valid_word_abbreviation(
            "internationalization".to_string(),
            "i12iz4n".to_string()
        ));
    }

    #[test]
    fn should_identify_invalid_word_abbreviation() {
        assert!(!valid_word_abbreviation(
            "apple".to_string(),
            "a2e".to_string()
        ));

        assert!(!valid_word_abbreviation(
            "abbde".to_string(),
            "a1b01e".to_string()
        ));

        assert!(!valid_word_abbreviation("hi".to_string(), "2i".to_string()));

        assert!(!valid_word_abbreviation("a".to_string(), "2".to_string()));
    }
}
