pub fn merge(vec_one: &mut [i32], m: i32, vec_two: &mut [i32], n: i32) {
    let (mut idx_one, mut idx_two) = (m - 1, n - 1);
    let mut right = (m + n - 1) as usize;

    while idx_two >= 0 {
        if idx_one >= 0 && vec_one[idx_one as usize] > vec_two[idx_two as usize] {
            vec_one[right] = vec_one[idx_one as usize];
            idx_one -= 1;
        } else {
            vec_one[right] = vec_two[idx_two as usize];
            idx_two -= 1;
        }
        right -= 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_merge_two_sorted_arrays() {
        let mut vec_one = vec![1, 2, 3, 0, 0, 0];
        merge(&mut vec_one, 3, &mut vec![2, 5, 6], 3);

        let expected = vec![1, 2, 2, 3, 5, 6];
        assert_eq!(expected, vec_one);
    }
}
