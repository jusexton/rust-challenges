pub fn merge(vec_one: &mut [i32], m: i32, vec_two: &mut [i32], n: i32) {
    let (mut idx_one, mut idx_two) = (m - 1, n - 1);
    let mut right = (m + n - 1) as usize;

    while let Some(&vec_two_item) = vec_two.get(idx_two as usize) {
        if let Some(&vec_one_item) = vec_one.get(idx_one as usize)
            && vec_one_item > vec_two_item
        {
            vec_one[right] = vec_one_item;
            idx_one -= 1;
        } else {
            vec_one[right] = vec_two_item;
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
        merge(&mut vec_one, 3, &mut [2, 5, 6], 3);

        let expected = vec![1, 2, 2, 3, 5, 6];
        assert_eq!(expected, vec_one);
    }
}
