static PRIMES: [u64; 26] = [
    2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97,
    101,
];

fn are_anagrams(a: &str, b: &str) -> bool {
    if a.len() != b.len() {
        return false;
    }

    let a_product = str_to_prime_product(a);
    let b_product = str_to_prime_product(b);
    a_product == b_product
}

fn str_to_prime_product(input: &str) -> u64 {
    input
        .to_ascii_lowercase()
        .chars()
        .map(|c| (c as u8) - b'a')
        .map(|idx| PRIMES[idx as usize])
        .product()
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use crate::other::anagram::are_anagrams;

    #[test_case("ab", "ba")]
    fn should_return_true_when_given_values_that_are_anagrams(a: &str, b: &str) {
        let result = are_anagrams(a, b);
        assert!(result)
    }

    #[test_case("aa", "bb")]
    fn should_return_false_when_given_values_that_are_not_anagrams(a: &str, b: &str) {
        let result = are_anagrams(a, b);
        assert!(!result)
    }
}
