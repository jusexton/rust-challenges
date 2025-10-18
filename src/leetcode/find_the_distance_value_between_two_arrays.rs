pub fn find_the_distance_value(a: Vec<i32>, b: Vec<i32>, d: i32) -> i32 {
    a.iter()
        .filter(|&x| b.iter().all(|y| (x - y).abs() > d))
        .count() as i32
}

#[cfg(test)]
mod tests {
    use super::find_the_distance_value;

    #[test]
    fn calculates_distance_value() {
        assert_eq!(
            2,
            find_the_distance_value(vec![4, 5, 8], vec![10, 9, 1, 8], 2)
        )
    }
}
