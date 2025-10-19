use std::collections::HashSet;

pub fn find_score(numbers: Vec<i32>) -> i64 {
    let mut marked: HashSet<usize> = HashSet::with_capacity(numbers.len());
    let mut numbers = numbers.into_iter().zip(0..).collect::<Vec<_>>();
    numbers.sort_unstable();
    numbers.into_iter().fold(0, |acc, (number, idx)| {
        if marked.contains(&idx) {
            return acc;
        }
        marked.insert(idx);
        marked.insert(idx.saturating_sub(1));
        marked.insert(idx + 1);
        acc + number as i64
    })
}

#[cfg(test)]
mod tests {
    use crate::leetcode::find_score_of_an_array_after_marking_all_elements::find_score;

    #[test]
    fn finds_score_after_marking_numbers() {
        assert_eq!(7, find_score(vec![2, 1, 3, 4, 5, 2]))
    }
}
