/// https://www.codewars.com/kata/5b752a42b11814b09c00005d
fn recursion(a: usize, b: usize) -> (usize, usize) {
    match (a, b) {
        c if c.0 == 0 || c.1 == 0 => c,
        c if c.0 >= 2 * c.1 => {
            let updated_a = c.0 - (2 * c.1);
            return recursion(updated_a, c.1);
        }
        c if c.1 >= 2 * c.0 => {
            let updated_b = c.1 - (2 * c.0);
            return recursion(c.0, updated_b);
        }
        _ => (a, b),
    }
}

#[cfg(test)]
mod tests {
    #![cfg(test)]
    extern crate test_case;

    use test_case::test_case;

    use crate::codewars::recursion::recursion;

    #[test_case((6, 19), (6, 7))]
    #[test_case((2, 1), (0, 1))]
    #[test_case((22, 5), (0, 1))]
    #[test_case((2, 10), (2, 2))]
    #[test_case((8796203, 7556), (1019, 1442))]
    #[test_case((19394, 19394), (19394, 19394))]
    fn test_recursion(input: (usize, usize), expected: (usize, usize)) {
        assert_eq!(expected, recursion(input.0, input.1))
    }
}
