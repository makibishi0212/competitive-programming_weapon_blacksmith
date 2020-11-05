use cargo_snippet::snippet;
use std::collections::VecDeque;
// 約数系

// エラストテネスの篩
fn eratosthenes(n: usize) -> Vec<usize> {
    let mut res = Vec::with_capacity(n - 1);
    for i in 2..=n {
        res.push(i);
    }
    for i in 2..=(n as f64).sqrt() as usize {
        if res[i - 2] < i {
            continue;
        }
        for j in (i * i..=n).step_by(i) {
            if res[j - 2] == j {
                res[j - 2] = i;
            }
        }
    }

    res.sort();
    res.dedup();

    res
}

// 約数列挙
fn enumerate_divisor(n: usize) -> Vec<usize> {
    let mut i = 1;

    let mut divisors = vec![];

    while i * i <= n {
        if n % i == 0 {
            divisors.push(i);
            if i != n / i {
                divisors.push(n / i);
            }
        }
        i += 1;
    }

    divisors.sort();

    divisors
}

mod test {
    use super::*;
    #[test]
    fn test_eratosthenes() {
        assert_eq!(eratosthenes(2), vec![2]);
        assert_eq!(eratosthenes(3), vec![2, 3]);
        assert_eq!(
            eratosthenes(199),
            vec![
                2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79,
                83, 89, 97, 101, 103, 107, 109, 113, 127, 131, 137, 139, 149, 151, 157, 163, 167,
                173, 179, 181, 191, 193, 197, 199
            ]
        );

        assert_eq!(
            eratosthenes(200),
            vec![
                2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79,
                83, 89, 97, 101, 103, 107, 109, 113, 127, 131, 137, 139, 149, 151, 157, 163, 167,
                173, 179, 181, 191, 193, 197, 199
            ]
        )
    }

    #[test]
    fn test_divisor() {
        assert_eq!(enumerate_divisor(2), vec![1, 2]);

        assert_eq!(
            enumerate_divisor(400),
            vec![1, 2, 4, 5, 8, 10, 16, 20, 25, 40, 50, 80, 100, 200, 400]
        );
    }
}
