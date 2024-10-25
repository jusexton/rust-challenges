fn sum_dig_pow(a: u64, b: u64) -> Vec<u64> {
    (a..=b).filter(|n| is_eureka(*n)).collect()
}

fn is_eureka(number: u64) -> bool {
    let sum = digits(number)
        .into_iter()
        .rev()
        .enumerate()
        .fold(0, |acc, (idx, curr)| acc + curr.pow((idx as u32) + 1));
    sum == number
}

fn digits(mut number: u64) -> Vec<u64> {
    let mut digits = vec![];
    while number > 0 {
        digits.push(number % 10);
        number /= 10;
    }
    digits
}

#[cfg(test)]
mod tests {
    use super::sum_dig_pow;

    #[test]
    fn generate_correct_values() {
        assert_eq!(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 89], sum_dig_pow(1, 100))
    }

    fn range_is_inclusive() {
        assert_eq!(vec![89], sum_dig_pow(10, 89))
    }
}
