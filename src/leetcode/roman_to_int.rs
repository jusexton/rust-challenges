use std::collections::HashMap;

pub fn roman_to_int(s: String) -> i32 {
    let roman = HashMap::from([
        (b'I', 1),
        (b'V', 5),
        (b'X', 10),
        (b'L', 50),
        (b'C', 100),
        (b'D', 500),
        (b'M', 1000),
    ]);

    let s = s.as_bytes();
    let mut sum = 0;
    for window in s.windows(2) {
        let (left, right) = (window[0], window[1]);
        let (left, right) = (roman.get(&left).unwrap(), roman.get(&right).unwrap());
        if left < right {
            sum -= left;
        } else {
            sum += left;
        }
    }

    sum += roman.get(&s[s.len() - 1]).unwrap();
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn converts_roman_string_to_integer() {
        // assert_eq!(3, roman_to_int("III".to_string()));
        assert_eq!(1994, roman_to_int("MCMXCIV".to_string()))
    }
}
