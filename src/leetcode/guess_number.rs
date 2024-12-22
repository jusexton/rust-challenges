use std::cmp::Ordering;

unsafe fn guess(number: i32) -> i32 {
    match number.cmp(&5) {
        Ordering::Less => 1,
        Ordering::Equal => 0,
        Ordering::Greater => -1,
    }
}

unsafe fn guess_number(n: i32) -> i32 {
    let (mut left, mut right) = (1, n);
    while left <= right {
        let mid = left + (right - left) / 2;
        match guess(mid) {
            1 => left = mid + 1,
            0 => return mid,
            -1 => right = mid - 1,
            _ => {}
        }
    }
    left
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn guesses_correct_number() {
        assert_eq!(5, unsafe { guess_number(20) })
    }
}
