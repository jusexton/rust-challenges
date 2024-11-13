use std::cmp::Ordering;

pub fn judge_square_sum(c: i32) -> bool {
    let (mut left, mut right) = (0, (c as f64).sqrt() as i32);
    while left <= right {
        let curr = (left * left) + (right * right);
        match curr.cmp(&c) {
            Ordering::Equal => return true,
            Ordering::Less => {
                left += 1;
            }
            Ordering::Greater => {
                right -= 1;
            }
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::judge_square_sum;

    #[test]
    fn correctly_determines_if_square_sums_exist() {
        assert!(judge_square_sum(5))
    }

    #[test]
    fn correctly_determines_if_square_sums_do_not_exist() {
        assert!(!judge_square_sum(3))
    }
}
