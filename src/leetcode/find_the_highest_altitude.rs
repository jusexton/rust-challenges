pub fn largest_altitude(gain: Vec<i32>) -> i32 {
    let mut max_alt = 0;
    let mut curr_alt = 0;
    for g in gain {
        curr_alt += g;
        if curr_alt > max_alt {
            max_alt = curr_alt;
        }
    }
    max_alt
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_largest_altitude() {
        assert_eq!(largest_altitude(vec![-5, 1, 5, 0, -7]), 1);
        assert_eq!(largest_altitude(vec![-4, -3, -2, -1, 4, 3, 2]), 0);
    }
}
