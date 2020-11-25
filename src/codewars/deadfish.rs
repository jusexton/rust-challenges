fn parse(code: &str) -> Vec<i32> {
    let mut current_value = 0i32;
    let mut output: Vec<i32> = Vec::new();

    for character in code.chars() {
        match character {
            'i' => current_value += 1,
            'd' => current_value -= 1,
            's' => current_value = current_value.pow(2),
            'o' => output.push(current_value),
            _ => {}
        }
    }

    return output;
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use crate::codewars::deadfish::parse;

    #[test_case("iiisdoso", &[8, 64])]
    #[test_case("iiisdosodddddiso", &[8, 64, 3600])]
    fn should_correctly_parse_the_dead_fish_string(input: &str, expected: &[i32]) {
        let actual = parse(input);
        assert_eq!(actual, expected)
    }
}
