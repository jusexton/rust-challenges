pub fn next_greatest_letter(letters: Vec<char>, target: char) -> char {
    let (mut left, mut right) = (0, letters.len());
    while left < right {
        let mid = left + (right - left) / 2;
        match letters[mid] > target {
            true => right = mid,
            false => left = mid + 1,
        }
    }
    letters[left % letters.len()]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_next_greatest_letter() {
        assert_eq!('b', next_greatest_letter(vec!['b', 'k'], 'a'));
        assert_eq!('c', next_greatest_letter(vec!['c', 'f', 'j'], 'a'));
        assert_eq!('f', next_greatest_letter(vec!['c', 'f', 'j'], 'c'));
        assert_eq!('x', next_greatest_letter(vec!['x', 'x', 'y', 'y'], 'z'));
    }
}
