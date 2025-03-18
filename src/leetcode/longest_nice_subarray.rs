pub fn longest_nice_subarray(numbers: Vec<i32>) -> i32 {
    let mut used_bits = 0;
    let mut left = 0;
    let mut res = usize::MIN;

    for right in 0..numbers.len() {
        while used_bits & numbers[right] != 0 {
            used_bits ^= numbers[left];
            left += 1;
        }
        used_bits |= numbers[right];
        res = res.max(right - left + 1)
    }

    res as i32
}
