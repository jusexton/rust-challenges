fn gps(s: i32, x: Vec<f64>) -> i32 {
    x.windows(2)
        .map(|distance| (3600.0 * (distance[1] - distance[0]) / s as f64) as i32)
        .max()
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    #![cfg(test)]
    extern crate test_case;

    use test_case::test_case;

    use crate::codewars::gps::gps;

    #[test_case(15, &[1.0], 0)]
    #[test_case(15, &[0.0, 0.19, 0.5, 0.75, 1.0, 1.25, 1.5, 1.75, 2.0, 2.25], 74)]
    fn should_return_the_maximum_average_speed_per_hour(
        interval: i32,
        positions: &[f64],
        expected: i32,
    ) {
        let max_average_speed = gps(interval, positions.to_vec());
        assert_eq!(max_average_speed, expected)
    }
}
