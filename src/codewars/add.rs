fn add(a: i32, b: i32) -> i32 {
    if b == 0 {
        return a;
    }

    return add(a ^ b, (a & b) << 1);
}

#[cfg(test)]
mod tests {
    #![cfg(test)]
    extern crate test_case;

    use test_case::test_case;

    use crate::codewars::add::add;

    #[test_case(0, 0, 0)]
    #[test_case(5, 0, 5)]
    #[test_case(0, 5, 5)]
    #[test_case(5, -5, 0)]
    fn should_correctly_add_two_numbers_together(number_one: i32, number_two: i32, expected: i32) {
        let actual = add(number_one, number_two);
        assert_eq!(expected, actual);
    }
}
