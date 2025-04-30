pub fn can_attend_meetings(mut intervals: Vec<Vec<i32>>) -> bool {
    intervals.sort_unstable_by_key(|inter| inter[0]);

    for w in intervals.windows(2) {
        if w[1][0] < w[0][1] {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use crate::{cell_vec, leetcode::meeting_rooms::can_attend_meetings};

    #[test]
    fn all_meetings_can_be_attended() {
        assert!(can_attend_meetings(cell_vec![(7, 10), (2, 4)]))
    }

    #[test]
    fn all_meetings_can_not_be_attended() {
        assert!(!can_attend_meetings(cell_vec![(7, 10), (2, 10)]))
    }
}
