use std::{cmp::Reverse, collections::BinaryHeap};

pub fn get_final_state(mut numbers: Vec<i32>, k: i32, multiplier: i32) -> Vec<i32> {
    let mut heap: BinaryHeap<_> = numbers.iter_mut().zip(0..).map(Reverse).collect();
    for _ in 0..k {
        *heap.peek_mut().unwrap().0 .0 *= multiplier;
    }
    numbers
}

#[cfg(test)]
mod tests {
    use super::get_final_state;

    #[test]
    fn calculates_final_state_after_k_operations_of_multiplication() {
        assert_eq!(
            vec![8, 4, 6, 5, 6],
            get_final_state(vec![2, 1, 3, 5, 6], 5, 2)
        );

        assert_eq!(vec![16, 8], get_final_state(vec![1, 2], 3, 4));

        assert_eq!(vec![27, 9, 15], get_final_state(vec![1, 3, 5], 5, 3))
    }
}
