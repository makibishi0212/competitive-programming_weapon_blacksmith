use cargo_snippet::snippet;

// f(x+2) = f(x) + f(x+1) (フィボナッチ数列)のような線形漸化式について、f(n)を高速で求める。
// f(x+k)の漸化式と、f(0)..f(x-1)の初期値を引数にとる。
// n: 求めたい項
// coefficients: f(x+k)の漸化式における、f(x+0)..f(x+k-1)の係数
// initial_value: f(k)の値を求めるために必要な、f(0)..f(k-1)の初期値。
#[snippet("@kitamasa")]
pub fn kitamasa<T: Copy + num::Integer + std::ops::AddAssign + num::CheckedMul>(
    zero_indexed_n: usize,
    coefficients: &[T],
    initial_value: &[T],
) -> T {
    let n = zero_indexed_n;
    if coefficients.len() != initial_value.len() {
        panic!();
    }

    if n < initial_value.len() {
        return initial_value[n];
    }

    let k = coefficients.len();

    let mut operations = vec![]; // 0なら2倍、1なら+1
    let mut tmp_n = n;
    while tmp_n != k {
        if tmp_n % 2 == 0 && tmp_n / 2 >= k {
            operations.push(0);
            tmp_n /= 2;
        } else {
            operations.push(1);
            tmp_n -= 1;
        }
    }

    let mut n_coefficients = coefficients.to_vec();

    fn calc_new_coefficients<T: Copy + num::Integer + std::ops::AddAssign + num::CheckedMul>(
        n_coefficients: &[T],
        initial_coefficients: &[T],
    ) -> Vec<T> {
        let k = n_coefficients.len();
        let mut new_n_coefficients = vec![T::zero(); k];
        for i in 0..k {
            if i == 0 {
                new_n_coefficients[i] = n_coefficients[k - 1] * initial_coefficients[0];
            } else {
                new_n_coefficients[i] =
                    n_coefficients[i - 1] + n_coefficients[k - 1] * initial_coefficients[i];
            }
        }

        new_n_coefficients
    }

    while operations.len() > 0 {
        let next = operations.pop().unwrap();

        match next {
            0 => {
                let mut new_n_coefficients = vec![T::zero(); k];

                let mut tmp_n_coefficients = n_coefficients.clone();
                for j in 0..k {
                    for i in 0..k {
                        new_n_coefficients[i] += n_coefficients[j] * tmp_n_coefficients[i];
                    }

                    tmp_n_coefficients = calc_new_coefficients(&tmp_n_coefficients, &coefficients);
                }

                n_coefficients = new_n_coefficients;
            }
            1 => {
                let new_n_coefficients = calc_new_coefficients(&n_coefficients, &coefficients);
                n_coefficients = new_n_coefficients;
            }
            _ => {}
        }
    }

    let mut value = T::zero();
    for i in 0..k {
        value += n_coefficients[i] * initial_value[i];
    }

    value
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn kitamasa_fibonacci_test() {
        let fibonacci_0 = kitamasa(0, &[1, 1], &[1, 1]);
        assert_eq!(fibonacci_0, 1);
        let fibonacci_1 = kitamasa(1, &[1, 1], &[1, 1]);
        assert_eq!(fibonacci_1, 1);
        let fibonacci_2 = kitamasa(2, &[1, 1], &[1, 1]);
        assert_eq!(fibonacci_2, 2);

        let fibonacci_3 = kitamasa(3, &[1, 1], &[1, 1]);
        assert_eq!(fibonacci_3, 3);

        let fibonacci_4 = kitamasa(4, &[1, 1], &[1, 1]);
        assert_eq!(fibonacci_4, 5);

        let fibonacci_9 = kitamasa(9, &[1, 1], &[1, 1]);
        assert_eq!(fibonacci_9, 55);

        let fibonacci_40 = kitamasa(40, &[1, 1], &[1, 1]);
        assert_eq!(fibonacci_40, 165580141);

        let fibonacci_50 = kitamasa(50, &[1usize, 1], &[1, 1]);
        assert_eq!(fibonacci_50, 20365011074);
    }

    #[test]
    fn four_terms_recurrence_formula_test() {
        // f(n)=f(n-4)+f(n-3)+2*f(n-2)+f(n-1)
        // f(0)=2,f(1)=3,f(2)=0,f(3)=1
        // 2,3,0,1,6,11,24,53,118,259

        let four_recurrence_4 = kitamasa(4, &[1, 1, 2, 1], &[2, 3, 0, 1]);
        assert_eq!(four_recurrence_4, 6);

        let four_recurrence_5 = kitamasa(5, &[1, 1, 2, 1], &[2, 3, 0, 1]);
        assert_eq!(four_recurrence_5, 11);

        let four_recurrence_6 = kitamasa(6, &[1, 1, 2, 1], &[2, 3, 0, 1]);
        assert_eq!(four_recurrence_6, 24);

        let four_recurrence_7 = kitamasa(7, &[1, 1, 2, 1], &[2, 3, 0, 1]);
        assert_eq!(four_recurrence_7, 53);

        let four_recurrence_8 = kitamasa(8, &[1, 1, 2, 1], &[2, 3, 0, 1]);
        assert_eq!(four_recurrence_8, 118);

        let four_recurrence_9 = kitamasa(9, &[1, 1, 2, 1], &[2, 3, 0, 1]);
        assert_eq!(four_recurrence_9, 259);

        let four_recurrence_20 = kitamasa(20, &[1, 1, 2, 1], &[2, 3, 0, 1]);
        assert_eq!(four_recurrence_20, 1557682);

        let four_recurrence_30 = kitamasa(30, &[1usize, 1, 2, 1], &[2, 3, 0, 1]);
        assert_eq!(four_recurrence_30, 4243127376);
    }
}
