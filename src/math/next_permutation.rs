use cargo_snippet::snippet;

// 与えられた配列を辞書順で次のものに並び替える
// arrayの要素は相異なる必要がある
#[snippet("@next_permutation")]
pub fn next_permutation<T: std::cmp::PartialOrd>(array: &mut [T]) -> Option<usize> {
    let n = array.len();

    if n < 2 {
        return None;
    }

    // まずはarrayを後ろから見た時、それ以降降順に並んでいるインデックスの位置を特定する
    let mut desc_begin_index = n - 1;
    for i in (0..n - 1).rev() {
        if array[i] <= array[i + 1] {
            break;
        }
        desc_begin_index = i;
    }

    if desc_begin_index == 0 {
        array.reverse();
        return None;
    }

    desc_begin_index -= 1;

    let pivot = &array[desc_begin_index];
    let mut swap_to = n;
    for i in (desc_begin_index..n).rev() {
        if array[i] > *pivot {
            swap_to = i;
            break;
        }
    }
    array.swap(desc_begin_index, swap_to);
    array[desc_begin_index + 1..n].reverse();

    Some(desc_begin_index)
}

// 与えられた配列を辞書順で前のものに並び替える
// arrayの要素は相異なる必要がある
#[snippet("@prev_permutation")]
pub fn prev_permutation<T: std::cmp::PartialOrd>(array: &mut [T]) -> Option<usize> {
    let n = array.len();

    if n < 2 {
        return None;
    }

    // まずはarrayを後ろから見た時、それ以降昇順に並んでいるインデックスの位置を特定する
    let mut asc_begin_index = n - 1;
    for i in (0..n - 1).rev() {
        if array[i] >= array[i + 1] {
            break;
        }
        asc_begin_index = i;
    }

    if asc_begin_index == 0 {
        array.reverse();
        return None;
    }

    asc_begin_index -= 1;

    let pivot = &array[asc_begin_index];
    let mut swap_to = n;
    for i in (asc_begin_index..n).rev() {
        if array[i] < *pivot {
            swap_to = i;
            break;
        }
    }
    array.swap(asc_begin_index, swap_to);
    array[asc_begin_index + 1..n].reverse();

    Some(asc_begin_index)
}

#[cfg(test)]
mod test {
    use super::*;
    use proptest::prelude::*;

    #[test]
    fn next_permutation_test() {
        let mut v = vec![1];
        assert_eq!(next_permutation(&mut v), None);
        assert_eq!(v, vec![1]);

        let mut v = vec![1, 2, 3, 4, 5];
        assert_eq!(next_permutation(&mut v), Some(3));
        assert_eq!(v, vec![1, 2, 3, 5, 4]);
        assert_eq!(next_permutation(&mut v), Some(2));
        assert_eq!(v, vec![1, 2, 4, 3, 5]);
        assert_eq!(next_permutation(&mut v), Some(3));
        assert_eq!(v, vec![1, 2, 4, 5, 3]);

        let mut v = vec![1, 20, 300, 4000, 50000];
        assert_eq!(next_permutation(&mut v), Some(3));
        assert_eq!(v, vec![1, 20, 300, 50000, 4000]);
        assert_eq!(next_permutation(&mut v), Some(2));
        assert_eq!(v, vec![1, 20, 4000, 300, 50000]);
        assert_eq!(next_permutation(&mut v), Some(3));
        assert_eq!(v, vec![1, 20, 4000, 50000, 300]);

        let mut v = vec![-1, -20, -300, -4000, -50000];
        assert_eq!(next_permutation(&mut v), None);
        assert_eq!(v, vec![-50000, -4000, -300, -20, -1]);
        assert_eq!(next_permutation(&mut v), Some(3));
        assert_eq!(v, vec![-50000, -4000, -300, -1, -20]);
        assert_eq!(next_permutation(&mut v), Some(2));
        assert_eq!(v, vec![-50000, -4000, -20, -300, -1]);
    }

    #[test]
    fn prev_permutation_test() {
        let mut v = vec![2, 1];
        assert_eq!(prev_permutation(&mut v), Some(0));
        assert_eq!(v, vec![1, 2]);

        let mut v = vec![1, 2, 3, 4, 5];
        assert_eq!(prev_permutation(&mut v), None);
        assert_eq!(v, vec![5, 4, 3, 2, 1]);
        assert_eq!(prev_permutation(&mut v), Some(3));
        assert_eq!(v, vec![5, 4, 3, 1, 2]);
        assert_eq!(prev_permutation(&mut v), Some(2));
        assert_eq!(v, vec![5, 4, 2, 3, 1]);
    }

    proptest! {
      #[test]
      fn next_prev(a :u8, b :u8,count:u8) {
          let min = std::cmp::min(a,b);
          let max = std::cmp::max(a,b);

          let mut array = vec![];
          for i in min..max {
            array.push(i);
          }
          let array_origin = array.clone();

          for _ in 0..count {
            next_permutation(&mut array);
          }
          for _ in 0..count {
            prev_permutation(&mut array);
          }
        prop_assert_eq!(array, array_origin);
      }
    }
}
