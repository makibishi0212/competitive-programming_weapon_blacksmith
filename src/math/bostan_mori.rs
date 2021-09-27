use cargo_snippet::snippet;

// https://ikatakos.com/pot/programming_algorithm/number_theory/barlekamp_massey
// https://qiita.com/ryuhe1/items/da5acbcce4ac1911f47a

pub fn bostan_mori(zero_indexed_n: usize, coefficients: &[i128], initial_value: &[i128]) -> i128 {
    if coefficients.len() != initial_value.len() {
        panic!();
    }

    let mut coefficients = coefficients.to_vec();
    coefficients.reverse();

    // 元々の漸化式の母関数をP(x)/Q(x)で表現
    let mut Q: Vec<i128> = Vec::with_capacity(coefficients.len() + 1);
    Q.push(1);
    coefficients.iter().for_each(|c| {
        Q.push(*c * (-1));
    });

    fn convolution(a: &[i128], b: &[i128]) -> Vec<i128> {
        if a.is_empty() || b.is_empty() {
            return vec![];
        }
        let n = a.len() - 1;
        let m = b.len() - 1;
        let mut ans = vec![0; n + m + 1];
        for i in 0..n + 1 {
            for j in 0..m + 1 {
                ans[i + j] += a[i] * b[j];
            }
        }
        ans
    }

    let mut P = convolution(&Q, &initial_value);
    P.resize(initial_value.len(), 0);

    let mut n = zero_indexed_n;
    while n > 0 {
        let mut Q_minus = Q.clone();
        for i in 0..Q.len() {
            if i % 2 == 1 {
                Q_minus[i] = -Q_minus[i];
            }
        }

        // P(x)/Q(x)の分子分母にQ(-x)を畳み込みでかける
        let PQ_minus = convolution(&P, &Q_minus); // 分子 P(x)Q(-x)
        let QQ_minus = convolution(&Q, &Q_minus); // 分母 Q(x)Q(-x)

        let mut new_P = vec![0; Q.len() - 1];
        for i in 0..Q.len() - 1 {
            let num_index = if n % 2 == 0 { 2 * i } else { 2 * i + 1 };
            if num_index < PQ_minus.len() {
                new_P[i] = PQ_minus[num_index];
            }
        }
        let mut new_Q = vec![0; Q.len()];
        for i in 0..Q.len() {
            new_Q[i] = QQ_minus[2 * i];
        }

        P = new_P;
        Q = new_Q;

        n /= 2;
    }

    P[0] / Q[0]
}

mod test {
    use super::bostan_mori;

    #[test]
    fn bostan_mori_fibonacci_test() {
        let fibonacci_0 = bostan_mori(0, &[1, 1], &[1, 1]);
        assert_eq!(fibonacci_0, 1);
        let fibonacci_1 = bostan_mori(1, &[1, 1], &[1, 1]);
        assert_eq!(fibonacci_1, 1);
        let fibonacci_2 = bostan_mori(2, &[1, 1], &[1, 1]);
        assert_eq!(fibonacci_2, 2);

        let fibonacci_3 = bostan_mori(3, &[1, 1], &[1, 1]);
        assert_eq!(fibonacci_3, 3);

        let fibonacci_4 = bostan_mori(4, &[1, 1], &[1, 1]);
        assert_eq!(fibonacci_4, 5);

        let fibonacci_9 = bostan_mori(9, &[1, 1], &[1, 1]);
        assert_eq!(fibonacci_9, 55);

        let fibonacci_40 = bostan_mori(40, &[1, 1], &[1, 1]);
        assert_eq!(fibonacci_40, 165580141);

        let fibonacci_50 = bostan_mori(50, &[1, 1], &[1, 1]);
        assert_eq!(fibonacci_50, 20365011074);
    }

    #[test]
    fn four_terms_recurrence_formula_test() {
        // f(n)=f(n-4)+f(n-3)+2*f(n-2)+f(n-1)
        // f(0)=2,f(1)=3,f(2)=0,f(3)=1
        // 2,3,0,1,6,11,24,53,118,259

        let four_recurrence_4 = bostan_mori(4, &[1, 1, 2, 1], &[2, 3, 0, 1]);
        assert_eq!(four_recurrence_4, 6);

        let four_recurrence_5 = bostan_mori(5, &[1, 1, 2, 1], &[2, 3, 0, 1]);
        assert_eq!(four_recurrence_5, 11);

        let four_recurrence_6 = bostan_mori(6, &[1, 1, 2, 1], &[2, 3, 0, 1]);
        assert_eq!(four_recurrence_6, 24);

        let four_recurrence_7 = bostan_mori(7, &[1, 1, 2, 1], &[2, 3, 0, 1]);
        assert_eq!(four_recurrence_7, 53);

        let four_recurrence_8 = bostan_mori(8, &[1, 1, 2, 1], &[2, 3, 0, 1]);
        assert_eq!(four_recurrence_8, 118);

        let four_recurrence_9 = bostan_mori(9, &[1, 1, 2, 1], &[2, 3, 0, 1]);
        assert_eq!(four_recurrence_9, 259);

        let four_recurrence_20 = bostan_mori(20, &[1, 1, 2, 1], &[2, 3, 0, 1]);
        assert_eq!(four_recurrence_20, 1557682);

        let four_recurrence_30 = bostan_mori(30, &[1, 1, 2, 1], &[2, 3, 0, 1]);
        assert_eq!(four_recurrence_30, 4243127376);
    }
}
