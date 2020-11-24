use cargo_snippet::snippet;

// floor_sum Σ_{0}^{n-1} floor((a*i+b)/m) を計算する。
#[snippet("@floor_sum")]
fn floor_sum(n: i64, m: i64, mut a: i64, mut b: i64) -> i64 {
    let mut ans = 0;

    if a >= m {
        let q_a = a / m;
        ans += n * (n - 1) * q_a / 2;
        a %= m;
    }
    if b >= m {
        let q_b = b / m;
        ans += n * q_b;
        b %= m;
    }

    let y_max = (a * n + b) / m;
    let x_max = y_max * m - b;

    if y_max == 0 {
        return ans;
    }

    // (x_max + a - 1) / a) はceil(x_max/a)
    ans += (n - (x_max + a - 1) / a) * y_max;
    ans += floor_sum(y_max, a, m, (a - x_max % a) % a);

    ans
}

mod test {
    use super::*;
    use proptest::prelude::*;

    #[test]
    fn floor_sum_test() {
        assert_eq!(floor_sum(0, 1, 0, 0), 0);
        assert_eq!(floor_sum(1, 1, 1, 1), 1);
        assert_eq!(floor_sum(2, 1, 1, 1), 3);
        assert_eq!(floor_sum(11, 1000, 1000, 0), 55);
        assert_eq!(floor_sum(100, 999999999, 999999999, 0), 4950);
        assert_eq!(floor_sum(100, 100, 10, 0), 450);
        assert_eq!(floor_sum(332955, 5590132, 2231, 999423), 22014575);
    }
}
