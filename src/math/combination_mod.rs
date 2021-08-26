use std::process::Output;

use crate::math::gcd::extgcd;
use cargo_snippet::snippet;

#[snippet("@power_mod")]
#[snippet("@prime_inverse_mod")]
#[snippet("@prime_combination_mod")]
pub fn power_mod<
    T: num::Unsigned + std::ops::BitAnd<Output = T> + std::ops::Shl<Output = T> + Copy,
>(
    base: T,
    exp: T,
    modulo: T,
) -> T {
    let one: T = num::one();
    let two: T = one + one;

    if exp == num::zero() {
        return num::one();
    } else if exp & one == num::zero() {
        let base = power_mod(base, exp / two, modulo) % modulo;
        return (base * base) % modulo;
    } else {
        return power_mod(base, exp - one, modulo) % modulo * base % modulo;
    }
}

#[snippet("@prime_inverse_mod")]
#[snippet("@prime_combination_mod")]
// 逆元
pub fn prime_inverse_mod<
    T: num::Unsigned + std::ops::BitAnd<Output = T> + std::ops::Shl<Output = T> + Copy,
>(
    element: T,
    prime_modulo: T,
) -> T {
    // フェルマーの小定理からpが素数なら a^(p-1) = 1
    // よってa*a^(p-2) = 1 より a^(p-2)がaの逆元
    let one: T = num::one();
    let two: T = one + one;
    power_mod(element, prime_modulo - two, prime_modulo) % prime_modulo
}

#[snippet("@inverse_mod")]
// must gcd(element, modulo) = 1
pub fn inverse_mod<
    T: num::Unsigned + std::cmp::PartialOrd + num::ToPrimitive + num::FromPrimitive + Copy,
>(
    element: T,
    modulo: T,
) -> T {
    let one: T = num::one();
    assert!(one <= modulo);
    let i_element = element.to_isize().unwrap();
    let i_modulo = modulo.to_isize().unwrap();
    let (x, _) = extgcd(i_element, i_modulo);
    let ans = (x + i_modulo) % i_modulo;
    let ans = T::from_isize(ans).unwrap();
    assert!(ans * element % modulo == num::one()); // ansが逆数になっていないならエラー
    ans
}

#[snippet("@permutation_mod")]
#[snippet("@prime_combination_mod")]
pub fn permutation_mod<
    T: num::Unsigned
        + num::ToPrimitive
        + num::FromPrimitive
        + std::ops::MulAssign<T>
        + std::ops::RemAssign<T>
        + Copy,
>(
    m: T,
    n: T,
    modulo: T,
) -> T {
    // m P n = m! / (m - n)!
    //       = m*(m-1)*(m-2)*...*(m-n+1)
    let mut numerator: T = num::one();
    let len = n.to_usize().unwrap();
    for i in 0..len {
        numerator *= m - T::from_usize(i).unwrap();
        numerator %= modulo;
    }

    numerator
}

#[snippet("@factorial_mod")]
#[snippet("@prime_combination_mod")]
pub fn factorial_mod<
    T: num::Unsigned
        + num::ToPrimitive
        + num::FromPrimitive
        + std::ops::MulAssign<T>
        + std::ops::RemAssign<T>
        + Copy,
>(
    n: T,
    modulo: T,
) -> T {
    // n!
    let mut element: T = num::one();
    let len = (n + num::one()).to_usize().unwrap();
    for j in 1..len {
        element *= T::from_usize(j).unwrap();
        element %= modulo;
    }

    element
}

#[snippet("@prime_combination_mod")]
pub fn prime_combination_mod(m: usize, n: usize, prime_modulo: usize) -> usize {
    // m C n = m! / ( n! * (m - n)! )
    //       = m*(m-1)*(m-2)*...*(m-n+1)/n!
    //       = m*(m-1)*(m-2)*...*(m-n+1) * (n!)^-1
    let permutation = permutation_mod(m, n, prime_modulo);

    // n!
    let mut element = 1;
    for j in 1..(n + 1) {
        element *= j;
        element %= prime_modulo;
    }
    permutation * prime_inverse_mod(element, prime_modulo) % prime_modulo
}

const LARGE_PRIME: usize = 1_000_000_007;
const LARGE_PRIME2: usize = 2_147_483_647;

#[test]
fn power_mod_test() {
    assert_eq!(power_mod(2, 5, LARGE_PRIME), 32);
    assert_eq!(power_mod(3, 12, LARGE_PRIME), 531441);
    assert_eq!(power_mod(10, 6, LARGE_PRIME), 1000000);
    assert_eq!(power_mod(10, 10, LARGE_PRIME), 999999937);
    assert_eq!(power_mod(10, 12, LARGE_PRIME), 999993007);
    assert_eq!(power_mod(10, 20, LARGE_PRIME), 4900);
    assert_eq!(power_mod(7, 7777, LARGE_PRIME), 199711806);
    assert_eq!(power_mod(13, 12, LARGE_PRIME), 84959395);
    assert_eq!(power_mod(13, 12, LARGE_PRIME2), 35036178);
    assert_eq!(power_mod(19, 11, LARGE_PRIME2), 8466704);
    assert_eq!(power_mod(7, 0, LARGE_PRIME), 1);
}

#[test]
fn prime_inverse_mod_test() {
    assert_eq!(prime_inverse_mod(700u64, 11), 8);
    assert_eq!(prime_inverse_mod(3u64, 2), 1);
    assert_eq!(prime_inverse_mod(1u64, 2), 1);
    assert_eq!(prime_inverse_mod(1u64, 11), 1);
    assert_eq!(prime_inverse_mod(1u64, 53), 1);
    assert_eq!(prime_inverse_mod(1, LARGE_PRIME), 1);
    assert_eq!((99 * prime_inverse_mod(99, LARGE_PRIME)) % LARGE_PRIME, 1);
    assert_eq!((558 * prime_inverse_mod(558, LARGE_PRIME)) % LARGE_PRIME, 1);
    assert_eq!(
        (77777 * prime_inverse_mod(77777, LARGE_PRIME)) % LARGE_PRIME,
        1
    );
    assert_eq!(
        (4321321 * prime_inverse_mod(4321321, LARGE_PRIME)) % LARGE_PRIME,
        1
    );
}

#[test]
fn inverse_mod_test() {
    // prime_inverse_modのテストは当然通る
    assert_eq!(inverse_mod(700, 11), 8usize);
    assert_eq!(inverse_mod(3, 2), 1usize);
    assert_eq!(inverse_mod(1, 2), 1usize);
    assert_eq!(inverse_mod(1, 11), 1usize);
    assert_eq!(inverse_mod(1, 53), 1usize);
    assert_eq!(inverse_mod(1, LARGE_PRIME), 1);
    assert_eq!(
        (99 * inverse_mod(99, LARGE_PRIME) as i64) % LARGE_PRIME as i64,
        1
    );
    assert_eq!((558 * inverse_mod(558, LARGE_PRIME)) % LARGE_PRIME, 1);
    assert_eq!((77777 * inverse_mod(77777, LARGE_PRIME)) % LARGE_PRIME, 1);
    assert_eq!(
        (4321321 * inverse_mod(4321321, LARGE_PRIME)) % LARGE_PRIME,
        1
    );

    assert_eq!(inverse_mod(2, 11), 6usize); // 2*6 % 11 = 1
    assert_eq!(inverse_mod(6, 11), 2usize); // 2*6 % 11 = 1
    assert_eq!(inverse_mod(2, 9), 5usize); // 2*5 % 9 = 1
    assert_eq!(inverse_mod(8, 9), 8usize); // 8*8 % 9 = 1
    assert_eq!(inverse_mod(8, 9), 8usize); // 8*8 % 9 = 1

    assert_eq!(inverse_mod(12, 125), 73usize); // 12*73 % 125 = 1
    assert_eq!(inverse_mod(12521, 5736), 257usize); // 12521*257 % 5736 = 1
}

#[test]
fn factorial_mod_test() {
    assert_eq!(factorial_mod(5, LARGE_PRIME), 120);
    assert_eq!(factorial_mod(10, LARGE_PRIME), 3628800);
    assert_eq!(factorial_mod(40, LARGE_PRIME), 799434881);
}

#[test]
fn prime_combination_mod_test() {
    assert_eq!(prime_combination_mod(5, 0, LARGE_PRIME), 1);
    assert_eq!(prime_combination_mod(5, 1, LARGE_PRIME), 5);
    assert_eq!(prime_combination_mod(5, 2, LARGE_PRIME), 10);
    assert_eq!(prime_combination_mod(5, 3, LARGE_PRIME), 10);
    assert_eq!(prime_combination_mod(5, 4, LARGE_PRIME), 5);
    assert_eq!(prime_combination_mod(5, 5, LARGE_PRIME), 1);
    assert_eq!(prime_combination_mod(4, 2, LARGE_PRIME), 6);
    assert_eq!(prime_combination_mod(7, 4, LARGE_PRIME), 35);
    assert_eq!(prime_combination_mod(10, 10, LARGE_PRIME), 1);
    assert_eq!(prime_combination_mod(20, 10, LARGE_PRIME), 184756);
    assert_eq!(prime_combination_mod(777, 77, LARGE_PRIME), 494594013);
}
