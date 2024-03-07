fn build_diamond(n: i32) -> Option<String> {
    let get_row = |count: usize| {
        let width = ((n as usize - count) / 2) + count;
        format!("{: >width$}\n", "*".repeat(count), width = width)
    };
    match n {
        n if n % 2 == 0 => None,
        n if n < 0 => None,
        _ => {
            let mut result = get_row(n as usize);
            for count in (1..=n - 2).rev().step_by(2) {
                let row = get_row(count as usize);
                result.insert_str(0, &row);
                result.push_str(&row);
            }

            Some(result)
        }
    }
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use crate::codewars::diamond::build_diamond;

    #[test_case(-1, None)]
    #[test_case(0, None)]
    #[test_case(2, None)]
    #[test_case(1, Some("*\n".to_string()))]
    #[test_case(3, Some(" *\n***\n *\n".to_string()))]
    #[test_case(5, Some("  *\n ***\n*****\n ***\n  *\n".to_string()))]
    fn should_correctly_build_diamond_string(input: i32, expected: Option<String>) {
        let actual = build_diamond(input);
        assert_eq!(actual, expected)
    }
}
