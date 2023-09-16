use cargo_snippet::snippet;

#[snippet("@IndexSet")]
pub struct IndexSet {
    vec: Vec<usize>,
    poses: Vec<Option<usize>>,
}

#[snippet("@IndexSet")]
impl IndexSet {
    fn new(n: usize) -> Self {
        Self {
            vec: vec![],
            poses: vec![None; n],
        }
    }

    // 0 <= value < n
    fn insert(&mut self, value: usize) {
        self.poses[value] = Some(self.vec.len());
        self.vec.push(value);
    }

    fn remove(&mut self, value: usize) {
        let Some(pos) = self.poses[value] else {
            return;
        };
        let Some(&last) = self.vec.last() else {
            return;
        };

        self.vec[pos] = last;
        self.vec.pop();
        self.poses[last] = Some(pos);
        self.poses[value] = None;
    }

    fn contains(&self, value: usize) -> bool {
        self.poses[value].is_some()
    }

    fn size(&self) -> usize {
        self.vec.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    #[test]
    fn test_index_set() {
        let mut index_set = IndexSet::new(10); // 0-9
        index_set.insert(5);
        assert!(index_set.contains(5));

        index_set.remove(5);
        assert!(!index_set.contains(5));

        index_set.insert(8);
        assert!(!index_set.contains(5));
        assert!(index_set.contains(8));

        index_set.insert(2);
        assert!(!index_set.contains(5));
        assert!(index_set.contains(2));
        assert!(index_set.contains(8));

        index_set.insert(4);
        assert!(!index_set.contains(5));
        assert!(index_set.contains(2));
        assert!(index_set.contains(4));
        assert!(index_set.contains(8));

        index_set.remove(8);
        assert!(!index_set.contains(5));
        assert!(index_set.contains(2));
        assert!(index_set.contains(4));
        assert!(!index_set.contains(8));

        assert_eq!(index_set.size(), 2);

        index_set.insert(5);
        assert!(index_set.contains(5));
        assert!(index_set.contains(2));
        assert!(index_set.contains(4));
        assert!(!index_set.contains(8));

        assert_eq!(index_set.size(), 3);
    }
}
