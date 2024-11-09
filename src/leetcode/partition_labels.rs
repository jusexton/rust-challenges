pub fn partition_labels(s: String) -> Vec<i32> {
    let right_most = s.char_indices().fold([0; 26], |mut acc, (idx, ch)| {
        acc[ch as usize - 97] = idx as i32;
        acc
    });

    let mut res = vec![];
    let (mut left, mut right) = (0, 0);
    for (idx, ch) in s.char_indices() {
        right = right.max(right_most[ch as usize - 97]);
        if right == idx as i32 {
            res.push(right - left + 1);
            left = right + 1
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::partition_labels;

    #[test]
    fn returns_single_length_when_only_one_partition() {
        assert_eq!(vec![10], partition_labels("eccbbbbdec".to_string()))
    }

    #[test]
    fn returns_partition_lengths() {
        assert_eq!(
            vec![9, 7, 8],
            partition_labels("ababcbacadefegdehijhklij".to_string())
        )
    }
}
