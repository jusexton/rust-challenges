use std::collections::HashMap;

fn sum_digits(mut n: i32) -> i32 {
    let mut sum = 0;
    while n > 0 {
        sum += n % 10;
        n /= 10;
    }
    sum
}

pub fn count_largest_group(n: i32) -> i32 {
    let groups = (1..=n).fold(HashMap::new(), |mut acc, curr| {
        *acc.entry(sum_digits(curr)).or_insert(0) += 1;
        acc
    });
    let max = *groups.values().max().unwrap();
    groups.into_values().filter(|v| *v == max).count() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_largest_group_by_digit_sum() {
        assert_eq!(4, count_largest_group(13));
        assert_eq!(2, count_largest_group(2));
    }
}
