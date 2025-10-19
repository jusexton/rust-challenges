use std::{cmp::Reverse, collections::BinaryHeap};

// TODO: Revisit this problem with a more optimized solution.
// Solution uses a brute force generation of pairs and did not pass time limit requirements.
// Posting here for historic purposes.
pub fn smallest_distance_pair(numbers: Vec<i32>, mut k: i32) -> i32 {
    let mut abs_differences = BinaryHeap::new();
    for (idx, &a) in numbers.iter().enumerate() {
        for &b in numbers.iter().skip(idx + 1) {
            abs_differences.push(Reverse(a.abs_diff(b) as i32));
        }
    }

    while k >= 2 {
        abs_differences.pop();
        k -= 1;
    }

    abs_differences.pop().unwrap().0
}

#[cfg(test)]
mod tests {
    use super::smallest_distance_pair;

    #[test]
    fn returns_smallest_distance_pair() {
        assert_eq!(0, smallest_distance_pair(vec![1, 3, 1], 1));
        assert_eq!(5, smallest_distance_pair(vec![1, 6, 1], 3));
    }
}
