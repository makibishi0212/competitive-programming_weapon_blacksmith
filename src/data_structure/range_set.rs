use cargo_snippet::snippet;
use std::collections::BTreeSet;

#[snippet("@RangeSet")]
#[derive(Debug, Clone)]
pub struct RangeSet {
    set: BTreeSet<(i64, i64)>,
}

#[snippet("@RangeSet")]
impl RangeSet {
    pub fn new() -> Self {
        Self {
            set: vec![
                (std::i64::MIN, std::i64::MIN),
                (std::i64::MAX, std::i64::MAX),
            ]
            .into_iter()
            .collect(),
        }
    }

    pub fn insert(&mut self, x: i64) {
        self.insert_range(x, x);
    }

    pub fn remove(&mut self, x: i64) {
        self.remove_range(x, x);
    }

    pub fn insert_range(&mut self, l: i64, r: i64) {
        let (new_l, new_r) = self.remove_intersect_ranges(l, r);
        self.set.insert((new_l, new_r));
    }

    pub fn remove_range(&mut self, l: i64, r: i64) {
        let (new_l, new_r) = self.remove_intersect_ranges(l, r);
        if new_l < l {
            self.set.insert((new_l, l - 1));
        }
        if r < new_r {
            self.set.insert((r + 1, new_r));
        }
    }

    fn remove_intersect_ranges(&mut self, l: i64, r: i64) -> (i64, i64) {
        let mut new_l = l;
        let mut new_r = r;
        let ranges: Vec<(i64, i64)> = self
            .set
            .range((l - 1, l - 1)..=(r + 1, std::i64::MAX))
            .copied()
            .collect();

        for (current_l, current_r) in ranges {
            new_l = current_l.min(new_l);
            new_r = current_r.max(new_r);

            self.set.remove(&(current_l, current_r));
        }

        let &(prev_l, prev_r) = self.set.range(..(l - 1, l - 1)).next_back().unwrap();
        if prev_r + 1 >= l {
            new_l = prev_l.min(new_l);
            new_r = prev_r.max(new_r);
            self.set.remove(&(prev_l, prev_r));
        }

        (new_l, new_r)
    }

    pub fn get_section(&self, x: i64) -> Option<(i64, i64)> {
        let &(l, r) = self.set.range(..(x + 1, x + 1)).next_back().unwrap();

        let in_range = l <= x && x <= r;

        if !in_range {
            return None;
        }

        Some((l, r))
    }

    pub fn ranges(&self) -> impl Iterator<Item = &(i64, i64)> {
        self.set
            .iter()
            .filter(|&&(left, right)| right != i64::MIN && left != i64::MAX)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use itertools::Itertools;
    use proptest::prelude::*;

    #[test]
    fn element_set_test() {
        let mut range_set = RangeSet::new();
        range_set.insert(0);
        range_set.insert(1);
        range_set.insert(3);
        range_set.insert(5);
        range_set.insert(4);
        range_set.insert(7);
        range_set.insert(8);
        assert_eq!(range_set.get_section(0), Some((0, 1)));
        assert_eq!(range_set.get_section(1), Some((0, 1)));
        assert_eq!(range_set.get_section(2), None);
        assert_eq!(range_set.get_section(3), Some((3, 5)));
        assert_eq!(range_set.get_section(4), Some((3, 5)));
        assert_eq!(range_set.get_section(5), Some((3, 5)));
        assert_eq!(range_set.get_section(6), None);
        assert_eq!(range_set.get_section(7), Some((7, 8)));
        assert_eq!(range_set.get_section(8), Some((7, 8)));

        range_set.remove(1);
        range_set.remove(4);
        range_set.remove(8);
        assert_eq!(range_set.get_section(0), Some((0, 0)));
        assert_eq!(range_set.get_section(1), None);
        assert_eq!(range_set.get_section(2), None);
        assert_eq!(range_set.get_section(3), Some((3, 3)));
        assert_eq!(range_set.get_section(4), None);
        assert_eq!(range_set.get_section(5), Some((5, 5)));
        assert_eq!(range_set.get_section(6), None);
        assert_eq!(range_set.get_section(7), Some((7, 7)));
        assert_eq!(range_set.get_section(8), None);
    }

    #[test]
    fn range_set_test() {
        let mut range_set = RangeSet::new();
        range_set.insert_range(0, 10);
        assert_eq!(range_set.get_section(0), Some((0, 10)));
        assert_eq!(range_set.get_section(10), Some((0, 10)));
        assert_eq!(range_set.set.len(), 1 + 2);

        range_set.remove_range(2, 4);
        assert_eq!(range_set.get_section(0), Some((0, 1)));
        assert_eq!(range_set.get_section(10), Some((5, 10)));
        assert_eq!(range_set.set.len(), 2 + 2);

        range_set.insert(3);
        assert_eq!(range_set.get_section(0), Some((0, 1)));
        assert_eq!(range_set.get_section(10), Some((5, 10)));
        assert_eq!(range_set.set.len(), 3 + 2);

        range_set.insert_range(2, 4);
        assert_eq!(range_set.get_section(0), Some((0, 10)));
        assert_eq!(range_set.get_section(10), Some((0, 10)));
        assert_eq!(range_set.set.len(), 1 + 2);
    }

    proptest! {
        #[test]
        fn all_insert_elements_have_section(inserts in prop::array::uniform32(-100i64..100)) {
            let mut range_set = RangeSet::new();
            inserts.iter().for_each(|&x| {
                range_set.insert(x);
            });

            inserts.iter().for_each(|&x| {
                assert!(range_set.get_section(x).is_some());
            });
        }

        #[test]
        fn all_remove_elements_do_not_have_section(insert_ranges in prop::array::uniform32(-100i64..100), removes in prop::array::uniform32(-100i64..100)) {

            let mut range_set = RangeSet::new();
            for pair in &insert_ranges.iter().chunks(2) {
                let x: Vec<i64> = pair.copied().collect();
                let (start, end) = (x[0].min(x[1]),x[0].max(x[1]));
                range_set.insert_range(start, end);
            }

            removes.iter().for_each(|&x| {
                range_set.remove(x);
            });

            removes.iter().for_each(|&x| {
                assert!(range_set.get_section(x).is_none());
            });
        }

        #[test]
        fn any_ranges_do_not_intersect_each_other(insert_ranges in prop::array::uniform32(-100i64..100), remove_ranges in prop::array::uniform32(-100i64..100)){
            let mut range_set = RangeSet::new();
            for pair in &insert_ranges.iter().chunks(2) {
                let x: Vec<i64> = pair.copied().collect();
                let (start, end) = (x[0].min(x[1]),x[0].max(x[1]));
                range_set.insert_range(start, end);
            }

            for pair in &remove_ranges.iter().chunks(2) {
                let x: Vec<i64> = pair.copied().collect();
                let (start, end) = (x[0].min(x[1]),x[0].max(x[1]));
                range_set.remove_range(start, end);
            }

            range_set.ranges().combinations(2).for_each(|ranges| {
                let (a_l,a_r) = ranges[0];
                let (b_l,b_r) = ranges[1];

                let b_l_in_a = a_l <= b_l && b_l <= a_r;
                let b_r_in_a = a_l <= b_r && b_r <= a_r;
                let a_l_in_b = b_l <= a_l && a_l <= b_r;
                let a_r_in_b = b_l <= a_r && a_r <= b_r;

                assert!(!b_l_in_a);
                assert!(!b_r_in_a);
                assert!(!a_l_in_b);
                assert!(!a_r_in_b);
            });
        }
    }
}
