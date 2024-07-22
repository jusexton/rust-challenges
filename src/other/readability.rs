fn coleman_liau_index(average_letters: f64, average_sentences: f64) -> usize {
    let index = (0.0588 * average_letters) - (0.296 * average_sentences) - 15.8;
    index.round() as usize
}

fn is_sentence_end(c: char) -> bool {
    c == '!' || c == '.' || c == '?'
}

#[derive(Debug)]
pub struct ColemanLiauIndex(usize);

impl From<&str> for ColemanLiauIndex {
    fn from(value: &str) -> Self {
        let mut character_count = 1;
        let mut word_count = 0;
        let mut sentence_count = 0;

        let mut previous = value.chars().nth(0).unwrap();
        for char in value.chars() {
            if char.is_whitespace() && !is_sentence_end(previous) {
                word_count += 1;
            } else if char.is_alphabetic() {
                character_count += 1;
            } else if is_sentence_end(char) {
                sentence_count += 1;
                word_count += 1;
            }
            previous = char
        }

        let average_letters = (character_count * 100) as f64 / word_count as f64;
        let average_sentences = (sentence_count * 100) as f64 / word_count as f64;
        let index = coleman_liau_index(average_letters, average_sentences);
        Self(index)
    }
}

impl PartialEq<usize> for ColemanLiauIndex {
    fn eq(&self, other: &usize) -> bool {
        self.0 == *other
    }
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use super::*;

    #[test_case(
        "Congratulations! Today is your day. You're off to Great Places! You're off and away!",
        3
    )]
    #[test_case(
        "Harry Potter was a highly unusual boy in many ways. For one thing, he hated the summer holidays more than any other time of year. For another, he really wanted to do his homework, but was forced to do it in secret, in the dead of the night. And he also happened to be a wizard.",
        5
    )]
    #[test_case(
        "As the average number of letters and words per sentence increases, the Coleman-Liau index gives the text a higher reading level. If you were to take this paragraph, for instance, which has longer words and sentences than either of the prior two examples, the formula would give the text an eleventh grade reading level.",
        11
    )]
    fn should_return_correct_grade(s: &str, grade: usize) {
        let index = ColemanLiauIndex::from(s);
        assert_eq!(index, grade)
    }
}
