use std::{
    collections::{BinaryHeap, HashMap},
    iter::FromIterator,
};

pub fn repeat_limited_string(s: String, repeat_limit: i32) -> String {
    let freq_count = s.chars().fold(HashMap::new(), |mut acc, curr| {
        *acc.entry(curr).or_insert(0) += 1;
        acc
    });
    let mut priority_queue = BinaryHeap::from_iter(freq_count);

    let mut result = String::new();
    while let Some((char, count)) = priority_queue.pop() {
        let take_count = count.min(repeat_limit);
        for _ in 0..take_count {
            result.push(char);
        }

        let updated_count = count - take_count;
        if updated_count > 0 {
            if let Some((next_char, next_count)) = priority_queue.pop() {
                result.push(next_char);
                priority_queue.push((char, updated_count));
                if next_count - 1 > 0 {
                    priority_queue.push((next_char, next_count - 1))
                }
            } else {
                return result;
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::repeat_limited_string;

    #[test]
    fn creates_largest_lexographically_sortable_string() {
        assert_eq!(
            "zzcccac".to_string(),
            repeat_limited_string("cczazcc".to_string(), 3)
        );

        assert_eq!(
            "bbabaa".to_string(),
            repeat_limited_string("aababab".to_string(), 2)
        )
    }
}
