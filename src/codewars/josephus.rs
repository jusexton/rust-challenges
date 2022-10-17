fn josephus_survivor(person_count: i32, skip: i32) -> i32 {
    (1..person_count + 1).fold(0, |acc, cur| (acc + skip) % cur) + 1
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use crate::codewars::josephus::josephus_survivor;

    #[test_case(7, 3, 4)]
    #[test_case(11, 19, 10)]
    #[test_case(1, 300, 1)]
    #[test_case(2, 300, 1)]
    #[test_case(7, 300, 7)]
    #[test_case(300, 300, 265)]
    fn test_josephus_survivor(n: i32, k: i32, expected: i32) {
        let actual = josephus_survivor(n, k);
        assert_eq!(actual, expected);
    }
}
