pub fn total_money(mut n: i32) -> i32 {
    let mut week = 1;
    let mut money = 0;
    while n >= 7 {
        money += ((week + week + 7 - 1) * 7) / 2;
        n -= 7;
        week += 1;
    }

    let remaining_days = n % 7;
    if remaining_days > 0 {
        money += ((week + week + remaining_days - 1) * remaining_days) / 2;
    }

    money
}

#[cfg(test)]
mod tests {
    use crate::leetcode::calculate_money_in_leetcode_bank::total_money;

    #[test]
    fn test_total_money() {
        assert_eq!(10, total_money(4));
        assert_eq!(37, total_money(10));
        assert_eq!(96, total_money(20));
    }
}
