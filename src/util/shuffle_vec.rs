use cargo_snippet::snippet;
use rand::*;

#[snippet("@shuffle_vec")]
pub fn shuffle_vec<T: Copy>(vec: &mut [T]) {
    for i in 0..vec.len() {
        let mut tmp: usize = random();
        tmp %= vec.len() - i;
        let to = i + tmp;

        vec.swap(i, to)
    }
}
