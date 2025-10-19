// Array and binary search solution.
// struct KthLargest {
//     k: i32,
//     values: Vec<i32>,
// }

// impl KthLargest {
//     fn new(k: i32, mut values: Vec<i32>) -> Self {
//         values.sort_unstable();
//         Self { k, values }
//     }

//     fn add(&mut self, val: i32) -> i32 {
//         let pos = self.values.binary_search(&val).unwrap_or_else(|e| e);
//         self.values.insert(pos, val);

//         let k_pos = self.values.len() - self.k as usize;
//         self.values[k_pos]
//     }
// }

use std::collections::BinaryHeap;

struct KthLargest {
    heap: BinaryHeap<std::cmp::Reverse<i32>>,
    k: usize,
}

impl KthLargest {
    fn new(k: i32, values: Vec<i32>) -> Self {
        let k = k as usize;
        let mut heap: BinaryHeap<_> = values.into_iter().map(std::cmp::Reverse).collect();
        for _ in k..heap.len() {
            heap.pop();
        }
        Self { heap, k }
    }

    fn add(&mut self, val: i32) -> i32 {
        self.heap.push(std::cmp::Reverse(val));
        if self.heap.len() > self.k {
            self.heap.pop();
        }
        self.heap.peek().unwrap().0
    }
}

#[cfg(test)]
mod tests {
    use super::KthLargest;

    #[test]
    fn kth_largest() {
        let values = vec![1, 2, 3];
        let mut largest = KthLargest::new(1, values);

        assert_eq!(3, largest.add(2));
        assert_eq!(4, largest.add(4));
        assert_eq!(4, largest.add(3));
    }
}
