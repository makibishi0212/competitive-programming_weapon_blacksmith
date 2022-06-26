use cargo_snippet::snippet;
use std::{cmp, usize};

#[snippet("@IntSqrtPower")]
#[snippet("@prime_factorize")]
pub trait IntSqrtPower {
    fn sqrt(&self) -> usize;
}

#[snippet("@IntSqrtPower")]
#[snippet("@prime_factorize")]
impl IntSqrtPower for usize {
    fn sqrt(&self) -> usize {
        if *self <= 1 {
            return *self;
        }

        let mut max = cmp::min(1 << 32, *self);
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

#[cfg(test)]
mod test {
    use super::*;
    use proptest::prelude::*;

    #[test]
    fn int_sqrt_power_test() {
        assert_eq!(4.sqrt(), 2);
        assert_eq!(9.sqrt(), 3);
        assert_eq!(10.sqrt(), 3);
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
