use rand::Rng;

use cargo_snippet::snippet;
use rand;

// must rand=0.7.3
// box-muller法による標準正規分布に従った乱数
#[snippet("@rnorm")]
pub fn rnorm() -> f64 {
    let mut rng = rand::thread_rng();
    let x: f64 = rng.sample(rand::distributions::Standard);
    let y: f64 = rng.sample(rand::distributions::Standard);
    return (-2.0 * (1.0 - x).ln()).sqrt() * (2.0 * std::f64::consts::PI * y).cos();
}
