pub fn process_str(s: String) -> String {
    s.chars().fold(String::new(), |mut result, char| {
        match char {
            'a'..='z' => result.push(char),
            '*' => {
                result.pop();
            }
            '#' => result = result.repeat(2),
            '%' => unsafe {
                result.as_bytes_mut().reverse();
            },
            _ => unreachable!(),
        }
        result
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_str() {
        assert_eq!("ba".to_string(), process_str("a#b%*".to_string()));
        assert_eq!("".to_string(), process_str("z*#".to_string()));
    }
}
