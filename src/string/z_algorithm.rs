use cargo_snippet::snippet;

#[snippet("@z_algorithm")]
pub fn z_algorithm(s: &[char]) -> Vec<usize> {
    // sと同じ長さの配列を返す。
    // 返す配列aの各要素a[i]には、s[0..n)とs[i..n)との最長共通接頭辞の長さが入る。
    let mut z = vec![0; s.len()];
    z[0] = s.len();

    let mut i = 1;
    let mut j = 0;
    while i < s.len() {
        while i + j < s.len() && s[j] == s[i + j] {
            j += 1;
        }
        z[i] = j;

        if j == 0 {
            i += 1;
            continue;
        }

        let mut k = 1;
        while k < j && k + z[k] < j {
            z[i + k] = z[k];
            k += 1;
        }
        i += k;
        j -= k;
    }

    z
}

mod test {
    use super::*;

    #[test]
    fn z_algorithm_test() {
        assert_eq!(z_algorithm(&['a', 'b', 'a', 'b', 'a']), [5, 0, 3, 0, 1]);
        assert_eq!(
            z_algorithm(&['a', 'b', 'c', 'd', 'e', 'f']),
            [6, 0, 0, 0, 0, 0]
        );
        let mut all_a = vec![];
        let mut result = vec![];
        for i in 0..100000 {
            all_a.push('a');
            result.push(100000 - i);
        }
        assert_eq!(z_algorithm(&all_a), result);
    }
}
