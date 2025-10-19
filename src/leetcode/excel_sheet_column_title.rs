pub fn title_to_number(column_title: String) -> i32 {
    column_title.chars().fold(0, |mut acc, curr| {
        acc = acc * 26 + curr as i32 - 65 + 1;
        acc
    })
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use crate::leetcode::excel_sheet_column_title::title_to_number;

    #[test_case("AA", 27)]
    #[test_case("ZY", 701)]
    fn title_to_number_tests(input: &str, expected: i32) {
        assert_eq!(expected, title_to_number(input.to_string()))
    }
}
