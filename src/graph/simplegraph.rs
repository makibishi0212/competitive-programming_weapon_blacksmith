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
            edges: vec![Vec::with_capacity(n); n],
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
        let mut remove_index = usize::MAX;
        self.edges[from]
            .iter()
            .enumerate()
            .for_each(|(index, &(from_adj, _))| {
                if from_adj == to {
                    remove_index = index;
                }
            });
        if remove_index == usize::MAX {
            panic!();
        }

        self.edges[from].remove(remove_index);

        if self.directed {
            return;
        }

        remove_index = usize::MAX;
        self.edges[to]
            .iter()
            .enumerate()
            .for_each(|(index, &(to_adj, _))| {
                if to_adj == from {
                    remove_index = index;
                }
            });
        if remove_index == usize::MAX {
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

        self.edges.iter().enumerate().for_each(|(_, v_edges)| {
            v_edges.iter().for_each(|&(to, _)| {
                degs[to] += 1;
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

    pub fn scc_decomposition(&self) -> Vec<Vec<usize>> {
        let mut num = vec![0; self.size];
        let mut low = vec![0; self.size];
        let mut stack: std::collections::VecDeque<usize> =
            std::collections::VecDeque::<usize>::new();
        let mut inStack = vec![false; self.size];

        let mut scc: Vec<Vec<usize>> = vec![];

        struct VisitEnv<'a, T> {
            scc: &'a mut Vec<Vec<usize>>,
            stack: &'a mut std::collections::VecDeque<usize>,
            inStack: &'a mut Vec<bool>,
            low: &'a mut Vec<usize>,
            num: &'a mut Vec<usize>,
            graph: &'a SimpleGraph<T>,
        };

        fn visit<T>(env: &mut VisitEnv<T>, v: usize, depth: usize) {
            env.low[v] = depth;
            env.num[v] = depth;
            let new_depth = depth + 1;

            env.stack.push_back(v);
            env.inStack[v] = true;

            env.graph.edges[v].iter().for_each(|&(to, _)| {
                if env.num[to] == 0 {
                    visit(env, to, new_depth);
                    env.low[v] = std::cmp::min(env.low[v], env.low[to]);
                } else if env.inStack[to] {
                    env.low[v] = std::cmp::min(env.low[v], env.num[to]);
                }
            });

            if env.low[v] == env.num[v] {
                let mut vertexes = vec![];

                loop {
                    let to = env.stack.pop_back().unwrap();
                    env.inStack[to] = false;
                    vertexes.push(to);

                    if to == v {
                        env.scc.push(vertexes);
                        break;
                    }
                }
            }
        };

        let mut env = VisitEnv::<T> {
            scc: &mut scc,
            stack: &mut stack,
            inStack: &mut inStack,
            low: &mut low,
            num: &mut num,
            graph: self,
        };

        for i in 0..self.size {
            let i_num = env.num[i];
            if i_num == 0 {
                visit(&mut env, i, 0);
            }
        }

        scc
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
    fn scc_decomposition_test() {
        let mut graph = SimpleGraph::<usize>::new(8, false);
        graph.add_edge(0, 1, 1);
        graph.add_edge(2, 3, 1);
        graph.add_edge(4, 5, 1);
        graph.add_edge(5, 6, 1);
        graph.add_edge(6, 4, 1);
        let scc = graph.scc_decomposition();
        assert_eq!(scc, vec![vec![0, 1], vec![2, 3], vec![6, 4, 5], vec![7]]);

        let mut directed_graph = SimpleGraph::<usize>::new(5, true);
        for i in 0..4 {
            directed_graph.add_edge(i, i + 1, 1);
        }
        let scc = directed_graph.scc_decomposition();
        assert_eq!(scc, vec![vec![4], vec![3], vec![2], vec![1], vec![0]]);

        let mut undirected_graph = SimpleGraph::<usize>::new(5, false);
        for i in 0..4 {
            undirected_graph.add_edge(i, i + 1, 1);
        }
        let scc = undirected_graph.scc_decomposition();
        assert_eq!(scc, vec![vec![4, 3, 2, 0, 1]]);
    }
}
