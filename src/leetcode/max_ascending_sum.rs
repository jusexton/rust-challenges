pub fn max_ascending_sum(numbers: Vec<i32>) -> i32 {
    let mut max_sum = i32::MIN;
    let mut current_sum = numbers[0];
    for i in 1..numbers.len() {
        match numbers[i - 1] < numbers[i] {
            true => current_sum += numbers[i],
            false => {
                max_sum = max_sum.max(current_sum);
                current_sum = numbers[i];
            }
        }
    }
    max_sum.max(current_sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_ascending_subarray() {
        assert_eq!(10, max_ascending_sum(vec![10, 1, 2, 3, 4, 0]))
    }

    #[test]
    fn many_ascending_subarrays() {
        assert_eq!(65, max_ascending_sum(vec![10, 20, 30, 5, 10, 50]));
    }
}
