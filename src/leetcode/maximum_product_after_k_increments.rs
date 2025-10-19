use std::{cmp::Reverse, collections::BinaryHeap};

const MODULO: u64 = 10u64.pow(9) + 7;

pub fn maximum_product(numbers: Vec<i32>, k: i32) -> i32 {
    let mut heap: BinaryHeap<_> = numbers.into_iter().map(Reverse).collect();
    for _ in 0..k {
        let v = heap.pop().unwrap();
        heap.push(Reverse(v.0 + 1));
    }
    heap.iter()
        .fold(1u64, |acc, curr| (acc * curr.0 as u64) % MODULO) as i32
}

#[cfg(test)]
mod tests {
    use super::maximum_product;

    #[test]
    fn produces_maximum_product() {
        assert_eq!(20, maximum_product(vec![0, 4], 5));
        assert_eq!(216, maximum_product(vec![6, 3, 3, 2], 2));
        assert_eq!(180820950, maximum_product(vec![24, 5, 64, 53, 26, 38], 54))
    }
}
