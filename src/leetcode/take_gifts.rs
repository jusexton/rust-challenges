use std::collections::BinaryHeap;

pub fn pick_gifts(gifts: Vec<i32>, mut k: i32) -> i64 {
    let mut heap = BinaryHeap::from(gifts);

    while k > 0 {
        let pile = heap.pop().unwrap();
        heap.push((pile as f64).sqrt().floor() as i32);
        k -= 1;
    }

    heap.iter().map(|&n| n as i64).sum()
}
