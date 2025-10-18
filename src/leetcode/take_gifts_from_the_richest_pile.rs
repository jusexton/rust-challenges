use std::collections::BinaryHeap;

pub fn pick_gifts(gifts: Vec<i32>, mut seconds: i32) -> i64 {
    let mut heap = BinaryHeap::from(gifts);

    while seconds > 0 {
        let pile = heap.pop().unwrap();
        heap.push((pile as f64).sqrt().floor() as i32);
        seconds -= 1;
    }

    heap.iter().map(|&n| n as i64).sum()
}

#[cfg(test)]
mod tests {
    use super::pick_gifts;

    #[test]
    fn computes_left_over_gifts_after_k_seconds() {
        assert_eq!(29, pick_gifts(vec![25, 64, 9, 4, 100], 4));
        assert_eq!(4, pick_gifts(vec![1, 1, 1, 1], 4));
    }
}
