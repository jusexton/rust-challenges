pub fn count_seniors(details: Vec<String>) -> i32 {
    details
        .into_iter()
        .filter(|d| d[11..13].parse::<i32>().unwrap() > 60)
        .count() as i32
}

#[cfg(test)]
mod tests {
    use super::count_seniors;

    #[test]
    fn returns_number_of_senior_citizens() {
        let details = vec![
            "7868190130M7522".to_string(),
            "5303914400F9211".to_string(),
            "9273338290F4010".to_string(),
        ];
        assert_eq!(2, count_seniors(details))
    }
}
