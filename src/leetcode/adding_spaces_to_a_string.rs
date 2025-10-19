pub fn add_spaces(s: String, spaces: Vec<i32>) -> String {
    let mut res = String::with_capacity(s.len() + spaces.len());
    let mut space_idx = 0;
    for (ch_idx, ch) in s.char_indices() {
        if spaces.get(space_idx) == Some(&(ch_idx as i32)) {
            res.push(' ');
            space_idx += 1;
        }
        res.push(ch);
    }
    res
}

// Optimized approach using memmove without iterating over every character in the given s.
// pub fn add_spaces(s: String, spaces: Vec<i32>) -> String {
//     let mut bytes = s.into_bytes();
//     let mut chunk_end_i = bytes.len();
//     bytes.extend(std::iter::repeat(0).take(spaces.len()));

//     for (i, space) in spaces.into_iter().enumerate().rev() {
//         let space = space as usize;
//         bytes.copy_within(space..chunk_end_i, space + i + 1);
//         bytes[space + i] = b' ';
//         chunk_end_i = space;
//     }

//     unsafe { String::from_utf8_unchecked(bytes) }
// }

#[cfg(test)]
mod tests {
    use super::add_spaces;

    #[test]
    fn inserts_spaces_throughout_given_string() {
        assert_eq!(
            "Leetcode Helps Me Learn".to_string(),
            add_spaces("LeetcodeHelpsMeLearn".to_string(), vec![8, 13, 15])
        );

        assert_eq!(
            "i code in py thon".to_string(),
            add_spaces("icodeinpython".to_string(), vec![1, 5, 7, 9])
        );
    }
}
