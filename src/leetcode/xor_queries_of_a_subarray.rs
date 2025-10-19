pub fn xor_queries(mut arr: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
    for idx in 1..arr.len() {
        arr[idx] ^= arr[idx - 1];
    }
    queries
        .iter()
        .map(|query| {
            let (start, end) = (query[0] as usize, query[1] as usize);
            match start {
                0 => arr[end],
                _ => arr[start - 1] ^ arr[end],
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::xor_queries;

    #[test]
    fn test() {
        let result = xor_queries(
            vec![1, 3, 4, 8],
            vec![vec![0, 1], vec![1, 2], vec![0, 3], vec![3, 3]],
        );
        assert_eq!(vec![2, 7, 14, 8], result)
    }
}
