use std::collections::HashSet;

fn doubleton(start: u32) -> u32 {
    let mut current = start + 1;

    loop {
        let characters: HashSet<char> = current.to_string().chars().collect();
        if characters.len() == 2 {
            return current;
        } else {
            current += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use crate::codewars::doubleton::doubleton;

    #[test_case(1, 10)]
    #[test_case(10, 12)]
    #[test_case(120, 121)]
    #[test_case(1234, 1311)]
    fn should_return_the_next_doubleton_number(start: u32, expected: u32) {
        let actual = doubleton(start);
        assert_eq!(expected, actual)
    }
}
