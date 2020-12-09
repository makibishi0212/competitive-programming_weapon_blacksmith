use cargo_snippet::snippet;

#[snippet("@ceil")]
fn ceil(top: usize, bottom: usize) -> usize {
    (top + bottom - 1) / bottom
}

mod test {
    use super::*;
    use proptest::prelude::*;

    #[test]
    fn ceil_test() {
        assert_eq!(ceil(6, 2), 3);
        assert_eq!(ceil(7, 2), 4);
        assert_eq!(ceil(8, 2), 4);
        assert_eq!(ceil(10000, 2), 5000);
        assert_eq!(ceil(10001, 2), 5001);
    }

    proptest! {
      #[test]
      fn ceil_random_num(a :u16, b :u16) {
        let a = a as usize+1;
        let b = b as usize;
        let big = std::cmp::max(a,b);
        let small = std::cmp::min(a,b);
        if big % small != 0 {
            prop_assert!(ceil(big, small) == big/small+1);
        }else {
            prop_assert!(ceil(big, small) == big/small);
        }
      }
    }
}
