// TODO: ワーシャルフロイド

use crate::data_structure::union_find::UnionFind;
use cargo_snippet::snippet;

#[snippet("@Graph")]
pub struct Graph<T> {
    size: usize,
    edges: Vec<(usize, usize, T)>,
    directed: bool,
}

#[snippet("@Graph")]
impl<T: std::marker::Copy + std::cmp::PartialOrd> Graph<T> {
    // directed: 有向グラフにするかどうか
    pub fn new(n: usize, directed: bool) -> Graph<T> {
        Graph::<T> {
            size: n,
            edges: vec![],
            directed,
        }
    }

    // nodeに隣接する頂点を列挙
    pub fn adjacent_nodes(&self, node: usize) -> Vec<usize> {
        self.edges
            .iter()
            .filter(|&&(from, _, _)| from == node)
            .map(|&(_, to, _)| to)
            .collect()
    }

    // 辺の追加 O(1)
    pub fn add_edge(&mut self, from: usize, to: usize, cost: T) {
        self.edges.push((from, to, cost));

        // 無向グラフなら、反対にも辺を張る
        if !self.directed {
            self.edges.push((to, from, cost));
        }
    }

    // 頂点のトポロジカルソート
    pub fn topological_sort(&self) -> Vec<usize> {
        let mut result = vec![];
        let mut isolated = std::collections::VecDeque::new();

        // deg[x] = a -> x, b -> xのような辺の数
        let mut degs = vec![0; self.size];

        let isolate_deg = if self.directed { 0 } else { 1 };

        self.edges.iter().for_each(|&(from, to, _)| {
            // 多重辺は無視
            if from != to {
                degs[to] += 1;
            }
        });

        degs.iter().enumerate().for_each(|(index, &deg)| {
            if deg == isolate_deg {
                isolated.push_front(index);
            }
        });

        let mut edges_from = vec![vec![]; self.size];
        self.edges.iter().for_each(|&(from, to, cost)| {
            edges_from[from].push((to, cost));
        });

        while !isolated.is_empty() {
            let next = isolated.pop_back().unwrap();
            result.push(next);
            for &(to, _) in &edges_from[next] {
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
            pub fn new(n: usize, edges: &[Vec<usize>]) -> Self {
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
        let mut pure_edges: Vec<Vec<usize>> = vec![vec![]; self.size];
        self.edges.iter().for_each(|&(from, to, _)| {
            pure_edges[from].push(to);
        });

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

        let mut edges_from = vec![vec![]; self.size];
        self.edges.iter().for_each(|&(from, to, cost)| {
            edges_from[from].push((to, cost));
        });

        while !que.is_empty() {
            let v = que.pop_back().unwrap();
            visits.push(v);

            let dead_end = edges_from[v]
                .iter()
                .all(|e| visited[e.0] != std::usize::MAX);

            if dead_end {
                if v != start {
                    que.push_back(visited[v]);
                }

                continue;
            }

            for i in 0..edges_from[v].len() {
                let e = edges_from[v][i];
                if visited[e.0] == std::usize::MAX {
                    visited[e.0] = v;
                    que.push_back(e.0);
                    break;
                }
            }
        }

        visits
    }

    pub fn is_tree(&self) -> bool {
        if self.directed {
            return false;
        }

        if self.size == 0 {
            return true;
        }

        let mut que = std::collections::VecDeque::new();
        que.push_back((0usize, 0usize));

        let mut visited = vec![false; self.size];

        let mut is_tree = true;

        let mut edges_from = vec![vec![]; self.size];
        self.edges.iter().for_each(|&(from, to, cost)| {
            edges_from[from].push((to, cost));
        });

        while !que.is_empty() {
            let (now, before) = que.pop_back().unwrap();
            if visited[now] {
                is_tree = false;
                break;
            } else {
                visited[now] = true;
            }

            edges_from[now].iter().for_each(|&(to, _)| {
                if to != before {
                    que.push_back((to, now));
                }
            });
        }

        if !is_tree {
            return false;
        }

        is_tree = visited.iter().all(|&visited_node| {
            return visited_node;
        });

        return is_tree;
    }
}

#[snippet("@Graph")]
// 負辺を含まないグラフのためのメソッド
impl<T: Copy + num::Unsigned + num::Bounded + num::Saturating + std::cmp::Ord> Graph<T> {
    // Dijkstraで1対nの最小距離を求める(vec[from]は自己ループ辺のコストになる) usize::MAXを超えるものはusize::MAXとして扱う
    pub fn min_dists(&self, from: usize) -> Vec<T> {
        let mut from_to_n = vec![T::max_value(); self.size];
        let mut queue: std::collections::BinaryHeap<std::cmp::Reverse<(T, usize)>> =
            std::collections::BinaryHeap::new();
        queue.push(std::cmp::Reverse((T::zero(), from)));

        let mut edges_from = vec![vec![]; self.size];
        self.edges.iter().for_each(|&(from, to, cost)| {
            edges_from[from].push((to, cost));
        });

        while !queue.is_empty() {
            let (cost, to) = queue.pop().unwrap().0;
            if from_to_n[to] < cost {
                continue;
            }

            for &(to_to, to_cost) in &edges_from[to] {
                let new_cost = cost.saturating_add(to_cost);
                if from_to_n[to_to] > new_cost {
                    from_to_n[to_to] = new_cost;
                    queue.push(std::cmp::Reverse((new_cost, to_to)));
                }
            }
        }

        from_to_n
    }

    pub fn all_min_dists(&self) -> Vec<Vec<T>> {
        (0..self.size).map(|node| self.min_dists(node)).collect()
    }

    pub fn mst(&self) -> Vec<usize> {
        let mut sorted_edges = vec![];
        self.edges
            .iter()
            .enumerate()
            .for_each(|(e_index, &(from, to, cost))| sorted_edges.push((cost, from, to, e_index)));
        sorted_edges.sort();

        let mut uf = UnionFind::new(self.size);
        let mut adopt_edge_indexes = vec![];

        sorted_edges.iter().for_each(|&(_, from, to, e_index)| {
            if !uf.same(from, to) {
                uf.unite(from, to);
                adopt_edge_indexes.push(e_index);
            }
        });

        adopt_edge_indexes
    }
}

#[snippet("@Graph")]
// 負辺を含むグラフのためのメソッド
impl<T: Copy + num::Signed + num::Bounded + std::ops::AddAssign + std::cmp::Ord> Graph<T> {
    // Bellman-Ford法で1対nの最小距離を求める。負閉路を検出した場合はNone
    // このメソッドでは自己ループ辺を無視する。
    pub fn min_dists_or_detect_negative_loop(&self, from: usize) -> Option<Vec<T>> {
        let mut from_to_n = vec![T::max_value(); self.size];
        from_to_n[from] = T::zero();
        for _ in 0..self.size {
            self.edges.iter().for_each(|&(from, to, cost)| {
                if from_to_n[from] != T::max_value() {
                    from_to_n[to] = std::cmp::min(from_to_n[to], from_to_n[from] + cost);
                }
            });
        }

        let mut dist_total_before: T = T::zero();

        from_to_n
            .iter()
            .filter(|&&dist| dist != T::max_value())
            .map(|dist| *dist)
            .for_each(|x| {
                dist_total_before += x;
            });

        // もう一周行う
        self.edges.iter().for_each(|&(from, to, cost)| {
            if from_to_n[from] != T::max_value() {
                from_to_n[to] = std::cmp::min(from_to_n[to], from_to_n[from] + cost);
            }
        });

        let mut dist_total_after: T = T::zero();

        from_to_n
            .iter()
            .filter(|&&dist| dist != T::max_value())
            .map(|dist| *dist)
            .for_each(|x| {
                dist_total_after += x;
            });

        if dist_total_before != dist_total_after {
            None
        } else {
            Some(from_to_n)
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use proptest::prelude::*;
    use rand::prelude::*;

    #[test]
    fn add_edge_directed_graph_test() {
        let mut graph = Graph::<usize>::new(5, true);
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
    fn add_edge_undirected_graph_test() {
        let mut graph = Graph::<usize>::new(5, false);
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
    fn min_dists_test() {
        let mut graph = Graph::<usize>::new(4, true);
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
        let mut graph = Graph::<usize>::new(4, true);
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
        let mut graph = Graph::<usize>::new(4, true);
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
        let mut graph = Graph::<usize>::new(5, true);
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
        let mut graph = Graph::<i64>::new(5, true);
        graph.add_edge(0, 1, 90);
        graph.add_edge(1, 2, 180);
        graph.add_edge(0, 2, 150);
        graph.add_edge(0, 3, 40);
        graph.add_edge(3, 4, 9000);

        assert_eq!(
            graph.min_dists_or_detect_negative_loop(0),
            Some(vec![0, 90, 150, 40, 9040])
        );
    }

    #[test]
    fn min_dists_i64_test2() {
        let mut graph = Graph::<i64>::new(5, true);
        graph.add_edge(0, 1, -50);
        graph.add_edge(1, 0, 70);
        graph.add_edge(1, 2, 180);
        graph.add_edge(0, 2, 150);
        graph.add_edge(0, 3, 40);
        graph.add_edge(3, 4, 9000);

        assert_eq!(
            graph.min_dists_or_detect_negative_loop(0),
            Some(vec![0, -50, 130, 40, 9040])
        );
    }

    #[test]
    fn min_dists_i64_negative_cycle_test() {
        let mut graph = Graph::<i64>::new(3, true);
        graph.add_edge(0, 1, -70);
        graph.add_edge(1, 0, 69);
        graph.add_edge(0, 2, 200);

        assert_eq!(graph.min_dists_or_detect_negative_loop(0), None);
    }

    #[test]
    fn min_dists_undirected_test() {
        // 上のテストを無向グラフにしたもの
        let mut graph = Graph::<usize>::new(5, false);
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
        let mut graph = Graph::<usize>::new(5, true);
        graph.add_edge(0, 1, 1);
        graph.add_edge(1, 2, 1);
        graph.add_edge(3, 2, 1);
        graph.add_edge(2, 4, 1);
        assert_eq!(graph.topological_sort(), vec![0, 3, 1, 2, 4]);

        let mut graph = Graph::<usize>::new(7, true);
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
        let mut graph = Graph::<usize>::new(4, false);
        graph.add_edge(2, 3, 1);
        graph.add_edge(0, 1, 1);
        graph.add_edge(1, 2, 1);
        assert_eq!(graph.topological_sort(), vec![0, 3, 1, 2]);
    }

    #[test]
    fn scc_test() {
        let mut graph = Graph::<usize>::new(8, false);
        graph.add_edge(0, 1, 1);
        graph.add_edge(2, 3, 1);
        graph.add_edge(4, 5, 1);
        graph.add_edge(5, 6, 1);
        graph.add_edge(6, 4, 1);
        let scc = graph.scc();
        assert_eq!(scc, vec![vec![7], vec![4, 5, 6], vec![2, 3], vec![0, 1]]);

        let mut directed_graph = Graph::<usize>::new(5, true);
        for i in 0..4 {
            directed_graph.add_edge(i, i + 1, 1);
        }
        let scc = directed_graph.scc();
        assert_eq!(scc, vec![vec![0], vec![1], vec![2], vec![3], vec![4]]);

        let mut undirected_graph = Graph::<usize>::new(5, false);
        for i in 0..4 {
            undirected_graph.add_edge(i, i + 1, 1);
        }
        let scc = undirected_graph.scc();
        assert_eq!(scc, vec![vec![0, 1, 2, 3, 4]]);

        let mut self_loop_graph = Graph::<usize>::new(5, true);
        self_loop_graph.add_edge(0, 0, 1);
        let scc = self_loop_graph.scc();
        assert_eq!(scc, vec![vec![4], vec![3], vec![2], vec![1], vec![0]]);
    }

    #[test]
    fn euler_tour_test() {
        let mut graph = Graph::<usize>::new(7, false);
        graph.add_edge(1, 2, 1);
        graph.add_edge(2, 3, 1);
        graph.add_edge(3, 4, 1);
        graph.add_edge(2, 5, 1);
        graph.add_edge(1, 6, 1);

        assert_eq!(graph.euler_tour(1), vec![1, 2, 3, 4, 3, 2, 5, 2, 1, 6, 1]);

        let mut graph = Graph::<usize>::new(4, true);
        graph.add_edge(0, 1, 1);
        graph.add_edge(1, 2, 1);
        graph.add_edge(2, 3, 1);
        graph.add_edge(3, 0, 1);
        assert_eq!(graph.euler_tour(0), vec![0, 1, 2, 3, 2, 1, 0]);

        let mut graph = Graph::<usize>::new(4, true);
        graph.add_edge(0, 1, 1);
        graph.add_edge(1, 2, 1);
        graph.add_edge(2, 0, 1);
        graph.add_edge(0, 3, 1);
        assert_eq!(graph.euler_tour(0), vec![0, 1, 2, 1, 0, 3, 0]);
    }

    #[test]
    fn is_tree_test() {
        let mut graph = Graph::<usize>::new(5, false);
        graph.add_edge(0, 1, 1);
        graph.add_edge(1, 2, 1);
        graph.add_edge(1, 3, 1);
        graph.add_edge(3, 4, 1);

        assert_eq!(graph.is_tree(), true);

        let mut graph = Graph::<usize>::new(6, false);
        graph.add_edge(0, 1, 1);
        graph.add_edge(1, 2, 1);
        graph.add_edge(1, 3, 1);
        graph.add_edge(3, 4, 1);

        assert_eq!(graph.is_tree(), false);

        let mut graph = Graph::<usize>::new(3, false);
        graph.add_edge(0, 1, 1);
        graph.add_edge(1, 2, 1);
        graph.add_edge(2, 0, 1);

        assert_eq!(graph.is_tree(), false);
    }

    proptest! {
        #[test]
        fn is_tree_random_test(graph_size :u8) {
            let graph_size=graph_size as usize;

            let mut graph = Graph::new(graph_size,false);
            for i in 1..graph_size {
                let mut from:usize = random();
                from%= i;
                graph.add_edge(from, i, 1);
            }
            assert_eq!(graph.is_tree(), true);
        }
    }

    #[test]
    fn mst_test() {
        let mut graph = Graph::<usize>::new(5, true);
        graph.add_edge(0, 1, 5);
        graph.add_edge(1, 2, 1);
        graph.add_edge(2, 3, 5);
        graph.add_edge(3, 4, 1);
        graph.add_edge(0, 1, 1);
        graph.add_edge(1, 2, 5);
        graph.add_edge(2, 3, 1);
        graph.add_edge(3, 4, 5);

        assert_eq!(graph.mst(), vec![4, 1, 6, 3]);
    }
}
