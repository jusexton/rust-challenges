pub fn the_maximum_achievable_x(num: i32, t: i32) -> i32 {
    num + t * 2
}

#[cfg(test)]
mod tests {
    use super::the_maximum_achievable_x;
    use test_case::test_case;

    #[test_case(4, 1, 6)]
    #[test_case(3, 2, 7)]
    fn test_max_achievable_number(num: i32, t: i32, expected: i32) {
        assert_eq!(the_maximum_achievable_x(num, t), expected);
    }
}
