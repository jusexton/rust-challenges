// Trivial array and linear summation solution.
// Sum Complexity: O(n)
// Update Complexity: O(1)
//
// struct NumArray {
//     numbers: Vec<i32>,
// }

// impl NumArray {
//     fn new(numbers: Vec<i32>) -> Self {
//         Self { numbers }
//     }

//     fn update(&mut self, index: i32, val: i32) {
//         self.numbers[index as usize] = val;
//     }

//     fn sum_range(&self, left: i32, right: i32) -> i32 {
//         self.numbers[left as usize..=right as usize].iter().sum()
//     }
// }

/// More complex segment tree solution.
/// Sum Complexity: O(log(n))
/// Update Complexity: O(log(n))
struct NumArray {
    tree: Vec<i32>,
    n: usize,
}

impl NumArray {
    pub fn new(arr: Vec<i32>) -> Self {
        let n = arr.len();
        // Calculate the size needed for the tree array
        // Size is 2 * (next power of 2 greater than n) - 1
        let height = (n as f64).log2().ceil() as u32;
        let size = 2 * 2_usize.pow(height);
        let tree = vec![0; size];

        let mut seg_tree = NumArray { tree, n };
        seg_tree.build(&arr, 0, 0, n - 1);
        seg_tree
    }

    fn build(&mut self, arr: &[i32], node: usize, start: usize, end: usize) {
        if start == end {
            self.tree[node] = arr[start];
            return;
        }

        let mid = (start + end) / 2;
        let left = 2 * node + 1;
        let right = 2 * node + 2;

        self.build(arr, left, start, mid);
        self.build(arr, right, mid + 1, end);
        self.tree[node] = self.tree[left] + self.tree[right];
    }

    pub fn update(&mut self, index: i32, value: i32) {
        self.update_tree(0, 0, self.n - 1, index as usize, value)
    }

    fn update_tree(&mut self, node: usize, start: usize, end: usize, idx: usize, val: i32) {
        if start == end {
            self.tree[node] = val;
            return;
        }

        let mid = (start + end) / 2;
        let left = 2 * node + 1;
        let right = 2 * node + 2;

        if idx <= mid {
            self.update_tree(left, start, mid, idx, val);
        } else {
            self.update_tree(right, mid + 1, end, idx, val);
        }

        self.tree[node] = self.tree[left] + self.tree[right];
    }

    pub fn sum_range(&self, left: i32, right: i32) -> i32 {
        self.query_tree(0, 0, self.n - 1, left as usize, right as usize)
    }

    fn query_tree(&self, node: usize, start: usize, end: usize, l: usize, r: usize) -> i32 {
        if r < start || end < l {
            // Range represented by node is completely outside the query range
            return 0;
        }

        if l <= start && end <= r {
            return self.tree[node];
        }

        let mid = (start + end) / 2;
        let left = 2 * node + 1;
        let right = 2 * node + 2;

        let left_sum = self.query_tree(left, start, mid, l, r);
        let right_sum = self.query_tree(right, mid + 1, end, l, r);

        left_sum + right_sum
    }
}

#[cfg(test)]
mod tests {
    use super::NumArray;

    #[test]
    fn updates_value_and_sums_given_range() {
        let mut arr = NumArray::new(vec![1, 3, 5]);
        assert_eq!(9, arr.sum_range(0, 2));

        arr.update(1, 2);
        assert_eq!(8, arr.sum_range(0, 2));
    }
}
