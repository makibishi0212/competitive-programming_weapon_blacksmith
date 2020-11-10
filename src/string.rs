use cargo_snippet::snippet;

fn lcp_array() {
    todo!();
}

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

#[snippet("@suffix_array")]
pub fn suffix_array(s: &[char]) -> Vec<usize> {
    // 長さnの文字列sの接尾辞(s[i..n),iは0..n-1)としてあり得るものを、辞書順にソートして返す。
    // メモリを無駄に食わないように、文字列そのものの代わりにsuffixの開始位置を返す。
    // SA-IS法 O(n)

    let mut max_s_i = 0; // 最大のchar to usize
    let s_i: Vec<usize> = s
        .iter()
        .map(|&c| {
            let i = c as usize;
            max_s_i = std::cmp::max(max_s_i, i);
            i
        })
        .collect();

    sa_is(&s_i, max_s_i)
}

#[snippet("@suffix_array")]
fn sa_is(s_i: &[usize], max_s_i: usize) -> Vec<usize> {
    let n = s_i.len();
    match n {
        0 => return vec![],
        1 => return vec![0],
        2 => {
            return if s_i[0] < s_i[1] {
                vec![0, 1]
            } else {
                vec![1, 0]
            }
        }
        _ => (),
    }

    // 辞書式順序でsがtより前にある場合、s < tと書くことにする。 ex) king < kong, b < ba
    // L型: s[i..] > s[i+1..] (s[i..] は s[i+1..]よりも辞書的に前にある)
    // S型: s[i..] <= s[i+1..] (s[i..] は s[i+1..]と同一かより後ろにある)
    let mut is_L = vec![true; n]; // インデックスiがLならtrue、Sならfalse

    for i in (0..n - 1).rev() {
        is_L[i] = if s_i[i] == s_i[i + 1] {
            is_L[i + 1]
        } else {
            s_i[i] >= s_i[i + 1]
        };
    }

    let mut char_L_count = vec![0; max_s_i + 1]; // L型として現れる各文字の出現回数を格納する(HashMapだと順序が保証されないので配列でやる)
    let mut char_S_count = vec![0; max_s_i + 1]; // S型として現れる各文字の出現回数を格納する
    for i in 0..n {
        if is_L[i] {
            char_L_count[s_i[i]] += 1;
        } else {
            char_S_count[s_i[i]] += 1;
        }
    }
    let mut char_ranges = vec![(0, 0); max_s_i + 1]; // 各文字が取る範囲　これは閉区間[a,b)
    let mut last = 0;
    for c in 0..=max_s_i {
        let c_total = char_L_count[c] + char_S_count[c];
        if c_total != 0 {
            char_ranges[c] = (last, last + c_total);
            last += c_total;
        }
    }

    // s[i..]がS型で、かつs[i-1..] がL型のiをLMS(LeftMostS)という。
    // LMSの位置を記録するとともにLMSの配列を生成する。
    let mut lms_index = vec![0; n];
    let mut lms_count = 0;
    for i in 1..n {
        if is_L[i - 1] && !is_L[i] {
            lms_count += 1;
            lms_index[i] = lms_count;
        }
    }
    let mut lms = Vec::with_capacity(lms_count);
    for i in 1..n {
        if lms_index[i] != 0 {
            lms.push(i);
        }
    }

    let mut sa = vec![0; n];

    induced_sort(&mut sa, &s_i, &lms, &char_L_count, &char_ranges);

    if lms_count > 0 {
        let mut sorted_lms = Vec::with_capacity(lms_count);
        for &i in &sa {
            if lms_index[i - 1] != 0 {
                sorted_lms.push(i - 1);
            }
        }

        // 各LMS部分文字列に番号を振ったもの。同一のLMS部分文字列であれば同じ番号がつく
        let mut lms_part_nums = vec![0; lms_count];
        let mut max_lms_part_index = 0;

        // LMS部分文字列同士の比較
        for i in 1..lms_count {
            let lms1_start = sorted_lms[i - 1];
            let lms1_end = if lms_index[lms1_start] == lms_count {
                n
            } else {
                lms[lms_index[lms1_start]]
            };

            let lms2_start = sorted_lms[i];
            let lms2_end = if lms_index[lms2_start] == lms_count {
                n
            } else {
                lms[lms_index[lms2_start]]
            };

            let same = if (lms1_end - lms1_start) != (lms2_end - lms2_start) {
                // 長さが違うなら、同じであるはずがない
                false
            } else {
                // 長さが同じなら、仕方がないので1文字ずつ比較する
                let mut same_all = true;
                for o in 0..(lms1_end - lms1_start) {
                    same_all = s_i[lms1_start + o] == s_i[lms2_start + o];
                    if !same_all {
                        break;
                    }
                }

                same_all
            };

            if !same {
                max_lms_part_index += 1;
            }
            lms_part_nums[lms_index[sorted_lms[i]] - 1] = max_lms_part_index;
        }
        let lms_part_sa = sa_is(&lms_part_nums, max_lms_part_index);
        for i in 0..lms_count {
            sorted_lms[i] = lms[lms_part_sa[i]];
        }

        induced_sort(&mut sa, &s_i, &sorted_lms, &char_L_count, &char_ranges);
    }

    sa.iter().map(|index1| index1 - 1).collect()
}

// SA-IS法の内部で使われるソート
#[snippet("@suffix_array")]
fn induced_sort(
    sa: &mut [usize],
    s_i: &[usize],
    lms: &[usize],
    char_L_count: &[usize],
    char_ranges: &[(usize, usize)],
) {
    let n = s_i.len();
    for i in sa.iter_mut() {
        *i = 0;
    }

    // (saのそのインデックスの先頭の文字のs_i[i], L型かどうか)
    let mut index_to_info = vec![(0, true); n];
    let mut checked = vec![false; char_L_count.len()];
    for i in 0..n {
        let c = s_i[i];
        let c_range = char_ranges[c];
        let mut l_count = char_L_count[c];
        if !checked[c] {
            for j in c_range.0..c_range.1 {
                let is_L = if l_count != 0 { true } else { false };
                if l_count != 0 {
                    l_count -= 1;
                };
                index_to_info[j] = (c, is_L);
            }
            checked[c] = true;
        }
    }

    // 0を未設定として使いたいので、saは1-indexedにする。

    // (1) saをLMSのインデックスで埋める
    let mut now_char_index = vec![std::usize::MAX; char_L_count.len()];

    for &i in lms.iter().rev() {
        let c = s_i[i];
        now_char_index[c] = if now_char_index[c] == std::usize::MAX {
            char_ranges[c].1
        } else {
            now_char_index[c]
        } - 1;
        sa[now_char_index[c]] = i + 1;
    }

    // (2) 正順にL型のインデックスを詰める
    let mut char_insert_count = vec![0; char_L_count.len()];

    // 一番最初だけ予め埋めておく
    let c = s_i[n - 1];
    sa[char_ranges[c].0 + char_insert_count[c]] = n;
    char_insert_count[c] += 1;

    for i in 0..n {
        if sa[i] < 2 {
            continue;
        }

        let target_index = sa[i] - 2;
        let target_c = s_i[target_index];

        let target_start_index = char_ranges[target_c].0;
        let to_index = target_start_index + char_insert_count[target_c];
        let to_is_L = index_to_info[to_index].1;
        if to_is_L {
            sa[to_index] = target_index + 1;
            char_insert_count[target_c] += 1;
        }
    }

    // (3) 逆順にS型のインデックスを詰める
    char_insert_count = vec![0; char_L_count.len()];
    for i in (0..n).rev() {
        if sa[i] < 2 {
            continue;
        }

        let target_index = sa[i] - 2;
        let target_c = s_i[target_index];

        let target_end_index = char_ranges[target_c].1 - 1;
        let to_index = target_end_index - char_insert_count[target_c];

        let to_is_S = !index_to_info[to_index].1;
        if to_is_S {
            sa[to_index] = target_index + 1;
            char_insert_count[target_c] += 1;
        }
    }
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

    #[test]
    fn suffix_array_test() {
        let str_1 = vec![
            'm', 'm', 'i', 'i', 's', 's', 'i', 'i', 's', 's', 'i', 'i', 'p', 'p', 'i', 'i',
        ];
        assert_eq!(
            suffix_array(&str_1),
            [15, 14, 10, 6, 2, 11, 7, 3, 1, 0, 13, 12, 9, 5, 8, 4]
        );

        let str_2 = vec!['a', 'b', 'r', 'a', 'c', 'a', 'd', 'a', 'b', 'r', 'a'];
        assert_eq!(suffix_array(&str_2), [10, 7, 0, 3, 5, 8, 1, 4, 6, 9, 2]);

        let str_3 = vec!['a', 'b', 'c', 'b', 'c', 'b', 'a'];
        assert_eq!(suffix_array(&str_3), [6, 0, 5, 3, 1, 4, 2]);

        let str_4 = vec!['a', 'a', 'a', 'a', 'a', 'a', 'a'];
        assert_eq!(suffix_array(&str_4), [6, 5, 4, 3, 2, 1, 0]);

        let mut str_5 = vec![];
        for i in 0..200000 {
            str_5.push('a');
        }
        let mut sa_5 = vec![];
        for i in (0..200000).rev() {
            sa_5.push(i);
        }

        assert_eq!(suffix_array(&str_5), sa_5);
    }
}
