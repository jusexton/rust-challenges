pub fn take_characters(s: String, k: i32) -> i32 {
    let s_bytes = s.as_bytes();
    let mut total_count = s_bytes.iter().fold([0; 3], |mut acc, curr| {
        acc[(curr - b'a') as usize] += 1;
        acc
    });

    if total_count.iter().any(|&c| c < k) {
        return -1;
    }

    let mut res = i32::MAX;
    let mut left = 0;
    for right in 0..s_bytes.len() {
        total_count[(s_bytes[right] - b'a') as usize] -= 1;

        while total_count.iter().any(|&c| c < k) {
            total_count[(s_bytes[left] - b'a') as usize] += 1;
            left += 1;
        }
        res = res.min(s_bytes.len() as i32 - (right as i32 - left as i32 + 1));
    }
    res
}

#[cfg(test)]
mod tests {
    use super::take_characters;

    #[test]
    fn finds_minimum_number_of_characters_to_take_from_left_and_right() {
        assert_eq!(8, take_characters("aabaaaacaabc".to_string(), 2))
    }
}
