use cargo_snippet::snippet;

// [0,n)の範囲のSetを程数倍高速で動作させるやつ
#[snippet("@IndexSet")]
pub struct IndexSet {
    vec: Vec<usize>,
    poses: Vec<Option<usize>>,
}

#[snippet("@IndexSet")]
impl IndexSet {
    pub fn new(n: usize) -> Self {
        Self {
            vec: vec![],
            poses: vec![None; n],
        }
    }

    // 0 <= value < n
    pub fn insert(&mut self, value: usize) {
        if self.contains(value) {
            return;
        }

        self.poses[value] = Some(self.vec.len());
        self.vec.push(value);
    }

    pub fn remove(&mut self, value: usize) {
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

    pub fn contains(&self, value: usize) -> bool {
        self.poses[value].is_some()
    }

    pub fn size(&self) -> usize {
        self.vec.len()
    }

    pub fn iter(&self) -> impl Iterator<Item = &usize> {
        self.vec.iter()
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

    proptest! {
        #[test]
        fn test(insert_values in prop::array::uniform32(any::<u8>()), remove_values in prop::array::uniform16(any::<u8>())) {
            let mut index_set = IndexSet::new(256);
            let mut set = std::collections::HashSet::new();

            insert_values.iter().for_each(|&value|{
                let value = value as usize;
                index_set.insert(value);
                set.insert(value);
            });

            remove_values.iter().for_each(|&value| {
                let value = value as usize;
                index_set.remove(value);
                set.remove(&value);
            });

            assert_eq!(index_set.size(), set.len());

            set.iter().for_each(|&value| {
                assert!(index_set.contains(value));
            });

            index_set.iter().for_each(|value| {
                assert!(set.contains(value));
            });
        }
    }
}
