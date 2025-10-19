pub fn num_rescue_boats(mut people: Vec<i32>, limit: i32) -> i32 {
    people.sort_unstable();

    let (mut left, mut right) = (0, people.len());
    let mut result = 0;
    while left < right {
        if people[left] + people[right - 1] <= limit {
            left += 1;
            right -= 1;
        } else {
            right -= 1;
        }
        result += 1;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::num_rescue_boats;

    #[test]
    fn returns_zero_when_people_is_empty() {
        assert_eq!(0, num_rescue_boats(vec![], 5))
    }

    #[test]
    fn correct_number_of_boats_used() {
        assert_eq!(1, num_rescue_boats(vec![1, 2], 3));
        assert_eq!(3, num_rescue_boats(vec![3, 2, 2, 1], 3));
        assert_eq!(4, num_rescue_boats(vec![3, 5, 3, 4], 5))
    }
}
