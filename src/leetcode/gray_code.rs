pub fn gray_code(n: i32) -> Vec<i32> {
    (0..1 << n).map(|i| i ^ (i >> 1)).collect()
}

#[cfg(test)]
mod tests {
    use crate::leetcode::gray_code::gray_code;

    #[test]
    fn produces_gray_code() {
        assert_eq!(vec![0, 1], gray_code(1));
        assert_eq!(vec![0, 1, 3, 2], gray_code(2));
    }
}
