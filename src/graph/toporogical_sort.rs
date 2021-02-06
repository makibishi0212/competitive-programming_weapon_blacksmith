use cargo_snippet::snippet;
use std::collections::VecDeque;

#[snippet("@topological_sort_directed")]
pub fn topological_sort_directed_graph(
    n: usize,
    zeroIndexedEdges: &[(usize, usize)],
) -> Vec<usize> {
    let mut result = vec![];
    let mut isolated = VecDeque::new();

    // to[x] = [a,b] x -> a, x -> b が存在
    let mut to = vec![vec![]; n];

    // deg[x] = a -> x, b -> xのような辺の数
    let mut deg = vec![0; n + 1];

    for e in zeroIndexedEdges {
        to[e.0].push(e.1);
        deg[e.1] += 1;
    }

    for i in 0..n {
        if deg[i] == 0 {
            isolated.push_front(i);
        }
    }

    while isolated.len() != 0 {
        let next = isolated.pop_back().unwrap();
        result.push(next);
        for i in &to[next] {
            deg[*i] -= 1;
            if deg[*i] == 0 {
                isolated.push_front(*i);
            }
        }
    }

    result
}

#[snippet("@topological_sort_undirected")]
pub fn topological_sort_undirected_graph(
    n: usize,
    zeroIndexedEdges: &[(usize, usize)],
) -> Vec<usize> {
    let mut result = vec![];
    let mut isolated = VecDeque::new();

    // to[x] = [a,b] x -> a, x -> b が存在
    let mut to = vec![vec![]; n];

    // deg[x] = a -> x, b -> xのような辺の数
    let mut deg = vec![0; n + 1];

    for e in zeroIndexedEdges {
        to[e.0].push(e.1);
        to[e.1].push(e.0);
        deg[e.0] += 1;
        deg[e.1] += 1;
    }

    for i in 0..n {
        if deg[i] == 1 {
            isolated.push_front(i);
        }
    }

    while isolated.len() != 0 {
        let next = isolated.pop_back().unwrap();
        result.push(next);
        for i in &to[next] {
            deg[*i] -= 1;
            if deg[*i] == 1 {
                isolated.push_front(*i);
            }
        }
    }

    result
}

mod test {
    use super::*;
    #[test]
    fn topological_sort_directed_graph_test() {
        let edges = [(0, 1), (1, 2), (3, 2), (2, 4)];
        let vertex = topological_sort_directed_graph(5, &edges);
        assert_eq!(vertex, vec![0, 3, 1, 2, 4]);

        let edges = [(0, 1), (1, 2), (2, 3), (3, 4)];
        let vertex = topological_sort_directed_graph(5, &edges);
        assert_eq!(vertex, vec![0, 1, 2, 3, 4]);

        let edges = [
            (0, 1),
            (2, 1),
            (3, 1),
            (4, 1),
            (5, 1),
            (6, 1),
            (1, 7),
            (8, 7),
            (9, 8),
        ];
        let vertex = topological_sort_directed_graph(10, &edges);
        assert_eq!(vertex, vec![0, 2, 3, 4, 5, 6, 9, 1, 8, 7]);
    }

    #[test]
    fn topological_sort_undirected_graph_test() {
        let edges = [(2, 3), (0, 1), (1, 2)];
        let vertex = topological_sort_undirected_graph(4, &edges);
        assert_eq!(vertex, vec![0, 3, 1, 2]);

        let edges = [(0, 2), (1, 2), (3, 5), (4, 5), (2, 6), (3, 6)];
        let vertex = topological_sort_undirected_graph(7, &edges);
        assert_eq!(vertex, vec![0, 1, 4, 2, 5, 6, 3]);

        let edges = [(0, 1), (1, 2), (1, 3), (3, 4)];
        let vertex = topological_sort_undirected_graph(5, &edges);
        assert_eq!(vertex, vec![0, 2, 4, 1, 3]);
    }
}
