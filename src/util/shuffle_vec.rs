use rand::*;

pub fn shuffle_vec<T: Copy>(vec: &mut [T]) {
    for i in 0..vec.len() {
        let mut tmp: usize = random();
        tmp %= vec.len() - i;
        let to = i + tmp;

        let tmp = vec[i];
        vec[i] = vec[to];
        vec[to] = tmp;
    }
}
