// pub fn search_insert(numbers: Vec<i32>, target: i32) -> i32 {
//     let idx = match numbers.binary_search(&target) {
//         Ok(i) => i,
//         Err(i) => i,
//     };
//     idx as i32
// }

use std::cmp::Ordering;

pub fn search_insert(numbers: Vec<i32>, target: i32) -> i32 {
    let (mut left, mut right) = (0, numbers.len());
    while left < right {
        let mid = left + (right - left) / 2;
        match numbers[mid].cmp(&target) {
            Ordering::Less => left = mid + 1,
            Ordering::Equal => return mid as i32,
            Ordering::Greater => right = mid,
        }
    }
    (left + (right - left) / 2) as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_index_when_element_exists() {
        assert_eq!(1, search_insert(vec![1, 4, 7, 10], 4))
    }

    #[test]
    fn returns_sorted_index_when_element_does_not_exist() {
        assert_eq!(0, search_insert(vec![10, 15, 17], 2))
    }
}
