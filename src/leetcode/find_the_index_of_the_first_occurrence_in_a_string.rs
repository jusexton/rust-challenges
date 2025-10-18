pub fn str_str(haystack: String, needle: String) -> i32 {
    if haystack.len() < needle.len() {
        return -1;
    }

    let haystack = haystack.as_bytes();
    let needle = needle.as_bytes();
    for i in 0..haystack.len() - needle.len() {
        let mut j = 0;
        while j < needle.len() && haystack[i + j] == needle[j] {
            println!("i:{i}");
            j += 1;
        }

        if j == needle.len() {
            return i as i32;
        }
    }

    -1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_index_of_needle_in_haystack() {
        assert_eq!(0, str_str("sadbutsad".to_string(), "sad".to_string()));
        assert_eq!(-1, str_str("leetcode".to_string(), "leeto".to_string()))
    }
}
