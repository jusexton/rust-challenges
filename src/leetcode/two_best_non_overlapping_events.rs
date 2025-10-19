pub fn max_two_events(events: Vec<Vec<i32>>) -> i32 {
    let mut event_boundaries = Vec::with_capacity(events.len() * 2);
    for event in events {
        event_boundaries.push((event[0], true, event[2]));
        event_boundaries.push((event[1] + 1, false, event[2]));
    }
    event_boundaries.sort_unstable();

    let mut res = 0;
    let mut m = 0;
    for (_, is_start, value) in event_boundaries {
        if is_start {
            res = res.max(m + value);
        } else {
            m = m.max(value);
        }
    }

    res
}

#[cfg(test)]
mod tests {
    use super::max_two_events;

    #[test]
    fn finds_the_two_max_value_events() {
        assert_eq!(
            4,
            max_two_events(vec![vec![1, 3, 2], vec![4, 5, 2], vec![2, 4, 3]])
        )
    }
}
