fn min_end(mut n: i32, x: i32) -> i64 {
    let mut res = x as i64;
    loop {
        n -= 1;
        if n <= 0 {
            break;
        }
        res = (res + 1) | x as i64;
    }
    res
}

#[cfg(test)]
mod tests {
    use super::min_end;

    #[test]
    fn n_as_one_returns_itself() {
        assert_eq!(5, min_end(1, 5))
    }

    #[test]
    fn returns_correct_end() {
        assert_eq!(6, min_end(3, 4))
    }
}
