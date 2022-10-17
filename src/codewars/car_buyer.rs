const VALUE_DECREASE_RATE: f64 = 0.005;

fn nb_months(old: i32, new: i32, saving: i32, percent: f64) -> (i32, i32) {
    let mut old = old as f64;
    let mut new = new as f64;
    let mut month = 0;
    let mut savings = 0.0;
    let mut percent = 1.0 - (percent / 100.0);
    while old + savings < new {
        month += 1;
        if month % 2 == 0 {
            percent -= VALUE_DECREASE_RATE;
        }

        old *= percent;
        new *= percent;
        savings += saving as f64;
    }
    (month, ((old + savings) - new).round() as i32)
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use crate::codewars::car_buyer::nb_months;

    #[test_case(2000, 8000, 1000, 1.5, (6, 766))]
    #[test_case(12000, 8000, 1000, 1.5 , (0, 4000))]
    #[test_case(18000, 32000, 1500, 1.25, (8, 332))]
    #[test_case(7500, 32000, 300, 1.55, (25, 122))]
    fn should_calculate_when_a_new_car_can_be_bought(
        old: i32,
        new: i32,
        saving: i32,
        percent: f64,
        expected: (i32, i32),
    ) {
        let actual = nb_months(old, new, saving, percent);
        assert_eq!(actual, expected)
    }
}
