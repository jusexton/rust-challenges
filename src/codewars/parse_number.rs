use std::{collections::HashMap, iter::FromIterator};

fn parse_bank_account(bank_account: &str) -> u64 {
    let mapping: HashMap<&str, u64> = HashMap::from_iter([
        (" _ | ||_|", 0),
        ("     |  |", 1),
        (" _  _||_ ", 2),
        (" _  _| _|", 3),
        ("   |_|  |", 4),
        (" _ |_  _|", 5),
        (" _ |_ |_|", 6),
        (" _   |  |", 7),
        (" _ |_||_|", 8),
        (" _ |_| _|", 9),
    ]);
    let line_length = (bank_account.len() - 3) / 3;
    let parts: Vec<&str> = bank_account.split('\n').collect();
    let mut result = 0;
    for idx in (0..line_length).step_by(3) {
        let digit_str = format!(
            "{}{}{}",
            &parts[0][idx..(idx + 3)],
            &parts[1][idx..(idx + 3)],
            &parts[2][idx..(idx + 3)]
        );
        result = (result * 10) + mapping.get(digit_str.as_str()).unwrap();
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            123456789,
            parse_bank_account(concat!(
                "    _  _     _  _  _  _  _ \n",
                "  | _| _||_||_ |_   ||_||_|\n",
                "  ||_  _|  | _||_|  ||_| _|\n"
            ))
        );
        assert_eq!(
            23056789,
            parse_bank_account(concat!(
                " _  _  _  _  _  _  _  _  _ \n",
                "| | _| _|| ||_ |_   ||_||_|\n",
                "|_||_  _||_| _||_|  ||_| _|\n"
            ))
        );
        assert_eq!(
            823856989,
            parse_bank_account(concat!(
                " _  _  _  _  _  _  _  _  _ \n",
                "|_| _| _||_||_ |_ |_||_||_|\n",
                "|_||_  _||_| _||_| _||_| _|\n"
            ))
        );
    }
}
