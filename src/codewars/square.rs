// https://www.codewars.com/kata/59a96d71dbe3b06c0200009c
fn generate_square(n: i32) -> String {
    if n < 0 {
        panic!("n cannot be a negative value.")
    }

    let layer = "+".repeat(n as usize);
    (0..n)
        .map(|_| String::from(&layer))
        .collect::<Vec<_>>()
        .join("\n")
}

// Refactored version
// fn generate_square(n: usize) -> String {
//     vec!["+".repeat(n); n].join("\n")
// }

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use crate::codewars::square::generate_square;

    #[test_case(0, "")]
    #[test_case(1, "+")]
    #[test_case(2, "++\n++")]
    #[test_case(3, "+++\n+++\n+++")]
    #[test_case(4, "++++\n++++\n++++\n++++")]
    fn test_generate_square(input: i32, expected: &str) {
        assert_eq!(expected.to_string(), generate_square(input))
    }

    #[test]
    #[should_panic]
    fn test_generate_square_with_negative_value() {
        generate_square(-1);
    }
}
