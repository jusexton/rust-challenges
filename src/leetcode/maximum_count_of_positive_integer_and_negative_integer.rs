use std::cmp::Ordering;

pub fn maximum_count(numbers: Vec<i32>) -> i32 {
    let counts = numbers.into_iter().fold((0, 0), |mut acc, curr| {
        match curr.cmp(&0) {
            Ordering::Greater => acc.1 += 1,
            Ordering::Less => acc.0 += 1,
            Ordering::Equal => {}
        }
        acc
    });
    counts.0.max(counts.1)
}

#[cfg(test)]
mod tests {
    use super::maximum_count;

    #[test]
    fn maximum_positive_negative_count_is_provided() {
        let numbers = vec![-1, -2, 0, 2, 5, -3];
        let actual = maximum_count(numbers);
        assert_eq!(3, actual)
    }
}
