use std::collections::HashMap;

pub fn has_groups_size_x(deck: Vec<i32>) -> bool {
    let counts = deck.into_iter().fold(HashMap::new(), |mut acc, curr| {
        *acc.entry(curr).or_insert(0) += 1;
        acc
    });

    counts.values().fold(0, |acc, curr| gcd(*curr, acc)) > 1
}

fn gcd(a: i32, b: i32) -> i32 {
    match b > 0 {
        true => gcd(b, a % b),
        false => a,
    }
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::has_groups_size_x;

    #[test]
    fn test_deck_that_has_valid_group() {
        let deck = vec![1, 2, 3, 4, 4, 3, 2, 1];
        let actual = has_groups_size_x(deck);
        assert!(actual);

        let deck = vec![1, 1, 2, 2, 2, 2];
        let actual = has_groups_size_x(deck);
        assert!(actual)
    }

    #[test]
    fn test_deck_that_does_not_have_valid_group() {
        let deck = vec![1, 1, 1, 2, 2, 2, 3, 3];
        let actual = has_groups_size_x(deck);
        assert!(!actual);

        let deck = vec![1, 1, 2, 2, 2, 2, 3];
        let actual = has_groups_size_x(deck);
        assert!(!actual)
    }

    #[test]
    fn test_deck_with_single_value_is_not_valid() {
        let deck = vec![1];
        let actual = has_groups_size_x(deck);
        assert!(!actual)
    }
}
