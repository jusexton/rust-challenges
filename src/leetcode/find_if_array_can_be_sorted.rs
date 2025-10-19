pub fn can_sort_array(numbers: Vec<i32>) -> bool {
    let (mut prev_max, mut new_max) = (0, numbers[0]);
    let mut bit_count = numbers[0].count_ones();
    for &n in &numbers[1..] {
        if n < prev_max {
            return false;
        }
        if bit_count == n.count_ones() {
            new_max = new_max.max(n);
        } else {
            prev_max = new_max;
            if n < prev_max {
                return false;
            }
            bit_count = n.count_ones();
            new_max = n;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::can_sort_array;

    #[test]
    fn single_value() {
        assert!(can_sort_array(vec![8]))
    }

    #[test]
    fn sortable() {
        assert!(can_sort_array(vec![8, 4, 2, 30, 15]));
        assert!(can_sort_array(vec![1, 2, 3, 4, 5]));
    }

    #[test]
    fn not_sortable() {
        assert!(!can_sort_array(vec![3, 16, 8, 4, 2]))
    }
}
