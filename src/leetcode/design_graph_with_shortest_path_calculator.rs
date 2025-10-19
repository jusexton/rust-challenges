use std::{cmp::Reverse, collections::BinaryHeap};

struct Graph {
    nodes: Vec<Vec<(usize, i32)>>,
}

impl Graph {
    fn new(n: i32, edges: Vec<Vec<i32>>) -> Self {
        let mut nodes = vec![vec![]; n as usize];
        for edge in edges {
            nodes[edge[0] as usize].push((edge[1] as usize, edge[2]));
        }
        Self { nodes }
    }

    fn add_edge(&mut self, edge: Vec<i32>) {
        self.nodes[edge[0] as usize].push((edge[1] as usize, edge[2]));
    }

    fn shortest_path(&self, src: i32, dest: i32) -> i32 {
        let (src, dest) = (src as usize, dest as usize);
        let mut to_visit = BinaryHeap::from([Reverse((0, src))]);
        let mut distances = vec![i32::MAX; self.nodes.len()];
        distances[src] = 0;

        while let Some(Reverse((cost, current_node))) = to_visit.pop() {
            if current_node == dest {
                return cost;
            }

            for (neighbor, dist) in &self.nodes[current_node] {
                let new_cost = dist + cost;
                if new_cost < distances[*neighbor] {
                    distances[*neighbor] = new_cost;
                    to_visit.push(Reverse((new_cost, *neighbor)));
                }
            }
        }

        -1
    }
}
