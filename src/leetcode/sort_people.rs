pub fn sort_people(names: Vec<String>, heights: Vec<i32>) -> Vec<String> {
    let mut values: Vec<_> = names.iter().zip(heights).collect();
    values.sort_by_key(|f| f.1);
    values.into_iter().map(|f| f.0.clone()).rev().collect()
}

#[cfg(test)]
mod tests {
    use crate::string_vec;

    use super::*;

    #[test]
    fn test_sort_people() {
        let names = string_vec!["Mary", "John", "Emma"];
        let heights = vec![180, 165, 170];
        let sorted = sort_people(names, heights);

        assert_eq!(string_vec!["Mary", "Emma", "John"], sorted)
    }
}
