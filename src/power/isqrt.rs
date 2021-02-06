use cargo_snippet::snippet;
use std::usize;

#[snippet("@IntSqrtPower")]
pub trait IntSqrtPower {
    fn sqrt(&self) -> usize;
}

#[snippet("@IntSqrtPower")]
impl IntSqrtPower for usize {
    fn sqrt(&self) -> usize {
        if *self <= 1 {
            return *self;
        }

        let mut max = 1 << 32;
        let mut min = 0;

        while max - min > 1 {
            let mid = (max + min) / 2;
            if mid * mid > *self {
                max = mid;
            } else {
                min = mid;
            }
        }

        min
    }
}

mod test {
    use super::*;
    use proptest::prelude::*;

    #[test]
    fn intSqrtPowerTest() {
        assert_eq!(500.sqrt(), 22);
        assert_eq!(10000.sqrt(), 100);
        assert_eq!(998001.sqrt(), 999);
        assert_eq!(9999999.sqrt(), 3162);
        assert_eq!(20000000000.sqrt(), 141421);
        assert_eq!((2147483648 * 2147483648).sqrt(), 2147483648);
    }

    proptest! {
      #[test]
      fn sqrt_random_num(a :u32) {
        let a = a as usize;

        prop_assert!((a*a).sqrt() == a);
      }
    }
}
