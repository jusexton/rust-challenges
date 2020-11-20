fn pascal_case(str: &str) -> String {
    return str.split_whitespace()
        .map(|word| first_letter_to_uppercase(word))
        .collect()
}

fn first_letter_to_uppercase(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

// Refactored version
// fn pascal_case(str: &str) -> String {
//     str.split_whitespace()
//         .map(|s| [&s[..1].to_uppercase(), &s[1..]].join(""))
//         .collect()
// }

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use crate::codewars::casing::first_letter_to_uppercase;
    use crate::codewars::casing::pascal_case;

    #[test_case("s", "S")]
    #[test_case("string", "String")]
    fn should_correctly_return_the_given_string_with_first_letter_uppercase(input: &str, expected: &str) {
        let actual = first_letter_to_uppercase(input);
        assert_eq!(actual, expected);
    }

    #[test_case("string", "String")]
    #[test_case("test string", "TestString")]
    fn should_correctly_convert_string_to_pascal_case(input: &str, expected: &str) {
        let actual = pascal_case(input);
        assert_eq!(actual, expected)
    }
}