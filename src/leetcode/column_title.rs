pub fn convert_to_title(mut column_number: i32) -> String {
    let mut title = String::new();
    while column_number > 0 {
        column_number -= 1;
        title.push(((column_number % 26) as u8 + b'A') as char);
        column_number /= 26;
    }
    title.chars().rev().collect()
}

#[cfg(test)]
mod tests {
    use crate::leetcode::column_title::convert_to_title;

    #[test]
    fn produces_correct_title() {
        assert_eq!("A".to_string(), convert_to_title(1));
        assert_eq!("Z".to_string(), convert_to_title(26));
        assert_eq!("AA".to_string(), convert_to_title(27));
        assert_eq!("ZY".to_string(), convert_to_title(701));
        assert_eq!("FXSHRXW".to_string(), convert_to_title(2147483647));
    }
}
