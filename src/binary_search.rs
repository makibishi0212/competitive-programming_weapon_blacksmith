//! ## About
//!
//! This is not function. Extra implement for Vec\<T\>.
//!
//! Give your vector the power of binary search.
//!
//! Your vector must be sorted. And `T` must implement std::cmp::PartialOrd.
//!
//! ## Prefix
//! `@BinarySearchPower`
//!
//! ## Example
//! ```
//! let array = vec![1,2,3,4,5];
//!
//! // return vector index;
//! assert_eq(array.lower_bound_only_asc(2),Some(1));
//!
//! // If value is over maximum value of the array, return None.
//! assert_eq(array.lower_bound_only_asc(6),None);
//! ```

use cargo_snippet::snippet;

#[snippet("@BinSearchPower")]
pub trait BinSearchPower<T: std::cmp::PartialOrd> {
    fn lower_bound_only_asc(&self, value: T) -> Option<usize>;
    fn upper_bound_only_asc(&self, value: T) -> Option<usize>;
}

#[snippet("@BinSearchPower")]
impl<T: std::cmp::PartialOrd> BinSearchPower<T> for [T] {
    // 指定された値以上の値のインデックスを返す
    fn lower_bound_only_asc(&self, value: T) -> Option<usize> {
        let mut ng = -1;
        let mut ok = self.len() as i64;

        while ok - ng > 1 {
            let mid = (ok + ng) / 2;

            if self[mid as usize] >= value {
                ok = mid;
            } else {
                ng = mid;
            }
        }

        if ok == self.len() as i64 {
            None
        } else {
            Some(ok as usize)
        }
    }

    // 指定された値より大きい値のインデックスを返す
    fn upper_bound_only_asc(&self, value: T) -> Option<usize> {
        let mut ng = -1;
        let mut ok = self.len() as i64;

        while ok - ng > 1 {
            let mid = (ok + ng) / 2;
            if self[mid as usize] > value {
                ok = mid;
            } else {
                ng = mid;
            }
        }

        if ok == self.len() as i64 {
            None
        } else {
            Some(ok as usize)
        }
    }
}

mod test {
    use super::*;

    #[test]
    fn binarySeachPowerTest() {
        let array = vec![-3, -1, 0, 1, 3, 5, 7, 111, 1210, 1345, 123479];
        assert_eq!(array[array.lower_bound_only_asc(1).unwrap()], 1);
        assert_eq!(array[array.upper_bound_only_asc(1).unwrap()], 3);
        assert_eq!(array[array.lower_bound_only_asc(-3).unwrap()], -3);
        assert_eq!(array[array.upper_bound_only_asc(-3).unwrap()], -1);
        assert_eq!(array[array.lower_bound_only_asc(111).unwrap()], 111);
        assert_eq!(array[array.upper_bound_only_asc(111).unwrap()], 1210);
        assert_eq!(array[array.lower_bound_only_asc(123478).unwrap()], 123479);
        assert_eq!(array[array.upper_bound_only_asc(123478).unwrap()], 123479);
        assert_eq!(array[array.lower_bound_only_asc(-20).unwrap()], -3);
        assert_eq!(array[array.upper_bound_only_asc(-20).unwrap()], -3);
        // 指定の数字が最大の値より大きい場合はnone
        assert_eq!(array.lower_bound_only_asc(125000), None);
        assert_eq!(array.upper_bound_only_asc(125000), None);
        assert_eq!(array[array.lower_bound_only_asc(123479).unwrap()], 123479);
        assert_eq!(array.upper_bound_only_asc(123479), None);

        // タプルの2分探索
        let tuple_array = vec![(0, 1), (1, 1), (2, 1), (4, 1), (4, 3)];
        assert_eq!(
            tuple_array[tuple_array.lower_bound_only_asc((3, 100)).unwrap()],
            (4, 1)
        );
        assert_eq!(
            tuple_array[tuple_array.upper_bound_only_asc((3, 100)).unwrap()],
            (4, 1)
        );
        assert_eq!(
            tuple_array[tuple_array.lower_bound_only_asc((2, 1)).unwrap()],
            (2, 1)
        );
        assert_eq!(
            tuple_array[tuple_array.upper_bound_only_asc((2, 1)).unwrap()],
            (4, 1)
        );

        // f64
        let double_array: Vec<f64> = vec![-0.1, 0.2, 0.3, 0.4, 0.5, 1.5];
        assert_eq!(
            double_array[double_array.lower_bound_only_asc(0.25).unwrap()],
            0.3
        );
        assert_eq!(
            double_array[double_array.upper_bound_only_asc(0.25).unwrap()],
            0.3
        );
        assert_eq!(
            double_array[double_array.lower_bound_only_asc(0.4).unwrap()],
            0.4
        );
        assert_eq!(
            double_array[double_array.upper_bound_only_asc(0.4).unwrap()],
            0.5
        );
        assert_eq!(
            double_array[double_array.lower_bound_only_asc(-11111111.1).unwrap()],
            -0.1
        );
        assert_eq!(
            double_array[double_array.upper_bound_only_asc(-11111111.1).unwrap()],
            -0.1
        );
        assert_eq!(double_array.lower_bound_only_asc(1.51), None);
        assert_eq!(double_array.upper_bound_only_asc(1.51), None);
    }
}
