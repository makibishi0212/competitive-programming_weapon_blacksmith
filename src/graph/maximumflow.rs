// 未実装
use cargo_snippet::snippet;

struct MaximumFlow {
    graph: Vec<Vec<Edge>>,
}

struct Edge {
    to: usize,
    rev: usize,
    capacity: i128,
    is_reversed: bool, // 逆辺かどうか
}

// Dinic法
impl MaximumFlow {
    fn new(n: usize) -> MaximumFlow {
        let mut graph: Vec<Vec<Edge>> = Vec::new();
        for _ in 0..n {
            graph.push(Vec::new());
        }

        MaximumFlow { graph }
    }

    //fn dfs(&mut self) -> i64 {}

    fn bfs(&mut self, s: usize) -> Vec<i32> {
        let v = self.graph.len();
        let mut level = vec![-1; v];
        level[s] = 0;
        let mut deque = std::collections::VecDeque::new();
        deque.push_back(s);
        while let Some(v) = deque.pop_front() {
            for e in self.graph[v].iter() {
                if e.capacity > 0 && level[e.to] < 0 {
                    level[e.to] = level[v] + 1;
                    deque.push_back(e.to);
                }
            }
        }

        level
    }

    fn add_edge(&mut self, from: usize, to: usize, capacity: i128) {
        let to_len = self.graph[to].len();
        let from_len = self.graph[from].len();
        self.graph[from].push(Edge {
            to,
            rev: to_len,
            is_reversed: false,
            capacity,
        });
        self.graph[to].push(Edge {
            to: from,
            rev: from_len,
            is_reversed: true,
            capacity: 0,
        });
    }

    fn flow(&mut self, s: usize, t: usize) {}
}
