pub fn remove_anagrams(words: Vec<String>) -> Vec<String> {
    let mut result = vec![];
    let mut prev_count = [0; 26];
    for word in words {
        let curr_count = word.bytes().fold([0; 26], |mut acc, curr| {
            acc[(curr - b'a') as usize] += 1;
            acc
        });

        if prev_count != curr_count {
            result.push(word);
        }

        prev_count = curr_count;
    }
    result
}

#[cfg(test)]
mod tests {
    use crate::{leetcode::removing_anagrams::remove_anagrams, string_vec};

    #[test]
    fn correctly_removes_adjacent_anagrams() {
        let words = string_vec!["abba", "baba", "bbaa", "cd", "cd"];
        assert_eq!(string_vec!["abba", "cd"], remove_anagrams(words));
    }
}
