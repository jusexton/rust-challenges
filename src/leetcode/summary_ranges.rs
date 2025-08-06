pub fn summary_ranges(numbers: Vec<i32>) -> Vec<String> {
    let n = numbers.len();
    let mut result = Vec::new();
    let mut i = 0;
    while i < n {
        let left = numbers[i];
        while i < n - 1 && numbers[i] + 1 == numbers[i + 1] {
            i += 1;
        }
        result.push(match left == numbers[i] {
            true => format!("{left}"),
            false => format!("{}->{}", left, numbers[i]),
        });
        i += 1;
    }
    result
}

#[cfg(test)]
mod tests {
    use crate::string_vec;

    use super::*;

    #[test]
    fn produces_ranges_of_consecutive_integers() {
        assert_eq!(
            string_vec!["0->2", "4->5", "7"],
            summary_ranges(vec![0, 1, 2, 4, 5, 7])
        );
        assert_eq!(
            string_vec!["0", "2->4", "6", "8->9"],
            summary_ranges(vec![0, 2, 3, 4, 6, 8, 9])
        )
    }
}
