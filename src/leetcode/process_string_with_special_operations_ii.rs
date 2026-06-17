pub fn process_str(s: String, mut k: i64) -> char {
    let mut length = s.chars().fold(0, |acc, ch| match ch {
        '*' if acc > 0 => acc - 1,
        '*' => acc,
        '#' => acc << 1,
        '%' => acc,
        _ => acc + 1,
    });

    if k >= length {
        return '.';
    }

    for char in s.chars().rev() {
        match char {
            '*' => {
                length += 1;
            }
            '#' => {
                length >>= 1;
                if k >= length {
                    k -= length;
                }
            }
            '%' => {
                k = length - 1 - k;
            }
            _ => {
                length -= 1;
                if k == length {
                    return char;
                }
            }
        }
    }

    // Because of the check on line 10, we know the function will be able to resolve
    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_str() {
        assert_eq!('a', process_str("a#b%*".to_string(), 1));
        assert_eq!('d', process_str("cd%#*#".to_string(), 3));
        assert_eq!('.', process_str("z*#".to_string(), 0));
    }
}
