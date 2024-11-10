pub fn minimum_subarray_length(numbers: Vec<i32>, k: i32) -> i32 {
    let mut res = i32::MAX;
    let mut bit_count = [0u32; 32];
    let mut left = 0;

    for right in 0..numbers.len() {
        add_bits(&mut bit_count, numbers[right]);

        while bit_sum(&bit_count) >= k && left <= right {
            res = res.min((right - left + 1) as i32);
            subtract_bits(&mut bit_count, numbers[left]);
            left += 1;
        }
    }

    match res {
        i32::MAX => -1,
        _ => res,
    }
}

fn add_bits(bit_count: &mut [u32; 32], mut number: i32) {
    let mut idx = 0;
    while number > 0 {
        bit_count[idx] += (number & 1) as u32;
        number >>= 1;
        idx += 1;
    }
}

fn subtract_bits(bit_count: &mut [u32; 32], mut number: i32) {
    let mut idx = 0;
    while number > 0 {
        bit_count[idx] -= (number & 1) as u32;
        number >>= 1;
        idx += 1;
    }
}

fn bit_sum(bit_count: &[u32; 32]) -> i32 {
    let mut res = 0;
    (0..32).for_each(|i| {
        if bit_count[i] > 0 {
            res |= 1 << i;
        }
    });
    res
}

#[cfg(test)]
mod tests {
    use crate::leetcode::shortest_subarray_xor::minimum_subarray_length;

    #[test]
    fn calculates_minimum_or_subarray_length() {
        assert_eq!(1, minimum_subarray_length(vec![1, 2], 0));
        assert_eq!(3, minimum_subarray_length(vec![2, 1, 8], 10));
        assert_eq!(1, minimum_subarray_length(vec![1, 2, 3], 2));
        assert_eq!(
            6,
            minimum_subarray_length(vec![1, 3, 3, 3, 3, 2, 4, 8, 16, 3, 3, 32], 63)
        );
    }
}
