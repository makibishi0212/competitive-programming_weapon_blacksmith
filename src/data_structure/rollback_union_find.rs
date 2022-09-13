use std::collections::VecDeque;

// https://nyaannyaan.github.io/library/data-structure/rollback-union-find.hpp.html
pub struct RollbackUnionFind {
    data: Vec<i64>,
    history_stack: VecDeque<((usize, i64), (usize, i64))>,
    size: usize,
    memory: usize, // 履歴の最大容量
}

impl RollbackUnionFind {
    pub fn new(n: usize, memory: usize) -> Self {
        RollbackUnionFind {
            data: vec![-1; n],
            history_stack: VecDeque::with_capacity(memory),
            size: n,
            memory,
        }
    }

    fn root(&self, x: usize) -> usize {
        if self.data[x] < 0 {
            return x;
        }

        return self.root(self.data[x] as usize);
    }

    pub fn same(&self, x: usize, y: usize) -> bool {
        debug_assert!(x < self.size);
        debug_assert!(y < self.size);
        self.root(x) == self.root(y)
    }

    pub fn unite(&mut self, x: usize, y: usize) {
        debug_assert!(x < self.size);
        debug_assert!(y < self.size);

        let root_x = self.root(x);
        let root_y = self.root(y);

        if self.history_stack.len() == self.memory {
            self.history_stack.pop_back();
        }
        self.history_stack
            .push_back(((root_x, self.data[root_x]), (root_y, self.data[root_y])));
        if root_x == root_y {
            return;
        }

        if self.data[root_x] > self.data[root_y] {
            self.data.swap(root_x, root_y);
        }
        self.data[root_x] += self.data[root_y];
        self.data[root_y] = root_x as i64;
    }

    pub fn undo(&mut self) -> bool {
        match self.history_stack.pop_back() {
            None => return false,
            Some(((x, data_x), (y, data_y))) => {
                self.data[x] = data_x;
                self.data[y] = data_y;
                return true;
            }
        }
    }

    pub fn size(&self, x: usize) -> usize {
        debug_assert!(x < self.size);
        (-self.data[self.root(x)]) as usize
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    #[test]
    fn rollback_union_find_test() {
        let mut uf = RollbackUnionFind::new(5, 3);
        uf.unite(2, 4);
        assert!(uf.same(2, 4));
        assert_eq!(uf.size(0), 1);
        assert_eq!(uf.size(1), 1);
        assert_eq!(uf.size(2), 2);
        assert_eq!(uf.size(3), 1);
        assert_eq!(uf.size(4), 2);

        uf.unite(4, 3);
        assert!(uf.same(2, 3));
        assert!(uf.same(3, 4));
        assert_eq!(uf.size(0), 1);
        assert_eq!(uf.size(1), 1);
        assert_eq!(uf.size(2), 3);
        assert_eq!(uf.size(3), 3);
        assert_eq!(uf.size(4), 3);

        uf.unite(0, 3);
        assert!(uf.same(0, 2));
        assert!(uf.same(0, 4));
        assert_eq!(uf.size(0), 4);
        assert_eq!(uf.size(1), 1);
        assert_eq!(uf.size(2), 4);
        assert_eq!(uf.size(3), 4);
        assert_eq!(uf.size(4), 4);

        assert_eq!(uf.undo(), true);
        assert!(!uf.same(0, 2));
        assert!(!uf.same(0, 4));
        assert_eq!(uf.size(0), 1);
        assert_eq!(uf.size(1), 1);
        assert_eq!(uf.size(2), 3);
        assert_eq!(uf.size(3), 3);
        assert_eq!(uf.size(4), 3);

        assert_eq!(uf.undo(), true);
        assert!(!uf.same(2, 3));
        assert!(!uf.same(3, 4));
        assert_eq!(uf.size(0), 1);
        assert_eq!(uf.size(1), 1);
        assert_eq!(uf.size(2), 2);
        assert_eq!(uf.size(3), 1);
        assert_eq!(uf.size(4), 2);

        assert_eq!(uf.undo(), true);
        assert_eq!(uf.size(0), 1);
        assert_eq!(uf.size(1), 1);
        assert_eq!(uf.size(2), 1);
        assert_eq!(uf.size(3), 1);
        assert_eq!(uf.size(4), 1);
    }

    proptest! {
        #[test]
        fn same_all_after_connect_half(connect_a in 0usize..10, connect_b in 10usize..20, x in 0usize..20, y in 0usize..20){
          let mut uf = RollbackUnionFind::new(20, 1);
          for i in 0..9 {
            uf.unite(i, i+1);
          }
          for i in 10..19 {
            uf.unite(i, i+1);
          }


          uf.unite(connect_a, connect_b);
          assert!(uf.same(x, y));
          assert_eq!(uf.size(x), 20);
          assert_eq!(uf.size(y), 20);

          assert_eq!(uf.undo(),true);
          assert_eq!(uf.size(x), 10);
          assert_eq!(uf.size(y), 10);
        }

        #[test]
        fn negative_total_equal_set_size(size in 100usize..200, connects in prop::array::uniform32(0usize..100)) {
            let mut uf = RollbackUnionFind::new(size, 1);
            for i in 0..connects.len() {
                uf.unite(0, i);
            }

            let mut total = 0i64;
            for i in 0..size {
                if uf.data[i].is_negative() {
                    total+= -uf.data[i];
                }
            }
            assert_eq!(total as usize, size);
        }
    }
}
