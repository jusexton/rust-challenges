fn returns_to_origin(moves: String) -> bool {
    let mut x = 0;
    let mut y = 0;

    for movement in moves.chars() {
        match movement {
            'U' => y += 1,
            'D' => y -= 1,
            'L' => x -= 1,
            'R' => x += 1,
            _ => {}
        }
    }

    return x == 0 && y == 0;
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use crate::leetcode::return_to_origin::returns_to_origin;

    #[test_case("", true)]
    #[test_case("UDLR", true)]
    #[test_case("ULDR", true)]
    #[test_case("UU", false)]
    #[test_case("UDLRL", false)]
    fn test_returns_to_origin(moves: &str, expected: bool) {
        let result = returns_to_origin(moves.to_string());
        assert_eq!(expected, result);
    }
}
