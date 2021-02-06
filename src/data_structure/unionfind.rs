use cargo_snippet::snippet;

#[snippet("@UnionFind")]
pub struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
    size: Vec<usize>,
}

#[snippet("@UnionFind")]
impl UnionFind {
    pub fn new(n: usize) -> UnionFind {
        UnionFind {
            parent: (0..n).collect(),
            rank: vec![0; n],
            size: vec![1; n],
        }
    }

    pub fn root(&mut self, x: usize) -> usize {
        let parent = self.parent[x];
        if parent != x {
            self.parent[x] = self.root(parent);
            self.rank[x] = 1;
        }
        return self.parent[x];
    }

    pub fn unite(&mut self, x: usize, y: usize) {
        if self.same(x, y) {
            return;
        }
        let x_root = self.root(x);
        let y_root = self.root(y);
        if self.rank[x_root] >= self.rank[y_root] {
            self.size[x_root] += self.size(y);
            self.rank[y_root] = 1;
            self.parent[y_root] = x_root;
        } else {
            let x_root = self.root(x);
            let y_root = self.root(y);
            self.size[y_root] += self.size(x);
            self.rank[x_root] = 1;
            self.parent[x_root] = y_root;
        }
    }

    pub fn size(&mut self, x: usize) -> usize {
        let x_root = self.root(x);
        return self.size[x_root];
    }

    pub fn same(&mut self, x: usize, y: usize) -> bool {
        self.root(x) == self.root(y)
    }

    pub fn groups(&mut self) -> Vec<Vec<usize>> {
        let n = self.size.len();
        let mut root_buf = vec![0; n];
        let mut group_size = vec![0; n];
        for i in 0..n {
            root_buf[i] = self.root(i);
            group_size[root_buf[i]] += 1;
        }

        let mut result = vec![Vec::new(); n];
        for i in 0..n {
            result[i].reserve(group_size[i]);
        }
        for i in 0..n {
            result[root_buf[i]].push(i);
        }

        result
            .into_iter()
            .filter(|x| !x.is_empty())
            .collect::<Vec<Vec<usize>>>()
    }
}

#[test]
fn union_find_test() {
    let mut group = UnionFind::new(5);
    assert_eq!(group.same(0, 3), false);
    assert_eq!(group.same(1, 3), false);
    assert_eq!(group.same(0, 4), false);
    group.unite(0, 3);
    assert_eq!(group.same(0, 3), true);
    assert_eq!(group.same(1, 3), false);
    assert_eq!(group.same(0, 4), false);
    group.unite(0, 4);
    assert_eq!(group.same(0, 3), true);
    assert_eq!(group.same(1, 3), false);
    assert_eq!(group.same(0, 4), true);
    group.unite(4, 1);
    assert_eq!(group.same(0, 3), true);
    assert_eq!(group.same(1, 3), true);
    assert_eq!(group.same(0, 4), true);
    assert_eq!(group.groups(), vec![vec![0, 1, 3, 4], vec![2]]);

    let mut group2 = UnionFind::new(200);
    (0..200).step_by(2).for_each(|x| {
        group2.unite(0, x);
    });
    assert_eq!(group2.same(0, 60), true);
    assert_eq!(group2.same(2, 60), true);
    assert_eq!(group2.same(130, 198), true);
    assert_eq!(group2.same(100, 160), true);
    assert_eq!(group2.same(5, 60), false);
    assert_eq!(group2.same(1, 160), false);
    assert_eq!(group2.same(5, 177), false);
    assert_eq!(group2.same(31, 85), false);
    assert_eq!(group2.same(97, 189), false);
    (1..200).step_by(2).for_each(|x| {
        group2.unite(1, x);
    });
    assert_eq!(group2.same(0, 60), true);
    assert_eq!(group2.same(2, 60), true);
    assert_eq!(group2.same(130, 198), true);
    assert_eq!(group2.same(100, 160), true);
    assert_eq!(group2.same(5, 60), false);
    assert_eq!(group2.same(1, 160), false);
    assert_eq!(group2.same(5, 177), true);
    assert_eq!(group2.same(31, 85), true);
    assert_eq!(group2.same(97, 189), true);
    group2.unite(1, 4);
    assert_eq!(group2.same(0, 60), true);
    assert_eq!(group2.same(2, 60), true);
    assert_eq!(group2.same(130, 198), true);
    assert_eq!(group2.same(100, 160), true);
    assert_eq!(group2.same(5, 60), true);
    assert_eq!(group2.same(1, 160), true);
    assert_eq!(group2.same(5, 177), true);
    assert_eq!(group2.same(31, 85), true);
    assert_eq!(group2.same(97, 189), true);
}
