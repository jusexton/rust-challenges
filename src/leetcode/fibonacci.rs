/// https://leetcode.com/problems/fibonacci-number
fn fibonacci(input: u32) -> u32 {
    match input {
        0 => 0,
        1 => 1,
        n => fibonacci(n - 2) + fibonacci(n - 1),
    }
}

#[cfg(test)]
mod tests {
    #![cfg(test)]
    extern crate test_case;

    use test_case::test_case;

    use crate::leetcode::fibonacci::fibonacci;

    #[test_case(0, 0)]
    #[test_case(1, 1)]
    #[test_case(2, 1)]
    #[test_case(3, 2)]
    #[test_case(4, 3)]
    fn fibonacci_tests(input: u32, expected: u32) {
        assert_eq!(expected, fibonacci(input))
    }
}
