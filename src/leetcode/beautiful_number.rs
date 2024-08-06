fn next_beautiful_number(n: i32) -> i32 {
    fn is_beautiful(mut n: usize) -> bool {
        let mut counts = [0; 10];
        while n > 0 {
            counts[n % 10] += 1;
            n /= 10;
        }
        (0..10).all(|idx| {
            let count = counts[idx];
            match count {
                0 => true,
                _ => count == idx,
            }
        })
    }
    (n + 1..).find(|n| is_beautiful(*n as usize)).unwrap()
}

#[cfg(test)]
mod tests {
    use super::next_beautiful_number;

    #[test]
    fn next_number() {
        assert_eq!(22, next_beautiful_number(1));
        assert_eq!(1333, next_beautiful_number(1000));
        assert_eq!(3133, next_beautiful_number(3000));
    }
}
