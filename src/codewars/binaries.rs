use std::fmt::Write;

fn code(s: &str) -> String {
    s.chars().fold(String::new(), |mut acc, curr| {
        let int_value = curr.to_digit(10).unwrap();
        let bits = format!("{:b}", int_value);
        let zero_count = bits.len() - 1;
        write!(acc, "{}1{}", "0".repeat(zero_count), bits).expect("Buffer write failed.");
        acc
    })
}

fn decode(s: &str) -> String {
    let mut result = String::new();
    let mut slice = s;
    while let Some(position) = slice.find('1') {
        let position = position + 1;
        let bits = &slice[position..position * 2];
        let decoded = u8::from_str_radix(bits, 2).unwrap();
        result.push_str(&decoded.to_string());
        slice = &slice[position * 2..]
    }

    result
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use crate::codewars::binaries::{code, decode};

    #[test_case("0", String::from("10"))]
    #[test_case("2", String::from("0110"))]
    #[test_case("3", String::from("0111"))]
    #[test_case("62", String::from("0011100110"))]
    fn should_correctly_code_given_str_int(str: &str, expected: String) {
        let actual = code(str);
        assert_eq!(actual, expected)
    }

    #[test_case("10001111", String::from("07"))]
    #[test_case(
        "01110111110001100100011000000110000011110011110111011100110000110001100110",
        String::from("33198877334422")
    )]
    #[test_case("001100001100001100001110001110001110011101110111001110001110001110001111001111001111001100001100001100", String::from("444666333666777444"))]
    fn should_correctly_decode_given_coded_value(coded: &str, expected: String) {
        let actual = decode(coded);
        assert_eq!(actual, expected)
    }
}
