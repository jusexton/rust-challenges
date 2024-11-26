use std::collections::HashMap;

pub fn single_number(numbers: Vec<i32>) -> i32 {
    let counter = numbers.into_iter().fold(HashMap::new(), |mut acc, curr| {
        *acc.entry(curr).or_insert(0) += 1;
        acc
    });
    counter.into_iter().find(|(_, v)| *v == 1).unwrap().0
}

// Optimized solution
// Credit: https://leetcode.com/problems/single-number-ii/solutions/2389678/rust-concise-solution-with-ternary-logic-explained/
// pub fn single_number(numbers: Vec<i32>) -> i32 {
//     numbers
//         .into_iter()
//         .fold([0, 0], |[a, b], x| [a ^ x & !b, a & x | b & !x])[0]
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_the_number_that_appears_once() {
        assert_eq!(3, single_number(vec![2, 2, 3, 2]));
    }
}
