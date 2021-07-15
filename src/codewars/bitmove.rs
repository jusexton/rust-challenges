fn interpreter(tape: &str, data: &str) -> String {
    let mut commands = tape.chars().cycle();
    data.chars()
        .map(|mut c| {
            while commands.next().unwrap() == '1' {
                if c == '0' {
                    c = '1'
                } else {
                    c = '0'
                }
            }
            c
        })
        .collect()
}

#[cfg(test)]
mod tests {
    #![cfg(test)]
    extern crate test_case;

    use test_case::test_case;

    use crate::codewars::bitmove::interpreter;

    #[test_case("10", "1010101", String::from("0101010"))]
    #[test_case("100", "1111111111", String::from("0101010101"))]
    #[test_case("1000001", "1111010", String::from("0111010"))]
    fn should_correctly_flip_specified_bits(tape: &str, data: &str, expected: String) {
        let actual = interpreter(tape, data);
        assert_eq!(actual, expected)
    }
}
