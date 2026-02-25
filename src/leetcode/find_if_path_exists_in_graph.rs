// Depth First Search
// pub fn valid_path(n: i32, edges: Vec<Vec<i32>>, source: i32, destination: i32) -> bool {
//     if source == destination {
//         return true;
//     }

//     let mut graph = vec![vec![]; n as usize];
//     for edge in edges {
//         graph[edge[0] as usize].push(edge[1]);
//         graph[edge[1] as usize].push(edge[0]);
//     }

//     let mut seen = HashSet::new();
//     let mut stack = vec![source];
//     seen.insert(source);
//     while let Some(current) = stack.pop() {
//         if current == destination {
//             return true;
//         }

//         for &neighbor in &graph[current as usize] {
//             if seen.insert(neighbor) {
//                 stack.push(neighbor);
//             }
//         }
//     }

//     false
// }

// Disjoint Set Union
pub fn valid_path(n: i32, edges: Vec<Vec<i32>>, source: i32, destination: i32) -> bool {
    if source == destination {
        return true;
    }

    let mut parent: Vec<i32> = (0..n).collect();

    fn find(i: i32, parent: &mut Vec<i32>) -> i32 {
        if parent[i as usize] == i {
            return i;
        }
        parent[i as usize] = find(parent[i as usize], parent);
        parent[i as usize]
    }

    // Union groups together
    for edge in edges {
        let root_a = find(edge[0], &mut parent);
        let root_b = find(edge[1], &mut parent);
        if root_a != root_b {
            parent[root_a as usize] = root_b;
        }
    }

    // Does source belong to same group as destination?
    find(source, &mut parent) == find(destination, &mut parent)
}

#[cfg(test)]
mod tests {
    use crate::leetcode::find_if_path_exists_in_graph::valid_path;

    #[test]
    fn test_valid_path() {
        assert!(valid_path(
            3,
            vec![vec![0, 1], vec![1, 2], vec![2, 0]],
            0,
            2
        ));
        assert!(!valid_path(
            6,
            vec![vec![0, 1], vec![0, 2], vec![3, 5], vec![5, 4], vec![4, 3]],
            0,
            5
        ));
    }
}
