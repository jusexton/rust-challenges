fn longest_slide_down(pyramid: &[Vec<u16>]) -> u16 {
    let mut acc = vec![0; pyramid.len() + 1];
    for (r_index, row) in pyramid.iter().enumerate().rev() {
        for i in 0..=r_index {
            acc[i] = acc[i].max(acc[i + 1]) + row[i];
        }
    }
    acc[0]
}

#[cfg(test)]
mod tests {
    use crate::codewars::pyramid_slide::longest_slide_down;

    #[test]
    fn test_longest_slide_down() {
        let input = &[vec![3], vec![7, 4], vec![2, 4, 6], vec![8, 5, 9, 3]];
        let result = longest_slide_down(input);

        assert_eq!(result, 23)
    }

    #[test]
    fn test_longest_slide_down_with_single_value() {
        let input = &[vec![3]];
        let result = longest_slide_down(input);

        assert_eq!(result, 3)
    }
}
