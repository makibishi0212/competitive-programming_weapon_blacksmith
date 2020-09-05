use cargo_snippet::snippet;

// 最大公約数
#[snippet("@gcd")]
#[snippet("@lcm")]
fn gcd(a: u64, b: u64) -> u64 {
    let mut aa: u64 = if a > b { a } else { b };
    let mut bb: u64 = if a > b { b } else { a };
    while bb != 0 {
        let tmp = bb;
        bb = aa % tmp;
        aa = tmp;
    }
    return aa;
}

// 最小公倍数
#[snippet("@lcm")]
fn lcm(a: u64, b: u64) -> u64 {
    a / gcd(a, b) * b
}

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
