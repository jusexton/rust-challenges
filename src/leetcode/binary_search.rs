// pub fn search(numbers: Vec<i32>, target: i32) -> i32 {
//     match numbers.binary_search(&target) {
//         Ok(i) => i as i32,
//         Err(_) => -1,
//     }
// }

use std::cmp::Ordering;

pub fn search(numbers: Vec<i32>, target: i32) -> i32 {
    let (mut left, mut right) = (0, numbers.len());
    while left < right {
        let mid = left + (right - left) / 2;
        match numbers[mid].cmp(&target) {
            Ordering::Equal => return mid as i32,
            Ordering::Less => left = mid + 1,
            Ordering::Greater => right = mid,
        }
    }
    -1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_element_index_when_it_exists() {
        assert_eq!(4, search(vec![-1, 0, 3, 5, 9, 12], 9));
        assert_eq!(0, search(vec![5], 5));
    }

    #[test]
    fn negative_one_when_element_does_not_exist() {
        assert_eq!(-1, search(vec![-1, 0, 3, 5, 9, 12], 20));
        assert_eq!(-1, search(vec![-1, 0, 3, 5, 9, 12], 2));
        assert_eq!(-1, search(vec![5], -5))
    }
}
