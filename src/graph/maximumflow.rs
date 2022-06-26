use cargo_snippet::snippet;

#[snippet("@MaximumFlow")]
struct MaximumFlow {
    graph: Vec<Vec<Edge>>,
}

#[snippet("@MaximumFlow")]
struct Edge {
    to: usize,
    to_edge_index: usize,
    capacity: i128,
}

#[snippet("@MaximumFlow")]

// Dinic法
impl MaximumFlow {
    fn new(n: usize) -> MaximumFlow {
        let mut graph: Vec<Vec<Edge>> = Vec::new();
        for _ in 0..n {
            graph.push(Vec::new());
        }

        MaximumFlow { graph }
    }

    fn bfs(&mut self, s: usize) -> Vec<i128> {
        let mut level = vec![-1; self.graph.len()];
        level[s] = 0;
        let mut que = std::collections::VecDeque::new();
        que.push_back(s);
        while let Some(v) = que.pop_front() {
            for e in self.graph[v].iter() {
                if e.capacity > 0 && level[e.to] < 0 {
                    level[e.to] = level[v] + 1;
                    que.push_back(e.to);
                }
            }
        }

        level
    }

    fn dfs(
        &mut self,
        from: usize,
        to: usize,
        level: &[i128],
        edge_used: &mut [usize],
        flow: i128,
    ) -> i128 {
        if from == to {
            return flow;
        }

        while edge_used[from] < self.graph[from].len() {
            let edge_index = edge_used[from];

            let flow = std::cmp::min(flow, self.graph[from][edge_index].capacity);
            let edge_to = self.graph[from][edge_index].to;

            if flow > 0 && level[from] < level[edge_to] {
                let child_flow = self.dfs(edge_to, to, level, edge_used, flow);
                if child_flow > 0 {
                    let rev_edge_index = self.graph[from][edge_index].to_edge_index;
                    self.graph[from][edge_index].capacity -= child_flow;
                    self.graph[edge_to][rev_edge_index].capacity += child_flow;
                    return child_flow;
                }
            }

            edge_used[from] += 1;
        }

        0
    }

    pub fn add_edge(&mut self, from: usize, to: usize, capacity: i128) {
        let to_len = self.graph[to].len();
        let from_len = self.graph[from].len();
        self.graph[from].push(Edge {
            to,
            to_edge_index: to_len,
            capacity,
        });

        // 逆辺
        self.graph[to].push(Edge {
            to: from,
            to_edge_index: from_len,
            capacity: 0,
        });
    }

    pub fn maximum_flow(&mut self, from: usize, to: usize) -> i128 {
        let mut flow: i128 = 0;

        loop {
            let level = self.bfs(from);
            if level[to] < 0 {
                return flow;
            }
            let mut edge_used = vec![0; self.graph.len()];
            loop {
                let one_path_flow = self.dfs(from, to, &level, &mut edge_used, std::i128::MAX);
                if one_path_flow == 0 {
                    break;
                }
                flow += one_path_flow;
            }
        }
    }
}

mod test {
    use super::*;

    #[test]
    fn maxflow_test() {
        let mut graph = MaximumFlow::new(3);

        graph.add_edge(0, 1, 999999999);
        graph.add_edge(1, 2, 5);

        let flow = graph.maximum_flow(0, 2);
        assert_eq!(flow, 5);

        let mut graph = MaximumFlow::new(4);

        graph.add_edge(0, 1, 6);
        graph.add_edge(0, 2, 4);
        graph.add_edge(1, 2, 1);
        graph.add_edge(1, 3, 8);
        graph.add_edge(2, 3, 5);

        let flow = graph.maximum_flow(0, 3);
        assert_eq!(flow, 10);
    }
}
