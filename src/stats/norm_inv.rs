use cargo_snippet::snippet;

// 正規分布の逆関数
#[snippet("@norm_inv")]
pub fn norm_inv(p: f64, avg: f64, std: f64) -> f64 {
    assert!(p <= 1.0);
    assert!(p >= 0.0);

    fn erfcinv(p: f64) -> f64 {
        let pp = if p < 1.0 { p } else { 2.0 - p };
        let t = (-2.0 * (pp / 2.0).ln()).sqrt();

        let mut x = -0.70711 * ((2.30753 + t * 0.27061) / (1.0 + t * (0.99229 + t * 0.04481)) - t);

        for _ in 0..2 {
            let err = erfc(x) - pp;
            x += err / (1.12837916709551257 * (-x * x).exp() - x * err);
        }

        return if p < 1.0 { x } else { -x };
    }

    fn erfc(mut x: f64) -> f64 {
        let cof: Vec<f64> = vec![
            -1.3026537197817094,
            6.4196979235649026e-1,
            1.9476473204185836e-2,
            -9.561514786808631e-3,
            -9.46595344482036e-4,
            3.66839497852761e-4,
            4.2523324806907e-5,
            -2.0278578112534e-5,
            -1.624290004647e-6,
            1.303655835580e-6,
            1.5626441722e-8,
            -8.5238095915e-8,
            6.529054439e-9,
            5.059343495e-9,
            -9.91364156e-10,
            -2.27365122e-10,
            9.6467911e-11,
            2.394038e-12,
            -6.886027e-12,
            8.94487e-13,
            3.13092e-13,
            -1.12708e-13,
            3.81e-16,
            7.106e-15,
            -1.523e-15,
            -9.4e-17,
            1.21e-16,
            -2.8e-17,
        ];
        let mut j = cof.len() - 1;
        let mut is_neg = false;
        let mut d = 0.0;
        let mut dd = 0.0;

        if x < 0.0 {
            x = -x;
            is_neg = true;
        }

        let t = 2.0 / (2.0 + x);
        let ty = 4.0 * t - 2.0;

        while j > 0 {
            let tmp = d;
            d = ty * d - dd + cof[j];
            dd = tmp;
            j -= 1;
        }

        let res = t * (-x * x + 0.5 * (cof[0] + ty * d) - dd).exp();
        return 1.0 - if is_neg { res - 1.0 } else { 1.0 - res };
    }

    return -1.4142135623730950488 * std * erfcinv(2.0 * p) + avg;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn norm_inv_test() {
        let norm_center = norm_inv(0.5, 0.0, 1.0);
        assert!(norm_center > -0.00000000000005);
        assert!(norm_center < 0.00000000000005);
        assert_eq!(norm_inv(0.908789, 40.0, 1.5), 42.00000200956616);
    }
}
