pub fn find_min_difference(time_points: Vec<String>) -> i32 {
    let mut minutes: Vec<_> = time_points
        .into_iter()
        .map(|time| time[..2].parse::<i32>().unwrap() * 60 + time[3..].parse::<i32>().unwrap())
        .collect();

    minutes.sort_unstable();

    let result = minutes
        .windows(2)
        .fold(i32::MAX, |acc, curr| acc.min(curr[1] - curr[0]));
    result.min(24 * 60 - minutes[minutes.len() - 1] + minutes[0])
}

#[cfg(test)]
mod tests {
    use super::find_min_difference;

    #[test]
    fn test() {
        let times = vec!["23:59".to_string(), "00:00".to_string()];
        assert_eq!(1, find_min_difference(times));

        let times = vec![
            "00:00".to_string(),
            "23:59".to_string(),
            "00:00".to_string(),
        ];
        assert_eq!(0, find_min_difference(times))
    }
}
