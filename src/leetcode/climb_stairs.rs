pub fn climb_stairs(n: i32) -> i32 {
    if n == 0 || n == 1 {
        return 1;
    }

    let mut curr = 1;
    let mut prev = 1;
    for _ in 2..=n {
        let temp = curr;
        curr = curr + prev;
        prev = temp;
    }

    curr
}

#[cfg(test)]
mod tests {
    use test_case::test_case;
    use crate::leetcode::climb_stairs::climb_stairs;

    #[test_case(0, 1)]
    #[test_case(1, 1)]
    #[test_case(2, 2)]
    #[test_case(3, 3)]
    #[test_case(10, 89)]
    fn should_produce(input: i32, expected: i32) {
        let actual = climb_stairs(input);
        assert_eq!(actual, expected)
    }
}