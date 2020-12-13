use cargo_snippet::snippet;
// 約数系

// エラストテネスの篩
#[snippet("@eratosthenes")]
#[snippet("@prime_factorize")]
fn eratosthenes(n: usize) -> Vec<usize> {
    let mut sieve = Vec::with_capacity(n - 1);
    for i in 2..=n {
        sieve.push(i);
    }
    for i in 2..=(n as f64).sqrt() as usize {
        if sieve[i - 2] < i {
            continue;
        }
        for j in (i * i..=n).step_by(i) {
            if sieve[j - 2] == j {
                sieve[j - 2] = i;
            }
        }
    }

    let mut result = Vec::with_capacity(n - 1);
    for i in 2..=n {
        if sieve[i - 2] == i {
            result.push(i);
        }
    }

    result
}

// 素因数分解
#[snippet("@prime_factorize")]
fn prime_factorize(mut num: usize) -> std::collections::HashMap<usize, usize> {
    let mut primes = eratosthenes(num);
    let mut divisors = std::collections::HashMap::new();

    let mut now = primes.len();
    while num != 1 {
        now -= 1;
        let now_prime = primes[now];
        let mut count = 0;

        while num % now_prime == 0 {
            num /= now_prime;
            count += 1;
        }

        if count != 0 {
            divisors.insert(now_prime, count);
        }
    }

    divisors
}

// 約数列挙
#[snippet("@enumerate_divisor")]
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
        );

        assert_eq!(
            eratosthenes(809),
            vec![
                2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79,
                83, 89, 97, 101, 103, 107, 109, 113, 127, 131, 137, 139, 149, 151, 157, 163, 167,
                173, 179, 181, 191, 193, 197, 199, 211, 223, 227, 229, 233, 239, 241, 251, 257,
                263, 269, 271, 277, 281, 283, 293, 307, 311, 313, 317, 331, 337, 347, 349, 353,
                359, 367, 373, 379, 383, 389, 397, 401, 409, 419, 421, 431, 433, 439, 443, 449,
                457, 461, 463, 467, 479, 487, 491, 499, 503, 509, 521, 523, 541, 547, 557, 563,
                569, 571, 577, 587, 593, 599, 601, 607, 613, 617, 619, 631, 641, 643, 647, 653,
                659, 661, 673, 677, 683, 691, 701, 709, 719, 727, 733, 739, 743, 751, 757, 761,
                769, 773, 787, 797, 809
            ]
        );
    }

    #[test]
    fn test_divisor() {
        assert_eq!(enumerate_divisor(2), vec![1, 2]);

        assert_eq!(
            enumerate_divisor(400),
            vec![1, 2, 4, 5, 8, 10, 16, 20, 25, 40, 50, 80, 100, 200, 400]
        );
    }

    #[test]
    fn test_prime_factorize() {
        let mut result840 = std::collections::HashMap::new();
        result840.insert(2, 3);
        result840.insert(3, 1);
        result840.insert(5, 1);
        result840.insert(7, 1);
        assert_eq!(prime_factorize(840), result840);

        let mut result7560 = std::collections::HashMap::new();
        result7560.insert(2, 3);
        result7560.insert(3, 3);
        result7560.insert(5, 1);
        result7560.insert(7, 1);
        assert_eq!(prime_factorize(7560), result7560);

        let mut result999773 = std::collections::HashMap::new();
        result999773.insert(999773, 1);
        assert_eq!(prime_factorize(999773), result999773);
    }
}
