use std::collections::HashMap;

pub fn find_repeated_dna_sequences(s: String) -> Vec<String> {
    let occurrences = s
        .as_bytes()
        .windows(10)
        .fold(HashMap::new(), |mut acc, curr| {
            let sequence = std::str::from_utf8(curr).unwrap();
            *acc.entry(sequence).or_insert(0) += 1;
            acc
        });
    occurrences
        .iter()
        .filter_map(|(key, value)| {
            if *value > 1 {
                Some(key.to_string())
            } else {
                None
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use super::find_repeated_dna_sequences;

    #[test_case("AAAAACCCCCAAAAACCCCCCAAAAAGGGTTT", &["CCCCCAAAAA","AAAAACCCCC"])]
    #[test_case("AAAAAAAAAAAAA", &["AAAAAAAAAA"])]
    fn test_find_repeated_dna_sequences(input: &str, expected: &[&str]) {
        let mut expected = to_string_vec(expected);
        expected.sort_unstable();

        let mut result = find_repeated_dna_sequences(input.to_string());
        result.sort_unstable();

        assert_eq!(result, expected)
    }

    fn to_string_vec(slices: &[&str]) -> Vec<String> {
        slices.iter().map(|it| String::from(*it)).collect()
    }
}
