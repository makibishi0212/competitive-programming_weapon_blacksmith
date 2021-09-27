use cargo_snippet::snippet;

// 多重辺を含むグラフ。頂点は0-indexed。自己ループ辺もok
#[snippet("@SimpleGraph")]
pub struct SimpleGraph<T> {
    size: usize,
    edges: Vec<Vec<(usize, T)>>,
    directed: bool,
}

#[snippet("@SimpleGraph")]
impl<T: std::marker::Copy + std::cmp::PartialOrd> SimpleGraph<T> {
    // directed: 有向グラフにするかどうか
    pub fn new(n: usize, directed: bool) -> SimpleGraph<T> {
        SimpleGraph::<T> {
            size: n,
            edges: vec![vec![]; n],
            directed,
        }
    }

    // nodeに隣接する頂点を列挙
    pub fn adjacent_nodes(&self, node: usize) -> Vec<usize> {
        self.edges[node]
            .iter()
            .map(|&(adj_node, _)| adj_node)
            .collect()
    }

    // 辺の追加 O(1)
    pub fn add_edge(&mut self, from: usize, to: usize, cost: T) {
        self.edges[from].push((to, cost));

        // 無向グラフなら、反対にも辺を張る
        if !self.directed {
            self.edges[to].push((from, cost));
        }
    }

    // 辺の削除 O(d(from)+d(to))
    pub fn remove_edge(&mut self, from: usize, to: usize) {
        let mut remove_index = std::usize::MAX;
        self.edges[from]
            .iter()
            .enumerate()
            .for_each(|(index, &(from_adj, _))| {
                if from_adj == to {
                    remove_index = index;
                }
            });
        if remove_index == std::usize::MAX {
            panic!();
        }

        self.edges[from].remove(remove_index);

        if self.directed {
            return;
        }

        remove_index = std::usize::MAX;
        self.edges[to]
            .iter()
            .enumerate()
            .for_each(|(index, &(to_adj, _))| {
                if to_adj == from {
                    remove_index = index;
                }
            });
        if remove_index == std::usize::MAX {
            panic!();
        }

        self.edges[to].remove(remove_index);
    }

    // 頂点のトポロジカルソート
    pub fn topological_sort(&self) -> Vec<usize> {
        let mut result = vec![];
        let mut isolated = std::collections::VecDeque::new();

        // deg[x] = a -> x, b -> xのような辺の数
        let mut degs = vec![0; self.size];

        let isolate_deg = if self.directed { 0 } else { 1 };

        self.edges.iter().enumerate().for_each(|(from, v_edges)| {
            v_edges.iter().for_each(|&(to, _)| {
                // 多重辺は無視
                if from != to {
                    degs[to] += 1;
                }
            });
        });

        degs.iter().enumerate().for_each(|(index, &deg)| {
            if deg == isolate_deg {
                isolated.push_front(index);
            }
        });

        while isolated.len() != 0 {
            let next = isolated.pop_back().unwrap();
            result.push(next);
            for &(to, _) in &self.edges[next] {
                degs[to] -= 1;
                if degs[to] == isolate_deg {
                    isolated.push_front(to);
                }
            }
        }

        result
    }

    /// return pair of (# of scc, scc id)
    fn scc_ids(&self) -> (usize, Vec<usize>) {
        // Compressed sparse row?
        pub struct Csr {
            start: Vec<usize>,
            elist: Vec<usize>,
        }

        impl Csr {
            pub fn new(n: usize, edges: &Vec<Vec<usize>>) -> Self {
                let mut e_count = 0;
                edges.iter().for_each(|edges_f| {
                    e_count += edges_f.len();
                });

                let mut csr = Csr {
                    start: vec![0; n + 1],
                    elist: vec![0; e_count],
                };
                for i in 0..edges.len() {
                    let from = i;
                    let edges_f = &edges[from];
                    for _ in edges_f.iter() {
                        csr.start[from + 1] += 1;
                    }
                }

                for i in 1..=n {
                    csr.start[i] += csr.start[i - 1];
                }

                let mut counter = csr.start.clone();
                for i in 0..edges.len() {
                    let edges_f = &edges[i];
                    let from = i;
                    for to in edges_f.iter() {
                        csr.elist[counter[from]] = *to;
                        counter[from] += 1;
                    }
                }

                csr
            }
        }

        struct _Env {
            graph: Csr,
            now_ord: usize,
            group_num: usize,
            visited: Vec<usize>,
            low: Vec<usize>,
            ord: Vec<Option<usize>>,
            ids: Vec<usize>,
        }

        // pure_edges[from] = [to1,to2,to3]
        let pure_edges: Vec<Vec<usize>> = self
            .edges
            .iter()
            .map(|edges_f| edges_f.iter().map(|e| e.0).collect())
            .collect();

        let mut env = _Env {
            graph: Csr::new(self.size, &pure_edges),
            now_ord: 0,
            group_num: 0,
            visited: Vec::with_capacity(self.size),
            low: vec![0; self.size],
            ord: vec![None; self.size],
            ids: vec![0; self.size],
        };

        fn dfs(v: usize, n: usize, env: &mut _Env) {
            env.low[v] = env.now_ord;
            env.ord[v] = Some(env.now_ord);
            env.now_ord += 1;
            env.visited.push(v);

            for i in env.graph.start[v]..env.graph.start[v + 1] {
                let to = env.graph.elist[i];
                if let Some(x) = env.ord[to] {
                    env.low[v] = std::cmp::min(env.low[v], x);
                } else {
                    dfs(to, n, env);
                    env.low[v] = std::cmp::min(env.low[v], env.low[to]);
                }
            }
            if env.low[v] == env.ord[v].unwrap() {
                loop {
                    let u = *env.visited.last().unwrap();
                    env.visited.pop();
                    env.ord[u] = Some(n);
                    env.ids[u] = env.group_num;
                    if u == v {
                        break;
                    }
                }
                env.group_num += 1;
            }
        }
        for i in 0..self.size {
            if env.ord[i].is_none() {
                dfs(i, self.size, &mut env);
            }
        }
        for x in env.ids.iter_mut() {
            *x = env.group_num - 1 - *x;
        }
        (env.group_num, env.ids)
    }

    pub fn scc(&self) -> Vec<Vec<usize>> {
        let ids = self.scc_ids();
        let group_num = ids.0;
        let mut counts = vec![0usize; group_num];
        for &x in ids.1.iter() {
            counts[x] += 1;
        }
        let mut groups: Vec<Vec<usize>> = (0..ids.0).map(|_| vec![]).collect();
        for i in 0..group_num {
            groups[i].reserve(counts[i]);
        }
        for i in 0..self.size {
            groups[ids.1[i]].push(i);
        }
        groups
    }

    pub fn euler_tour(&self, start: usize) -> Vec<usize> {
        let mut visits = vec![];

        let mut que = std::collections::VecDeque::new();
        que.push_back(start);

        let mut visited = vec![std::usize::MAX; self.size];
        visited[start] = start;

        while !que.is_empty() {
            let v = que.pop_back().unwrap();
            visits.push(v);

            let dead_end = self.edges[v]
                .iter()
                .all(|e| visited[e.0] != std::usize::MAX);

            if dead_end {
                if v != start {
                    que.push_back(visited[v]);
                }

                continue;
            }

            for i in 0..self.edges[v].len() {
                let e = self.edges[v][i];
                if visited[e.0] == std::usize::MAX {
                    visited[e.0] = v;
                    que.push_back(e.0);
                    break;
                }
            }
        }

        visits
    }
}

// 負辺を含まないグラフのためのメソッド
#[snippet("@SimpleGraph")]
impl SimpleGraph<usize> {
    // Dijkstraで1対nの最小距離を求める(vec[from]は自己ループ辺のコストになる) usize::MAXを超えるものはusize::MAXとして扱う
    pub fn min_dists(&self, from: usize) -> Vec<usize> {
        let mut from_to_n = vec![std::usize::MAX; self.size];
        let mut queue = std::collections::BinaryHeap::new();
        queue.push(std::cmp::Reverse((0, from)));

        while !queue.is_empty() {
            let (cost, to) = queue.pop().unwrap().0;
            if from_to_n[to] < cost {
                continue;
            }

            for &(to_to, to_cost) in &self.edges[to] {
                let new_cost = cost.saturating_add(to_cost);
                if from_to_n[to_to] > new_cost {
                    from_to_n[to_to] = new_cost;
                    queue.push(std::cmp::Reverse((new_cost, to_to)));
                }
            }
        }

        from_to_n
    }

    pub fn all_min_dists(&self) -> Vec<Vec<usize>> {
        (0..self.size).map(|node| self.min_dists(node)).collect()
    }
}

// 負辺を含むグラフのためのメソッド
#[snippet("@SimpleGraph")]
impl SimpleGraph<i64> {
    // Bellman-Ford法で1対nの最小距離を求める。負閉路を検出した場合はNone
    // このメソッドでは自己ループ辺を無視する。
    pub fn min_dists(&self, from: usize) -> Option<Vec<i64>> {
        let mut from_to_n = vec![std::i64::MAX; self.size];
        from_to_n[from] = 0;
        for _ in 0..self.size {
            self.edges.iter().enumerate().for_each(|(from_v, v_edges)| {
                v_edges.iter().for_each(|&(to_v, cost)| {
                    if from_to_n[from_v] != std::i64::MAX {
                        from_to_n[to_v] = std::cmp::min(from_to_n[to_v], from_to_n[from_v] + cost);
                    }
                });
            });
        }

        let dist_total_before: i64 = from_to_n
            .iter()
            .filter(|&&dist| dist != std::i64::MAX)
            .map(|dist| *dist)
            .sum();

        // もう一周行う
        self.edges.iter().enumerate().for_each(|(from_v, v_edges)| {
            v_edges.iter().for_each(|&(to_v, cost)| {
                if from_to_n[from_v] != std::i64::MAX {
                    from_to_n[to_v] = std::cmp::min(from_to_n[to_v], from_to_n[from_v] + cost);
                }
            });
        });

        let dist_total_after: i64 = from_to_n
            .iter()
            .filter(|&&dist| dist != std::i64::MAX)
            .map(|dist| *dist)
            .sum();

        if dist_total_before != dist_total_after {
            None
        } else {
            Some(from_to_n)
        }
    }
}

mod test {
    use super::*;

    #[test]
    fn add_edge_directed_graph_test() {
        let mut graph = SimpleGraph::<usize>::new(5, true);
        graph.add_edge(1, 0, 1);
        graph.add_edge(1, 2, 1);
        graph.add_edge(1, 3, 1);
        graph.add_edge(1, 4, 1);

        assert_eq!(graph.adjacent_nodes(1), vec![0, 2, 3, 4]);
        assert_eq!(graph.adjacent_nodes(0), vec![]);
        assert_eq!(graph.adjacent_nodes(2), vec![]);
        assert_eq!(graph.adjacent_nodes(3), vec![]);
        assert_eq!(graph.adjacent_nodes(4), vec![]);
    }

    #[test]
    fn remove_edge_directed_graph_test() {
        let mut graph = SimpleGraph::<usize>::new(5, true);
        graph.add_edge(1, 0, 1);
        graph.add_edge(1, 2, 1);
        graph.add_edge(1, 3, 1);
        graph.add_edge(1, 4, 1);

        graph.remove_edge(1, 0);
        graph.remove_edge(1, 3);

        assert_eq!(graph.adjacent_nodes(1), vec![2, 4]);
    }

    #[test]
    fn add_edge_undirected_graph_test() {
        let mut graph = SimpleGraph::<usize>::new(5, false);
        graph.add_edge(1, 0, 1);
        graph.add_edge(1, 2, 1);
        graph.add_edge(1, 3, 1);
        graph.add_edge(1, 4, 1);

        assert_eq!(graph.adjacent_nodes(1), vec![0, 2, 3, 4]);
        assert_eq!(graph.adjacent_nodes(0), vec![1]);
        assert_eq!(graph.adjacent_nodes(2), vec![1]);
        assert_eq!(graph.adjacent_nodes(3), vec![1]);
        assert_eq!(graph.adjacent_nodes(4), vec![1]);
    }

    #[test]
    fn remove_edge_undirected_graph_test() {
        let mut graph = SimpleGraph::<usize>::new(5, false);
        graph.add_edge(1, 0, 1);
        graph.add_edge(1, 2, 1);
        graph.add_edge(1, 3, 1);
        graph.add_edge(1, 4, 1);

        graph.remove_edge(1, 0);
        graph.remove_edge(1, 3);

        assert_eq!(graph.adjacent_nodes(1), vec![2, 4]);
        assert_eq!(graph.adjacent_nodes(0), vec![]);
        assert_eq!(graph.adjacent_nodes(2), vec![1]);
        assert_eq!(graph.adjacent_nodes(3), vec![]);
        assert_eq!(graph.adjacent_nodes(4), vec![1]);
    }

    #[test]
    fn min_dists_test() {
        let mut graph = SimpleGraph::<usize>::new(4, true);
        graph.add_edge(0, 1, 100);
        graph.add_edge(1, 2, 100);
        graph.add_edge(2, 3, 150);
        graph.add_edge(3, 0, 3);

        assert_eq!(graph.min_dists(0), vec![353, 100, 200, 350]);
        assert_eq!(graph.min_dists(1), vec![253, 353, 100, 250]);
        assert_eq!(graph.min_dists(2), vec![153, 253, 353, 150]);
        assert_eq!(graph.min_dists(3), vec![3, 103, 203, 353]);

        assert_eq!(
            graph.all_min_dists(),
            vec![
                vec![353, 100, 200, 350],
                vec![253, 353, 100, 250],
                vec![153, 253, 353, 150],
                vec![3, 103, 203, 353]
            ]
        )
    }

    #[test]
    fn min_dists_large_number_test() {
        let mut graph = SimpleGraph::<usize>::new(4, true);
        graph.add_edge(0, 1, std::usize::MAX / 3 - 1);
        graph.add_edge(1, 2, std::usize::MAX / 3 - 1);
        graph.add_edge(2, 3, std::usize::MAX / 3 - 1);
        graph.add_edge(3, 0, std::usize::MAX / 3 - 1);

        assert_eq!(
            graph.min_dists(0),
            vec![
                std::usize::MAX,
                std::usize::MAX / 3 - 1,
                std::usize::MAX / 3 * 2 - 2,
                std::usize::MAX - 3
            ]
        );
    }

    #[test]
    fn min_dists_self_loop_case_test() {
        let mut graph = SimpleGraph::<usize>::new(4, true);
        graph.add_edge(0, 1, 100);
        graph.add_edge(1, 2, 100);
        graph.add_edge(2, 3, 150);
        graph.add_edge(3, 0, 3);

        // 上のテストに自己ループ辺を追加
        graph.add_edge(0, 0, 0);
        graph.add_edge(3, 3, 11);

        assert_eq!(graph.min_dists(0), vec![0, 100, 200, 350]);
        assert_eq!(graph.min_dists(1), vec![253, 353, 100, 250]);
        assert_eq!(graph.min_dists(2), vec![153, 253, 353, 150]);
        assert_eq!(graph.min_dists(3), vec![3, 103, 203, 11]);
    }

    #[test]
    fn min_dists_test2() {
        let mut graph = SimpleGraph::<usize>::new(5, true);
        graph.add_edge(0, 1, 90);
        graph.add_edge(1, 2, 180);
        graph.add_edge(0, 2, 150);
        graph.add_edge(0, 3, 40);
        graph.add_edge(3, 4, 9000);

        assert_eq!(
            graph.min_dists(0),
            vec![18446744073709551615, 90, 150, 40, 9040]
        );
        assert_eq!(
            graph.min_dists(1),
            vec![
                std::usize::MAX,
                18446744073709551615,
                180,
                18446744073709551615,
                18446744073709551615
            ]
        );
        assert_eq!(
            graph.min_dists(2),
            vec![
                18446744073709551615,
                18446744073709551615,
                18446744073709551615,
                18446744073709551615,
                18446744073709551615
            ]
        );
        assert_eq!(
            graph.min_dists(3),
            vec![
                18446744073709551615,
                18446744073709551615,
                18446744073709551615,
                18446744073709551615,
                9000
            ]
        );
        assert_eq!(
            graph.min_dists(4),
            vec![
                18446744073709551615,
                18446744073709551615,
                18446744073709551615,
                18446744073709551615,
                18446744073709551615
            ]
        );
    }

    #[test]
    fn min_dists_i64_test() {
        let mut graph = SimpleGraph::<i64>::new(5, true);
        graph.add_edge(0, 1, 90);
        graph.add_edge(1, 2, 180);
        graph.add_edge(0, 2, 150);
        graph.add_edge(0, 3, 40);
        graph.add_edge(3, 4, 9000);

        assert_eq!(graph.min_dists(0), Some(vec![0, 90, 150, 40, 9040]));
    }

    #[test]
    fn min_dists_i64_test2() {
        let mut graph = SimpleGraph::<i64>::new(5, true);
        graph.add_edge(0, 1, -50);
        graph.add_edge(1, 0, 70);
        graph.add_edge(1, 2, 180);
        graph.add_edge(0, 2, 150);
        graph.add_edge(0, 3, 40);
        graph.add_edge(3, 4, 9000);

        assert_eq!(graph.min_dists(0), Some(vec![0, -50, 130, 40, 9040]));
    }

    #[test]
    fn min_dists_i64_negative_cycle_test() {
        let mut graph = SimpleGraph::<i64>::new(3, true);
        graph.add_edge(0, 1, -70);
        graph.add_edge(1, 0, 69);
        graph.add_edge(0, 2, 200);

        assert_eq!(graph.min_dists(0), None);
    }

    #[test]
    fn min_dists_undirected_test() {
        // 上のテストを無向グラフにしたもの
        let mut graph = SimpleGraph::<usize>::new(5, false);
        graph.add_edge(0, 1, 90);
        graph.add_edge(1, 2, 180);
        graph.add_edge(0, 2, 150);
        graph.add_edge(0, 3, 40);
        graph.add_edge(3, 4, 9000);

        assert_eq!(graph.min_dists(0), vec![80, 90, 150, 40, 9040]);
        assert_eq!(graph.min_dists(1), vec![90, 180, 180, 130, 9130]);
        assert_eq!(graph.min_dists(2), vec![150, 180, 300, 190, 9190]);
        assert_eq!(graph.min_dists(3), vec![40, 130, 190, 80, 9000]);
        assert_eq!(graph.min_dists(4), vec![9040, 9130, 9190, 9000, 18000]);
    }

    #[test]
    fn topological_sort_directed_test() {
        let mut graph = SimpleGraph::<usize>::new(5, true);
        graph.add_edge(0, 1, 1);
        graph.add_edge(1, 2, 1);
        graph.add_edge(3, 2, 1);
        graph.add_edge(2, 4, 1);
        assert_eq!(graph.topological_sort(), vec![0, 3, 1, 2, 4]);

        let mut graph = SimpleGraph::<usize>::new(7, true);
        graph.add_edge(1, 4, 1);
        graph.add_edge(5, 2, 1);
        graph.add_edge(3, 0, 1);
        graph.add_edge(5, 5, 1);
        graph.add_edge(4, 1, 1);
        graph.add_edge(0, 3, 1);
        graph.add_edge(4, 2, 1);
        assert_eq!(graph.topological_sort(), vec![5, 6]);
    }

    #[test]
    fn topological_sort_undirected_test() {
        let mut graph = SimpleGraph::<usize>::new(4, false);
        graph.add_edge(2, 3, 1);
        graph.add_edge(0, 1, 1);
        graph.add_edge(1, 2, 1);
        assert_eq!(graph.topological_sort(), vec![0, 3, 1, 2]);
    }

    #[test]
    fn scc_test() {
        let mut graph = SimpleGraph::<usize>::new(8, false);
        graph.add_edge(0, 1, 1);
        graph.add_edge(2, 3, 1);
        graph.add_edge(4, 5, 1);
        graph.add_edge(5, 6, 1);
        graph.add_edge(6, 4, 1);
        let scc = graph.scc();
        assert_eq!(scc, vec![vec![7], vec![4, 5, 6], vec![2, 3], vec![0, 1]]);

        let mut directed_graph = SimpleGraph::<usize>::new(5, true);
        for i in 0..4 {
            directed_graph.add_edge(i, i + 1, 1);
        }
        let scc = directed_graph.scc();
        assert_eq!(scc, vec![vec![0], vec![1], vec![2], vec![3], vec![4]]);

        let mut undirected_graph = SimpleGraph::<usize>::new(5, false);
        for i in 0..4 {
            undirected_graph.add_edge(i, i + 1, 1);
        }
        let scc = undirected_graph.scc();
        assert_eq!(scc, vec![vec![0, 1, 2, 3, 4]]);

        let mut self_loop_graph = SimpleGraph::<usize>::new(5, true);
        self_loop_graph.add_edge(0, 0, 1);
        let scc = self_loop_graph.scc();
        assert_eq!(scc, vec![vec![4], vec![3], vec![2], vec![1], vec![0]]);
    }

    #[test]
    fn euler_tour_test() {
        let mut graph = SimpleGraph::<usize>::new(7, false);
        graph.add_edge(1, 2, 1);
        graph.add_edge(2, 3, 1);
        graph.add_edge(3, 4, 1);
        graph.add_edge(2, 5, 1);
        graph.add_edge(1, 6, 1);

        assert_eq!(graph.euler_tour(1), vec![1, 2, 3, 4, 3, 2, 5, 2, 1, 6, 1]);

        let mut graph = SimpleGraph::<usize>::new(4, true);
        graph.add_edge(0, 1, 1);
        graph.add_edge(1, 2, 1);
        graph.add_edge(2, 3, 1);
        graph.add_edge(3, 0, 1);
        assert_eq!(graph.euler_tour(0), vec![0, 1, 2, 3, 2, 1, 0]);

        let mut graph = SimpleGraph::<usize>::new(4, true);
        graph.add_edge(0, 1, 1);
        graph.add_edge(1, 2, 1);
        graph.add_edge(2, 0, 1);
        graph.add_edge(0, 3, 1);
        assert_eq!(graph.euler_tour(0), vec![0, 1, 2, 1, 0, 3, 0]);
    }
}
