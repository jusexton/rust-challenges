fn people_on_the_bus(bus_stops: &[(i32, i32)]) -> i32 {
    bus_stops
        .iter()
        .fold(0, |acc, curr| acc + (curr.0 - curr.1))
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use crate::codewars::people_on_the_bus::people_on_the_bus;

    #[test_case(&[(5, 0)], 5)]
    #[test_case(&[(5, 0), (5, 0)], 10)]
    #[test_case(&[(5, 0), (0, 5)], 0)]
    #[test_case(&[(5, 0), (2, 2)], 5)]
    #[test_case(&[(5, 0), (2, 3)], 4)]
    fn should_calculate_how_many_people_are_left_on_the_bus(stops: &[(i32, i32)], expected: i32) {
        let actual = people_on_the_bus(stops);
        assert_eq!(actual, expected)
    }
}
