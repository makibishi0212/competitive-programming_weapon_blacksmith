use cargo_snippet::snippet;

pub struct Graph<T> {
    size: usize,
    edges: Vec<(usize, usize, T)>,
    directed: bool,
}

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
            self.edges.push((from, to, cost));
        }
    }
}

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

    /*
    #[test]
    fn remove_edge_directed_graph_test() {
        let mut graph = Graph::<usize>::new(5, true);
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
    fn remove_edge_undirected_graph_test() {
        let mut graph = Graph::<usize>::new(5, false);
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
    */
}
