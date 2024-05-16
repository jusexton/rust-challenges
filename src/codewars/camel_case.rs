fn camel_case(str: &str) -> String {
    let mut words = str.split(&['-', '_']);
    let mut camel_cased = String::with_capacity(str.len());

    if let Some(first_word) = words.next() {
        camel_cased.push_str(first_word);
    }

    for word in words {
        let mut chars = word.chars();
        if let Some(first_char) = chars.next() {
            camel_cased.push(first_char.to_ascii_uppercase());
            camel_cased.extend(chars);
        }
    }

    camel_cased
}

#[cfg(test)]
mod tests {
    use crate::codewars::camel_case::camel_case;

    #[test]
    fn should_correctly_convert_string_to_camel_case() {
        let input = "";
        let expected = "";
        let actual = camel_case(input);
        assert_eq!(actual, expected);

        let input = "string";
        let expected = "string";
        let actual = camel_case(input);
        assert_eq!(actual, expected);

        let input = "test-string-test";
        let expected = "testStringTest";
        let actual = camel_case(input);
        assert_eq!(actual, expected);

        let input = "test_string";
        let expected = "testString";
        let actual = camel_case(input);
        assert_eq!(actual, expected);

        let input = "Test_String";
        let expected = "TestString";
        let actual = camel_case(input);
        assert_eq!(actual, expected);
    }
}
