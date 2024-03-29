use cargo_snippet::snippet;

// 最大公約数
#[snippet("@gcd")]
#[snippet("@lcm")]
pub fn gcd(a: usize, b: usize) -> usize {
    let mut aa: usize = if a > b { a } else { b };
    let mut bb: usize = if a > b { b } else { a };
    while bb != 0 {
        let tmp = bb;
        bb = aa % tmp;
        aa = tmp;
    }

    aa
}

// 最小公倍数
#[snippet("@lcm")]
pub fn lcm(a: usize, b: usize) -> usize {
    a / gcd(a, b) * b
}

// 拡張ユークリッド ax+by=gcd(a,b)を解き、x,yを返す
#[snippet("@extgcd")]
#[snippet("@inverse_mod")]
pub fn extgcd<T: num::Num + Copy>(a: T, b: T) -> (T, T) {
    if b == T::zero() {
        return (T::one(), T::zero());
    }

    let d = extgcd(b, a % b);

    (d.1, d.0 - a / b * d.1)
}

#[cfg(test)]

mod test {
    use super::*;
    use proptest::prelude::*;
    #[test]
    fn gcd_test() {
        assert_eq!(gcd(121, 88), 11);
        assert_eq!(gcd(10500, 7455), 105);
        assert_eq!(gcd(1000000007, 2935623514300), 1);
        assert_eq!(gcd(4368123795, 321432138720), 105);
        assert_eq!(gcd(32114, 321432), 2);
    }

    #[test]
    fn lcm_test() {
        assert_eq!(lcm(10, 60), 60);
        assert_eq!(lcm(13, 19), 247);
        assert_eq!(lcm(3, 40), 120);
        assert_eq!(lcm(555333, 555333), 555333);
        assert_eq!(lcm(1267, 741238), 939148546);
    }

    #[test]
    fn extgcd_test() {
        assert_eq!(extgcd(111, 30), (3, -11));
    }

    proptest! {
      #[test]

      // gcd(a,b) * lcm(a,b) == a*bは常に成り立つ
      fn lcm_random(a :u16, b :u16) {
          let a = a as usize;
          let b = b as usize;
        prop_assert!(gcd(a,b) * lcm(a,b) == a*b);
      }

      #[test]
      fn extgcd_random(a :u16, b :u16) {
          let a = a as i64;
          let b = b as i64;

          let gcd_num = gcd(a as usize, b as usize) as i64;
          let x_y = extgcd(a, b);
          let x = x_y.0;
          let y = x_y.1;

        prop_assert!(a*x+b*y == gcd_num);
      }
    }
}
