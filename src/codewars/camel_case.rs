fn camel_case(str: &str) -> String {
    str.split(['-', '_'])
        .enumerate()
        .map(|(index, s)| {
            if index == 0 {
                String::from(s)
            } else {
                [&s[..1].to_uppercase(), &s[1..]].join("")
            }
        })
        .collect()
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
