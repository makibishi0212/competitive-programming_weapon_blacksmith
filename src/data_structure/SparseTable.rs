use std::{cmp, usize};

use cargo_snippet::snippet;

#[snippet("@SparseTable")]
pub struct SparseTable<T: std::clone::Clone + std::fmt::Debug> {
    // f(f(a,b),c) = f(a,f(b,c))、f(a,a)=a を満たす必要がある
    operation: fn(T, T) -> T,
    sparse_table: Vec<Vec<Option<T>>>,
    lookup: Vec<usize>,
}

#[snippet("@SparseTable")]
impl<T: std::clone::Clone + std::fmt::Debug> SparseTable<T> {
    pub fn new(array: Vec<T>, operation: fn(T, T) -> T) -> SparseTable<T> {
        let mut table_length = 0;
        while (1 << table_length) <= array.len() {
            table_length += 1;
        }

        let mut sparse_table: Vec<Vec<Option<T>>> =
            vec![vec![None; 1 << table_length]; table_length];
        array
            .iter()
            .enumerate()
            .for_each(|(index, value)| sparse_table[0][index] = Some(value.clone()));
        for i in 1..table_length {
            for j in 0..((1 << table_length) - (1 << i)) {
                let a = sparse_table[i - 1][j].clone();
                let b = sparse_table[i - 1][j + (1 << (i - 1))].clone();

                if let Some(value_a) = a {
                    if let Some(value_b) = b {
                        sparse_table[i][j] = Some(operation(value_a, value_b));
                    } else {
                        sparse_table[i][j] = Some(value_a);
                    }
                } else {
                    if let Some(value_b) = b {
                        sparse_table[i][j] = Some(value_b);
                    } else {
                        sparse_table[i][j] = None;
                    }
                }
            }
        }

        let mut lookup = vec![0; array.len() + 1];
        for i in 2..lookup.len() {
            lookup[i] = lookup[i >> 1] + 1;
        }

        SparseTable {
            operation,
            sparse_table,
            lookup,
        }
    }

    pub fn query(&self, left_index: usize, right_index: usize) -> T {
        let b = self.lookup[right_index - left_index];
        let operation = self.operation;

        let a = self.sparse_table[b][left_index].clone();
        let b = self.sparse_table[b][right_index - (1 << b)].clone();

        if let Some(value_a) = a {
            if let Some(value_b) = b {
                operation(value_a, value_b)
            } else {
                value_a
            }
        } else {
            if let Some(value_b) = b {
                value_b
            } else {
                panic!()
            }
        }
    }
}

mod test {
    use super::*;
    use proptest::prelude::*;

    #[test]
    fn sparse_table_test() {
        let st = SparseTable::new(vec![1, 2, 3, 4, 5], |a, b| cmp::max(a, b));
        assert_eq!(st.query(0, 1), 1);
        assert_eq!(st.query(0, 2), 2);
        assert_eq!(st.query(0, 3), 3);
        assert_eq!(st.query(0, 4), 4);
        assert_eq!(st.query(0, 5), 5);

        let st = SparseTable::new(vec![1, 2, 3, 4, 5], |a, b| cmp::min(a, b));
        assert_eq!(st.query(0, 1), 1);
        assert_eq!(st.query(0, 2), 1);
        assert_eq!(st.query(0, 3), 1);
        assert_eq!(st.query(0, 4), 1);
        assert_eq!(st.query(0, 5), 1);

        let st = SparseTable::new(vec![240, 212, 38, 4999, 5, 1234, 547, 199999, 0], |a, b| {
            cmp::min(a, b)
        });
        assert_eq!(st.query(0, 3), 38);
        assert_eq!(st.query(3, 5), 5);
        assert_eq!(st.query(5, 7), 547);
        assert_eq!(st.query(7, 8), 199999);
        assert_eq!(st.query(7, 9), 0);
        assert_eq!(st.query(1, 4), 38);

        // 最大公約数
        let st = SparseTable::new(vec![4, 4, 6, 6, 5], |a, b| {
            let mut aa: usize = if a > b { a } else { b };
            let mut bb: usize = if a > b { b } else { a };
            while bb != 0 {
                let tmp = bb;
                bb = aa % tmp;
                aa = tmp;
            }
            return aa;
        });
        assert_eq!(st.query(0, 1), 4);
        assert_eq!(st.query(0, 2), 4);
        assert_eq!(st.query(0, 3), 2);
        assert_eq!(st.query(0, 4), 2);
        assert_eq!(st.query(0, 5), 1);

        assert_eq!(st.query(2, 4), 6);
        assert_eq!(st.query(4, 5), 5);
    }

    proptest! {
        #[test]
        fn random_array_min(array :Vec<usize>) {
            if  let Some(min) = array.iter().min() {
                let st = SparseTable::new(array.clone(),|a, b| cmp::min(a, b));
                let result =st.query(0, array.len());
                assert_eq!(result,*min);
            }
        }

        #[test]
        fn random_array_max(array :Vec<usize>) {
            if  let Some(max) = array.iter().max() {
                let st = SparseTable::new(array.clone(),|a, b| cmp::max(a, b));
                let result =st.query(0, array.len());
                assert_eq!(result,*max);
            }
        }
    }
}
