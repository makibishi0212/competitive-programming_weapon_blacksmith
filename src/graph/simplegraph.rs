use std::usize;

use cargo_snippet::snippet;

// 多重辺を含まないグラフ。頂点は0-indexed。自己ループ辺はok
#[snippet("@SimpleGraph")]
pub struct SimpleGraph<T> {
    size: usize,
    edges: Vec<std::collections::BTreeMap<usize, T>>,
    directed: bool,
}

#[snippet("@SimpleGraph")]
impl<T: std::ops::Add + std::ops::Sub<Output = T> + std::marker::Copy + std::cmp::PartialOrd>
    SimpleGraph<T>
{
    pub fn new(n: usize, directed: bool) -> SimpleGraph<T> {
        SimpleGraph::<T> {
            size: n,
            edges: vec![std::collections::BTreeMap::new(); n],
            directed,
        }
    }

    // prior_min_cost: すでに与えられたコストより小さいコストが保存されている時、そちらを優先するかどうか
    pub fn add_or_update_edge(&mut self, from: usize, to: usize, cost: T, prior_min_cost: bool) {
        let new_cost = {
            if !prior_min_cost {
                cost
            } else {
                if let Some(old_cost) = self.edges[from].get(&to) {
                    if *old_cost > cost {
                        cost
                    } else {
                        *old_cost
                    }
                } else {
                    cost
                }
            }
        };
        self.edges[from].insert(to, new_cost);

        // 無向グラフなら、反対にも辺を張る
        if !self.directed {
            self.edges[to].insert(from, new_cost);
        }
    }

    pub fn get_edge(&self, from: usize, to: usize) -> Option<T> {
        if let Some(cost) = self.edges[from].get(&to) {
            Some(*cost)
        } else {
            None
        }
    }

    pub fn topological_sort(&self) -> Vec<usize> {
        let mut result = vec![];
        let mut isolated = std::collections::VecDeque::new();

        // deg[x] = a -> x, b -> xのような辺の数
        let mut degs = vec![0; self.size];

        let isolate_deg = if self.directed { 0 } else { 1 };

        self.edges.iter().enumerate().for_each(|(_, v_edges)| {
            v_edges.keys().for_each(|&to| {
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
            for &i in self.edges[next].keys() {
                println!("{:?}", i);
                degs[i] -= 1;
                if degs[i] == isolate_deg {
                    isolated.push_front(i);
                }
            }
        }

        result
    }
}

// 負辺を含まないグラフのためのメソッド
#[snippet("@SimpleGraph")]
impl SimpleGraph<usize> {
    // Dijkstraで1対nの最小距離を求める(vec[from]は自己ループ辺のコストになる)
    pub fn min_dists(&self, from: usize) -> Vec<usize> {
        let mut visited = vec![false; self.size];
        let mut from_to_n = vec![std::usize::MAX; self.size];
        let mut queue = std::collections::BinaryHeap::new();
        queue.push(std::cmp::Reverse((0, from)));

        while !queue.is_empty() {
            let (cost, to) = queue.pop().unwrap().0;
            if visited[to] {
                continue;
            }
            visited[to] = true;

            for (&to_to, &to_cost) in &self.edges[to] {
                from_to_n[to_to] = std::cmp::min(from_to_n[to_to], cost + to_cost);
                queue.push(std::cmp::Reverse((cost + to_cost, to_to)));
            }
        }

        from_to_n
    }
}

// 負辺を含むグラフのためのメソッド
impl SimpleGraph<i64> {
    // Bellman-Ford法で1対nの最小距離を求める。負閉路を検出した場合はNone
    // このメソッドでは自己ループ辺を無視する。
    pub fn min_dists(&self, from: usize) -> Option<Vec<i64>> {
        let mut from_to_n = vec![std::i64::MAX; self.size];
        from_to_n[from] = 0;
        for _ in 0..self.size {
            self.edges.iter().enumerate().for_each(|(from_v, v_edges)| {
                v_edges.iter().for_each(|(&to_v, &cost)| {
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
            v_edges.iter().for_each(|(&to_v, &cost)| {
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
    use crate::graph;

    use super::*;

    #[test]
    fn edge_update_test() {
        let mut graph = SimpleGraph::<usize>::new(2, true);
        graph.add_or_update_edge(0, 1, 100, true);
        assert_eq!(graph.get_edge(0, 1), Some(100));
        graph.add_or_update_edge(0, 1, 200, true);
        assert_eq!(graph.get_edge(0, 1), Some(100));
        graph.add_or_update_edge(0, 1, 200, false);
        assert_eq!(graph.get_edge(0, 1), Some(200));
    }

    #[test]
    fn min_dists_test() {
        let mut graph = SimpleGraph::<usize>::new(4, true);
        graph.add_or_update_edge(0, 1, 100, true);
        graph.add_or_update_edge(1, 2, 100, true);
        graph.add_or_update_edge(2, 3, 150, true);
        graph.add_or_update_edge(3, 0, 3, true);

        assert_eq!(graph.min_dists(0), vec![353, 100, 200, 350]);
        assert_eq!(graph.min_dists(1), vec![253, 353, 100, 250]);
        assert_eq!(graph.min_dists(2), vec![153, 253, 353, 150]);
        assert_eq!(graph.min_dists(3), vec![3, 103, 203, 353]);
    }

    #[test]
    fn min_dists_self_loop_case_test() {
        let mut graph = SimpleGraph::<usize>::new(4, true);
        graph.add_or_update_edge(0, 1, 100, true);
        graph.add_or_update_edge(1, 2, 100, true);
        graph.add_or_update_edge(2, 3, 150, true);
        graph.add_or_update_edge(3, 0, 3, true);

        // 上のテストに自己ループ辺を追加
        graph.add_or_update_edge(0, 0, 0, true);
        graph.add_or_update_edge(3, 3, 11, true);

        assert_eq!(graph.min_dists(0), vec![0, 100, 200, 350]);
        assert_eq!(graph.min_dists(1), vec![253, 353, 100, 250]);
        assert_eq!(graph.min_dists(2), vec![153, 253, 353, 150]);
        assert_eq!(graph.min_dists(3), vec![3, 103, 203, 11]);
    }

    #[test]
    fn min_dists_test2() {
        let mut graph = SimpleGraph::<usize>::new(5, true);
        graph.add_or_update_edge(0, 1, 90, true);
        graph.add_or_update_edge(1, 2, 180, true);
        graph.add_or_update_edge(0, 2, 150, true);
        graph.add_or_update_edge(0, 3, 40, true);
        graph.add_or_update_edge(3, 4, 9000, true);

        assert_eq!(
            graph.min_dists(0),
            vec![18446744073709551615, 90, 150, 40, 9040]
        );
        assert_eq!(
            graph.min_dists(1),
            vec![
                18446744073709551615,
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
        graph.add_or_update_edge(0, 1, 90, true);
        graph.add_or_update_edge(1, 2, 180, true);
        graph.add_or_update_edge(0, 2, 150, true);
        graph.add_or_update_edge(0, 3, 40, true);
        graph.add_or_update_edge(3, 4, 9000, true);

        assert_eq!(graph.min_dists(0), Some(vec![0, 90, 150, 40, 9040]));
    }

    #[test]
    fn min_dists_i64_test2() {
        let mut graph = SimpleGraph::<i64>::new(5, true);
        graph.add_or_update_edge(0, 1, -50, true);
        graph.add_or_update_edge(1, 0, 70, true);
        graph.add_or_update_edge(1, 2, 180, true);
        graph.add_or_update_edge(0, 2, 150, true);
        graph.add_or_update_edge(0, 3, 40, true);
        graph.add_or_update_edge(3, 4, 9000, true);

        assert_eq!(graph.min_dists(0), Some(vec![0, -50, 130, 40, 9040]));
    }

    #[test]
    fn min_dists_i64_negative_cycle_test() {
        let mut graph = SimpleGraph::<i64>::new(3, true);
        graph.add_or_update_edge(0, 1, -70, true);
        graph.add_or_update_edge(1, 0, 69, true);
        graph.add_or_update_edge(0, 2, 200, true);

        assert_eq!(graph.min_dists(0), None);
    }

    #[test]
    fn min_dists_undirected_test() {
        // 上のテストを無向グラフにしたもの
        let mut graph = SimpleGraph::<usize>::new(5, false);
        graph.add_or_update_edge(0, 1, 90, true);
        graph.add_or_update_edge(1, 2, 180, true);
        graph.add_or_update_edge(0, 2, 150, true);
        graph.add_or_update_edge(0, 3, 40, true);
        graph.add_or_update_edge(3, 4, 9000, true);

        assert_eq!(graph.min_dists(0), vec![80, 90, 150, 40, 9040]);
        assert_eq!(graph.min_dists(1), vec![90, 180, 180, 130, 9130]);
        assert_eq!(graph.min_dists(2), vec![150, 180, 300, 190, 9190]);
        assert_eq!(graph.min_dists(3), vec![40, 130, 190, 80, 9000]);
        assert_eq!(graph.min_dists(4), vec![9040, 9130, 9190, 9000, 18000]);
    }

    #[test]
    fn topological_sort_directed_test() {
        let mut graph = SimpleGraph::<usize>::new(5, true);
        graph.add_or_update_edge(0, 1, 1, false);
        graph.add_or_update_edge(1, 2, 1, false);
        graph.add_or_update_edge(3, 2, 1, false);
        graph.add_or_update_edge(2, 4, 1, false);
        assert_eq!(graph.topological_sort(), vec![0, 3, 1, 2, 4]);
    }

    #[test]
    fn topological_sort_undirected_test() {
        let mut graph = SimpleGraph::<usize>::new(4, false);
        graph.add_or_update_edge(2, 3, 1, false);
        graph.add_or_update_edge(0, 1, 1, false);
        graph.add_or_update_edge(1, 2, 1, false);
        assert_eq!(graph.topological_sort(), vec![0, 3, 1, 2]);
    }
}
