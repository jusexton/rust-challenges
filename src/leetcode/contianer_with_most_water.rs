use std::cmp;

pub fn max_area(heights: Vec<i32>) -> i32 {
    let mut max_area = i32::MIN;
    let (mut left, mut right) = (0, heights.len() - 1);
    while left < right {
        let area = (right - left) * cmp::min(heights[left], heights[right]) as usize;
        max_area = cmp::max(max_area, area as i32);

        match heights[left] > heights[right] {
            true => right -= 1,
            false => left += 1,
        }
    }
    max_area
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn max_water_area_is_calculated() {
        assert_eq!(49, max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]))
    }
}
