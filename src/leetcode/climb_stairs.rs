pub fn climb_stairs(n: i32) -> i32 {
    let result = (0..n).fold((1, 0), |(res, prev), _| (res + prev, res));
    result.0
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use crate::leetcode::climb_stairs::climb_stairs;

    #[test_case(0, 1)]
    #[test_case(1, 1)]
    #[test_case(2, 2)]
    #[test_case(3, 3)]
    #[test_case(4, 5)]
    #[test_case(5, 8)]
    #[test_case(6, 13)]
    #[test_case(10, 89)]
    fn should_produce(input: i32, expected: i32) {
        let actual = climb_stairs(input);
        assert_eq!(actual, expected)
    }
}
