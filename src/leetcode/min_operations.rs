pub fn minimum_operations(numbers: Vec<i32>) -> i32 {
    let n = numbers.len();
    let mut freq = [0; 101];
    for i in (0..n).rev() {
        let idx = numbers[i] as usize;
        freq[idx] += 1;
        if freq[idx] > 1 {
            return div_ceil(i as i32 + 1, 3);
        }
    }
    0
}

fn div_ceil(a: i32, b: i32) -> i32 {
    (a + b - 1) / b
}

#[cfg(test)]
mod tests {
    use crate::leetcode::min_operations::minimum_operations;

    #[test]
    fn simple() {
        assert_eq!(2, minimum_operations(vec![1, 2, 3, 4, 2, 3, 3, 5, 7]))
    }

    #[test]
    fn remove_all() {
        assert_eq!(2, minimum_operations(vec![4, 5, 6, 4, 4]))
    }

    #[test]
    fn remove_none() {
        assert_eq!(0, minimum_operations(vec![6, 7, 8, 9]))
    }
}
