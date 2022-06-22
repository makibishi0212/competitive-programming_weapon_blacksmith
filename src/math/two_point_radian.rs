use cargo_snippet::snippet;
// 2点間の角度(ラジアン)
#[snippet("@two_point_radian")]
pub fn two_point_radian(p1: (f64, f64), p2: (f64, f64)) -> f64 {
    (p2.1 - p1.1).atan2(p2.0 - p1.0)
}

mod test {
    use crate::math::two_point_radian;
    use proptest::prelude::*;

    use super::*;

    #[test]
    fn two_point_radian_test() {
        let rad = two_point_radian((0.0, 0.0), (1.0, 1.0));
        let deg = rad * 180.0 / std::f64::consts::PI;
        assert_eq!(deg, 45.0);

        let origin_deg = 102.0;
        let origin_rad = origin_deg * std::f64::consts::PI / 180.0;
        let rad = two_point_radian((0.0, 0.0), (origin_rad.cos(), origin_rad.sin()));
        let deg = rad * 180.0 / std::f64::consts::PI;
        assert_eq!(deg, origin_deg);

        let origin_deg = 73.0;
        let origin_rad = origin_deg * std::f64::consts::PI / 180.0;
        let rad = two_point_radian((0.0, 0.0), (origin_rad.cos(), origin_rad.sin()));
        let deg = rad * 180.0 / std::f64::consts::PI;
        assert_eq!(deg, origin_deg);

        let origin_deg = 244.0;
        let origin_rad = origin_deg * std::f64::consts::PI / 180.0;
        let rad = two_point_radian((0.0, 0.0), (origin_rad.cos(), origin_rad.sin()));
        let deg = rad * 180.0 / std::f64::consts::PI;
        assert_eq!(deg.round(), (origin_deg - 360.0).round());

        let origin_deg = 303.0;
        let origin_rad = origin_deg * std::f64::consts::PI / 180.0;
        let rad = two_point_radian((0.0, 0.0), (origin_rad.cos(), origin_rad.sin()));
        let deg = rad * 180.0 / std::f64::consts::PI;
        assert_eq!(deg.round(), (origin_deg - 360.0).round());

        let rad = two_point_radian((0.0, 0.0), (3.0_f64.sqrt(), 1.0));
        let deg = rad * 180.0 / std::f64::consts::PI;
        assert_eq!(deg.round(), 30.0);
    }

    proptest! {
      #[test]
      fn next_prev(float_num :f64) {
        let origin_deg = float_num.floor() % 90.0;
        let origin_rad = origin_deg * std::f64::consts::PI / 180.0;
        let rad = two_point_radian((0.0, 0.0), (origin_rad.cos(), origin_rad.sin()));
        let deg = rad * 180.0 / std::f64::consts::PI;
        assert_eq!(deg.round(), origin_deg.round());
      }
    }
}
