use cargo_snippet::snippet;

struct MaximumFlow {
    graph: Vec<Vec<Edge>>,
}

struct Edge {
    to: usize,
    rev: usize,
    capacity: i64,
    is_reversed: bool, // 逆辺かどうか
}

// Dinic法
impl MaximumFlow {
    fn new(n: usize) -> MaximumFlow {
        let mut graph: Vec<Vec<Edge>> = Vec::new();
        for _ in 0..n {
            graph.push(Vec::new());
        }

        MaximumFlow { graph: graph }
    }

    //fn dfs(&mut self) -> i64 {}

    //fn bfs(&mut self) -> i64 {}

    fn add_edge(&mut self) {}

    fn flow(&mut self, s: usize, t: usize) {}
}
