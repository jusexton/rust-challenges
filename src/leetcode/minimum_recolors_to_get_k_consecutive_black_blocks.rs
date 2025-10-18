pub fn minimum_recolors(blocks: String, k: i32) -> i32 {
    let k = k as usize;
    let blocks = blocks.as_bytes();
    let mut black_count = 0;
    let mut min_count = i32::MAX;

    for i in 0..blocks.len() {
        if i >= k && blocks[i - k] == b'B' {
            black_count -= 1;
        }
        if blocks[i] == b'B' {
            black_count += 1;
        }
        min_count = min_count.min(k as i32 - black_count);
    }

    min_count
}

#[cfg(test)]
mod tests {
    use super::minimum_recolors;

    #[test]
    fn minimum_number_of_recolors() {
        assert_eq!(3, minimum_recolors("WBBWWBBWBW".to_string(), 7));
        assert_eq!(0, minimum_recolors("WBB".to_string(), 1))
    }
}
