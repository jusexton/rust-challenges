pub fn max_depth(s: String) -> i32 {
    let mut current_depth = 0;
    let mut max_depth = 0;
    for c in s.chars() {
        match c {
            '(' => current_depth += 1,
            ')' => {
                max_depth = max_depth.max(current_depth);
                current_depth -= 1;
            }
            _ => {}
        }
    }
    max_depth
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_depth() {
        assert_eq!(2, max_depth("(())".to_string()));
        assert_eq!(3, max_depth("(1+(2*3)+((8)/4))+1".to_string()));
        assert_eq!(3, max_depth("()(())((()()))".to_string()));
    }
}
