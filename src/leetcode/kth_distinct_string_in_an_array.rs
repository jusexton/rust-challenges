use std::collections::HashMap;

pub fn kth_distinct(arr: Vec<String>, mut k: i32) -> String {
    let counts = arr.iter().fold(HashMap::new(), |mut acc, curr| {
        *acc.entry(curr).or_insert(0) += 1;
        acc
    });

    for s in arr.iter() {
        if counts[&s] == 1 {
            k -= 1;
            if k == 0 {
                return s.clone();
            }
        }
    }

    "".to_string()
}

#[cfg(test)]
mod tests {
    use crate::string_vec;

    use super::kth_distinct;

    #[test]
    fn kth_distinct_string_is_returned() {
        let strings = string_vec!["d", "b", "c", "b", "c", "a"];
        let k = 2;
        assert_eq!("a".to_string(), kth_distinct(strings, k))
    }
}
