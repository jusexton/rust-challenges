// int[] res = new int[n.length];
// int val = (1 << maximumBit) - 1;
// for (int i = 0; i < n.length; ++i)
//     res[n.length - i - 1] = val ^= n[i];
// return res;

fn get_maximum_xor(numbers: Vec<i32>, maximum_bit: i32) -> Vec<i32> {
    let n = numbers.len();
    let mut res = vec![0; n];
    let mut val = (1 << maximum_bit) - 1;
    for i in 0..n {
        val ^= numbers[i];
        res[n - i - 1] = val;
    }
    res
}

#[cfg(test)]
mod tests {
    use super::get_maximum_xor;

    #[test]
    fn returns_maximum_xor() {
        assert_eq!(vec![0, 3, 2, 3], get_maximum_xor(vec![0, 1, 1, 3], 2))
    }
}
