use std::collections::HashMap;

struct RangeFreqQuery {
    values: HashMap<i32, Vec<i32>>,
}

impl RangeFreqQuery {
    fn new(arr: Vec<i32>) -> Self {
        let n = arr.len();
        let values: HashMap<i32, Vec<i32>> =
            arr.into_iter()
                .enumerate()
                .fold(HashMap::with_capacity(n), |mut acc, (idx, v)| {
                    acc.entry(v).or_default().push(idx as i32);
                    acc
                });
        Self { values }
    }

    fn query(&self, left: i32, right: i32, value: i32) -> i32 {
        match self.values.get(&value) {
            Some(vec) => {
                let r = vec.partition_point(|&probe| probe < right + 1);
                let l = vec.partition_point(|&probe| probe < left);
                (r - l) as i32
            }
            None => 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::RangeFreqQuery;

    #[test]
    fn queries_freq_of_number_in_provided_range() {
        let freq_range = RangeFreqQuery::new(vec![12, 33, 4, 56, 22, 2, 34, 33, 22, 12, 34, 56]);
        assert_eq!(1, freq_range.query(1, 2, 4));
        assert_eq!(2, freq_range.query(0, 11, 33))
    }
}
