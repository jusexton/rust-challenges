fn multi_table(number: u64) -> String {
    (1..=10)
        .map(|n| format!("{} * {} = {}", n, number, n * number))
        .collect::<Vec<String>>()
        .join("\n")
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use crate::codewars::multiplication_table::multi_table;

    #[test_case(5, "1 * 5 = 5\n2 * 5 = 10\n3 * 5 = 15\n4 * 5 = 20\n5 * 5 = 25\n6 * 5 = 30\n7 * 5 = 35\n8 * 5 = 40\n9 * 5 = 45\n10 * 5 = 50")]
    fn should_return_correct_multiplication_table(number: u64, expected: &str) {
        let table_string = multi_table(number);
        assert_eq!(table_string, expected.to_string());
    }
}
