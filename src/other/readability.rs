const SENTENCE_ENDINGS: &[char] = &['!', '.', '?'];

struct StringStats {
    character_count: i32,
    word_count: i32,
    sentence_count: i32,
}

impl StringStats {
    fn from(s: &str) -> Self {
        let mut character_count = 0;
        let mut word_count = 0;
        let mut sentence_count = 0;

        for index in 0..s.len() {
            let char = s.chars().nth(index).unwrap();

            if char.is_whitespace() {
                let previous_char = s.chars().nth(index - 1).unwrap();
                if !SENTENCE_ENDINGS.contains(&previous_char) {
                    word_count += 1;
                }
            } else if char.is_alphabetic() {
                character_count += 1;
            } else if SENTENCE_ENDINGS.contains(&char) {
                sentence_count += 1;
                word_count += 1;
            }
        }

        StringStats {
            character_count,
            word_count,
            sentence_count,
        }
    }

    fn average_letters(&self) -> f64 {
        (self.character_count * 100) as f64 / self.word_count as f64
    }

    fn average_sentences(&self) -> f64 {
        (self.sentence_count * 100) as f64 / self.word_count as f64
    }
}

fn readability(s: &str) -> i32 {
    let stats = StringStats::from(s);
    coleman_liau_index(stats.average_letters(), stats.average_sentences())
}

fn coleman_liau_index(average_letters: f64, average_sentences: f64) -> i32 {
    let index = (0.0588 * average_letters) - (0.296 * average_sentences) - 15.8;
    index.round() as i32
}

#[cfg(test)]
mod tests {
    #![cfg(test)]
    extern crate test_case;

    use test_case::test_case;

    use crate::other::readability::readability;

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
    fn should_return_correct_grade(s: &str, grade: i32) {
        let result = readability(s);
        assert_eq!(result, grade)
    }
}
