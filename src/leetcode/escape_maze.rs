use std::collections::HashSet;

pub fn is_escape_possible(blocked: Vec<Vec<i32>>, source: Vec<i32>, target: Vec<i32>) -> bool {
    fn dfs(
        current: (i32, i32),
        target: (i32, i32),
        seen: &mut HashSet<(i32, i32)>,
        blocked: &HashSet<(i32, i32)>,
    ) -> bool {
        if current.0 < 0
            || current.0 >= 1_000_000
            || current.1 < 0
            || current.1 >= 1_000_000
            || seen.contains(&current)
            || blocked.contains(&current)
        {
            return false;
        }

        seen.insert(current);
        if seen.len() > 20_000 || current == target {
            return true;
        }

        dfs((current.0 + 1, current.1), target, seen, blocked)
            || dfs((current.0 - 1, current.1), target, seen, blocked)
            || dfs((current.0, current.1 + 1), target, seen, blocked)
            || dfs((current.0, current.1 - 1), target, seen, blocked)
    }

    let blocked: HashSet<_> = blocked
        .into_iter()
        .map(|point| (point[0], point[1]))
        .collect();
    let source = (source[0], source[1]);
    let target = (target[0], target[1]);
    dfs(source, target, &mut HashSet::new(), &blocked)
        && dfs(target, source, &mut HashSet::new(), &blocked)
}

#[cfg(test)]
mod tests {
    use crate::cell_vec;

    use super::*;

    #[test]
    fn determines_when_maze_can_be_escaped() {
        assert!(is_escape_possible(vec![], vec![0, 0], vec![999999, 999999]))
    }

    #[test]
    fn determines_when_maze_can_not_be_escaped() {
        assert!(!is_escape_possible(
            cell_vec![(0, 1), (1, 0)],
            vec![0, 0],
            vec![0, 2]
        ))
    }
}
