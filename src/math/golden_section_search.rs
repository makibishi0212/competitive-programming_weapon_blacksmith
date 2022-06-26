use cargo_snippet::snippet;

#[snippet("@golden_section_search")]
pub fn golden_section_search<T: std::cmp::PartialOrd>(
    f: fn(f64) -> T,
    mut min: f64,
    mut max: f64,
    end_width: f64,
) -> f64 {
    assert!(max > min);
    let golden_ratio = (1.0 + (5.0f64).sqrt()) / 2.0;

    // 内分点計算
    let mut x_1 = (max - min) / (golden_ratio + 1.0) + min;
    let mut x_2 = (max - min) / golden_ratio + min;

    // 単峰なので、内分点の評価値が f(min) > f(x_1) > f(max)やf(min) < f(x_1) < f(max)のようになることはない
    assert!(f(min) > f(x_1) && f(min) > f(x_2) || f(min) < f(x_1) && f(min) < f(x_2));
    assert!(f(max) > f(x_1) && f(max) > f(x_2) || f(max) < f(x_1) && f(max) < f(x_2));

    // 上に凸かどうか
    let convex = if f(min) > f(x_1) { false } else { true };

    while (max - min) > end_width {
        // x_1が次の区間の中心になるかどうか
        let center_x_1 = if convex {
            f(x_1) > f(x_2)
        } else {
            f(x_1) < f(x_2)
        };

        if center_x_1 {
            max = x_2;
            x_2 = x_1;
            x_1 = (max - min) / (golden_ratio + 1.0) + min;
        } else {
            min = x_1;
            x_1 = x_2;
            x_2 = (max - min) / golden_ratio + min;
        }
    }

    (min + max) / 2.0
}

// 単峰でない可能性が考えられる場合用。適当なところで範囲の絞り込みを打ち切る
#[snippet("@golden_section_search_loose")]
pub fn golden_section_search_loose<T: std::cmp::PartialOrd>(
    f: fn(f64) -> T,
    mut min: f64,
    mut max: f64,
    end_width: f64,
) -> f64 {
    assert!(max > min);
    let golden_ratio = (1.0 + (5.0f64).sqrt()) / 2.0;

    // 内分点計算
    let mut x_1 = (max - min) / (golden_ratio + 1.0) + min;
    let mut x_2 = (max - min) / golden_ratio + min;

    // 上に凸かどうか
    let convex = if f(min) > f(x_1) { false } else { true };

    let mut count = 0;

    while (max - min) > end_width && count < 50 {
        // x_1が次の区間の中心になるかどうか
        let center_x_1 = if convex {
            f(x_1) > f(x_2)
        } else {
            f(x_1) < f(x_2)
        };

        if center_x_1 {
            max = x_2;
            x_2 = x_1;
            x_1 = (max - min) / (golden_ratio + 1.0) + min;
        } else {
            min = x_1;
            x_1 = x_2;
            x_2 = (max - min) / golden_ratio + min;
        }

        count += 1;
    }

    (min + max) / 2.0
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn golden_section_search_test() {
        // およそ0-2PIの範囲でcos(x)
        assert_eq!(
            golden_section_search(|x| { x.cos() }, 0.0, 6.28, 0.000000001),
            3.14159266372048
        );

        // およそ(-PI/2)-(PI/2)の範囲でcos(x)
        assert_eq!(
            golden_section_search(|x| { x.cos() }, -1.57, 1.57, 0.000000001),
            0.000000010246525816069812
        );

        // y=(x - 85.0) * (x - 85.0)
        assert_eq!(
            golden_section_search(
                |x| { (x - 85.0) * (x - 85.0) },
                -500000.0,
                500000.0,
                0.00000000001
            ),
            84.99999999999918
        );

        // loose
        assert_eq!(
            golden_section_search_loose(|x| { x.cos() }, 0.0, 6.28, 0.000000001),
            3.14159266372048
        );

        assert_eq!(
            golden_section_search_loose(|x| { x.cos() }, -1.57, 1.57, 0.000000001),
            0.000000010246525816069812
        );

        assert_eq!(
            golden_section_search_loose(
                |x| { (x - 85.0) * (x - 85.0) },
                -500000.0,
                500000.0,
                0.00000000001
            ),
            84.99999821798717
        );
    }
}
