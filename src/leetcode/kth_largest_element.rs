use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub fn find_kth_largest(numbers: Vec<i32>, k: i32) -> i32 {
    let k = k as usize;
    let mut heap: BinaryHeap<_> = numbers[..k].iter().map(Reverse).collect();
    for number in &numbers[k..] {
        heap.push(Reverse(number));
        heap.pop();
    }
    *heap.peek().unwrap().0
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use crate::leetcode::kth_largest_element::find_kth_largest;

    #[test_case(&[1, 2, 3, 4], 2, 3)]
    #[test_case(&[1, 1, 3, 4, 5, 6], 5, 1)]
    #[test_case(&[1, 2, 3], 1, 3)]
    #[test_case(&[15, 5, 1, 3, 4, 5, 6, 9, 11], 2, 11)]
    fn should_return_kth_largest_element(numbers: &[i32], k: i32, expected: i32) {
        let result = find_kth_largest(numbers.to_vec(), k);
        assert_eq!(result, expected);
    }
}
