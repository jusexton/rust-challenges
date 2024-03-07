use std::collections::HashMap;

fn count(input: &str) -> HashMap<char, i32> {
    input.chars().fold(HashMap::new(), |mut acc, curr| {
        *acc.entry(curr).or_insert(0) += 1;
        acc
    })
}

#[cfg(test)]
mod tests {
    use crate::codewars::count_characters::count;
    use std::collections::HashMap;

    #[test]
    fn test_count() {
        let actual = count("abcabc");

        let expected = HashMap::from([('a', 2), ('b', 2), ('c', 2)]);
        assert_eq!(actual, expected)
    }
}
