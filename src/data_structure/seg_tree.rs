use cargo_snippet::snippet;

#[snippet("@SegTree")]
pub struct SegTree<T: std::clone::Clone + std::fmt::Debug> {
    // operation(a,b) = operation(b,a)、operation(a,operation(b,c)) = operation(operation(a,b),c)の必要がある
    operation: fn(&T, &T) -> T,
    segment_array: Vec<Option<T>>,
}

#[snippet("@SegTree")]
impl<T: std::clone::Clone + std::fmt::Debug> SegTree<T> {
    pub fn new(array: Vec<T>, operation: fn(&T, &T) -> T) -> SegTree<T> {
        let origin_length = array.len();
        let mut all_array_length = 1;
        while origin_length * 2 > all_array_length {
            all_array_length <<= 1;
        }

        let mut segment_array: Vec<Option<T>> = vec![None; all_array_length];
        let mut target_index = all_array_length >> 1;

        // 最初に元々の値を埋める
        for i in 0..origin_length {
            segment_array[i + target_index] = Some(array[i].clone());
        }

        // 元々の値を使って区間の値を埋めていく
        target_index -= 1;
        while target_index != 0 {
            let left_index = target_index << 1;
            let right_index = (target_index << 1) + 1;

            let value = if let Some(left) = &segment_array[left_index] {
                if let Some(right) = &segment_array[right_index] {
                    Some(operation(&left, &right))
                } else {
                    Some(left.clone())
                }
            } else {
                if let Some(right) = &segment_array[right_index] {
                    Some(right.clone())
                } else {
                    None
                }
            };
            segment_array[target_index] = value;
            target_index -= 1;
        }

        SegTree {
            operation,
            segment_array,
        }
    }

    // [start_index,end_index)でクエリに答える
    pub fn query(&self, start_index: usize, end_index: usize) -> T {
        if start_index >= end_index {
            panic!();
        }
        let origin_start_index = self.segment_array.len() >> 1;
        let internal_start = origin_start_index + start_index;
        let internal_end = origin_start_index + end_index - 1;

        return match self.calc_section(internal_start, internal_end) {
            Some(x) => x,
            None => panic!(),
        };
    }

    // 1つの要素を更新する
    pub fn update(&mut self, index: usize, new_value: T) {
        let origin_start_index = self.segment_array.len() >> 1;

        let mut target_index = origin_start_index + index;
        self.segment_array[target_index] = Some(new_value);
        target_index >>= 1;
        while target_index != 0 {
            let new_segment_value = if let Some(left) = &self.segment_array[target_index << 1] {
                if let Some(right) = &self.segment_array[(target_index << 1) + 1] {
                    let operation = self.operation;
                    operation(&left, &right)
                } else {
                    left.clone()
                }
            } else {
                panic!()
            };

            self.segment_array[target_index] = Some(new_segment_value);

            target_index >>= 1;
        }
    }

    pub fn get(&self, index: usize) -> T {
        let origin_start_index = self.segment_array.len() >> 1;

        let mut target_index = origin_start_index + index;

        self.segment_array[target_index].clone().unwrap()
    }

    // 内部
    fn calc_section(&self, internal_start: usize, internal_end: usize) -> Option<T> {
        if internal_start == internal_end {
            return self.segment_array[internal_start].clone();
        } else {
            let section_and = internal_start & internal_end;
            let end_offset = section_and ^ internal_end;
            // CAUTION: 64bit only
            let xor_digits = 64 - end_offset.leading_zeros();

            // start,endがXXX000とXXX111のような形になっている
            if xor_digits == end_offset.count_ones() {
                // XXX000とXXX111ならXXX部分を取り出す
                let section_index = section_and >> xor_digits;
                return self.segment_array[section_index].clone();
            } else {
                // XXX000とXXX101なら、[XXX000,XXX011)と[XXX100,XXX101)をそれぞれ計算
                let left_end = section_and + (1 << (xor_digits - 1)) - 1;
                let right_start = left_end + 1;

                if let Some(left) = self.calc_section(internal_start, left_end) {
                    if let Some(right) = self.calc_section(right_start, internal_end) {
                        let operation = self.operation;
                        return Some(operation(&left, &right));
                    } else {
                        return Some(left);
                    }
                } else {
                    panic!()
                }
            }
        }
    }
}

#[snippet("@SegTree")]
impl<T: std::clone::Clone + std::fmt::Debug> std::fmt::Debug for SegTree<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let origin_start_index = self.segment_array.len() >> 1;
        let target_slice: Vec<_> = self.segment_array[origin_start_index..]
            .iter()
            .filter(|&element| {
                if element.is_some() {
                    return true;
                } else {
                    return false;
                }
            })
            .map(|element| element.as_ref().unwrap().clone())
            .collect();

        f.debug_list().entries(&target_slice).finish()
    }
}

mod test {
    use super::*;
    use rand::Rng;
    use std::cmp;
    use std::i64::MAX;
    #[test]
    fn seg_tree_test() {
        let mut cum_sum = SegTree::new(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], |&a, &b| a + b);
        assert_eq!(cum_sum.query(0, 4), 10);
        assert_eq!(cum_sum.query(2, 6), 18);
        assert_eq!(cum_sum.query(4, 10), 45);
        assert_eq!(cum_sum.query(3, 8), 30);
        assert_eq!(cum_sum.query(0, 9), 45);
        assert_eq!(cum_sum.get(0), 1);
        assert_eq!(cum_sum.get(4), 5);
        assert_eq!(cum_sum.get(7), 8);

        cum_sum.update(0, 10);
        assert_eq!(cum_sum.get(0), 10);
        assert_eq!(cum_sum.query(0, 4), 19);
        assert_eq!(cum_sum.query(2, 6), 18);
        assert_eq!(cum_sum.query(4, 10), 45);
        assert_eq!(cum_sum.query(3, 8), 30);
        assert_eq!(cum_sum.query(0, 9), 54);

        cum_sum.update(4, 55);
        assert_eq!(cum_sum.get(4), 55);
        assert_eq!(cum_sum.query(0, 4), 19);
        assert_eq!(cum_sum.query(2, 6), 68);
        assert_eq!(cum_sum.query(4, 10), 95);
        assert_eq!(cum_sum.query(3, 8), 80);
        assert_eq!(cum_sum.query(0, 9), 104);

        cum_sum.update(4, 5);
        assert_eq!(cum_sum.get(4), 5);
        assert_eq!(cum_sum.query(0, 4), 19);
        assert_eq!(cum_sum.query(2, 6), 18);
        assert_eq!(cum_sum.query(4, 10), 45);
        assert_eq!(cum_sum.query(3, 8), 30);
        assert_eq!(cum_sum.query(0, 9), 54);

        let mut max_value = SegTree::new(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], |&a, &b| {
            std::cmp::max(a, b)
        });
        assert_eq!(max_value.query(0, 4), 4);
        assert_eq!(max_value.query(2, 6), 6);
        assert_eq!(max_value.query(4, 10), 10);
        assert_eq!(max_value.query(3, 8), 8);
        assert_eq!(max_value.query(0, 9), 9);

        max_value.update(0, 10);
        assert_eq!(max_value.query(0, 4), 10);
        assert_eq!(max_value.query(2, 6), 6);
        assert_eq!(max_value.query(4, 10), 10);
        assert_eq!(max_value.query(3, 8), 8);
        assert_eq!(max_value.query(0, 9), 10);

        max_value.update(4, 55);
        assert_eq!(max_value.query(0, 4), 10);
        assert_eq!(max_value.query(2, 6), 55);
        assert_eq!(max_value.query(4, 10), 55);
        assert_eq!(max_value.query(3, 8), 55);
        assert_eq!(max_value.query(0, 9), 55);

        max_value.update(4, 5);
        assert_eq!(max_value.query(0, 4), 10);
        assert_eq!(max_value.query(2, 6), 6);
        assert_eq!(max_value.query(4, 10), 10);
        assert_eq!(max_value.query(3, 8), 8);
        assert_eq!(max_value.query(0, 9), 10);

        let mut min_value = SegTree::new(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], |&a, &b| {
            std::cmp::min(a, b)
        });
        assert_eq!(min_value.query(0, 4), 1);
        assert_eq!(min_value.query(2, 6), 3);
        assert_eq!(min_value.query(4, 10), 5);
        assert_eq!(min_value.query(3, 8), 4);
        assert_eq!(min_value.query(0, 9), 1);

        min_value.update(0, 10);
        assert_eq!(min_value.query(0, 4), 2);
        assert_eq!(min_value.query(2, 6), 3);
        assert_eq!(min_value.query(4, 10), 5);
        assert_eq!(min_value.query(3, 8), 4);
        assert_eq!(min_value.query(0, 9), 2);

        min_value.update(4, 55);
        assert_eq!(min_value.query(0, 4), 2);
        assert_eq!(min_value.query(2, 6), 3);
        assert_eq!(min_value.query(4, 10), 6);
        assert_eq!(min_value.query(3, 8), 4);
        assert_eq!(min_value.query(0, 9), 2);

        min_value.update(4, 5);
        assert_eq!(min_value.query(0, 4), 2);
        assert_eq!(min_value.query(2, 6), 3);
        assert_eq!(min_value.query(4, 10), 5);
        assert_eq!(min_value.query(3, 8), 4);
        assert_eq!(min_value.query(0, 9), 2);

        let desc_vec = |v1: &Vec<usize>, v2: &Vec<usize>| {
            let mut new_array: Vec<usize> = Vec::new();
            let mut v1_index = 0;
            let mut v2_index = 0;

            while v1_index < v1.len() || v2_index < v2.len() {
                let v1_element = if v1_index < v1.len() {
                    v1[v1_index] as i64
                } else {
                    -1
                };
                let v2_element = if v2_index < v2.len() {
                    v2[v2_index] as i64
                } else {
                    -1
                };

                let max = if v1_element >= v2_element {
                    v1_index += 1;
                    v1[v1_index - 1]
                } else {
                    v2_index += 1;
                    v2[v2_index - 1]
                };

                new_array.push(max)
            }

            new_array
        };

        let mut desc_array = SegTree::new(
            vec![
                vec![26],
                vec![8],
                vec![17],
                vec![19],
                vec![6],
                vec![38],
                vec![57],
                vec![43],
                vec![135],
                vec![0],
            ],
            desc_vec,
        );
        assert_eq!(desc_array.query(0, 4), vec![26, 19, 17, 8]);
        assert_eq!(desc_array.query(2, 6), vec![38, 19, 17, 6]);
        assert_eq!(desc_array.query(4, 10), vec![135, 57, 43, 38, 6, 0]);
        assert_eq!(desc_array.query(3, 8), vec![57, 43, 38, 19, 6]);
        assert_eq!(
            desc_array.query(0, 9),
            vec![135, 57, 43, 38, 26, 19, 17, 8, 6]
        );

        desc_array.update(0, vec![10]);
        assert_eq!(desc_array.query(0, 4), vec![19, 17, 10, 8]);
        assert_eq!(desc_array.query(2, 6), vec![38, 19, 17, 6]);
        assert_eq!(desc_array.query(4, 10), vec![135, 57, 43, 38, 6, 0]);
        assert_eq!(desc_array.query(3, 8), vec![57, 43, 38, 19, 6]);
        assert_eq!(
            desc_array.query(0, 9),
            vec![135, 57, 43, 38, 19, 17, 10, 8, 6]
        );

        desc_array.update(4, vec![138]);
        assert_eq!(desc_array.query(0, 4), vec![19, 17, 10, 8]);
        assert_eq!(desc_array.query(2, 6), vec![138, 38, 19, 17]);
        assert_eq!(desc_array.query(4, 10), vec![138, 135, 57, 43, 38, 0]);
        assert_eq!(desc_array.query(3, 8), vec![138, 57, 43, 38, 19,]);
        assert_eq!(
            desc_array.query(0, 9),
            vec![138, 135, 57, 43, 38, 19, 17, 10, 8]
        );

        desc_array.update(4, vec![6]);
        assert_eq!(desc_array.query(0, 4), vec![19, 17, 10, 8]);
        assert_eq!(desc_array.query(2, 6), vec![38, 19, 17, 6]);
        assert_eq!(desc_array.query(4, 10), vec![135, 57, 43, 38, 6, 0]);
        assert_eq!(desc_array.query(3, 8), vec![57, 43, 38, 19, 6]);
        assert_eq!(
            desc_array.query(0, 9),
            vec![135, 57, 43, 38, 19, 17, 10, 8, 6]
        );

        let combine_sorted_vec = |v1: &Vec<usize>, v2: &Vec<usize>| {
            let mut new_array: Vec<usize> = Vec::new();
            let mut v1_index = 0;
            let mut v2_index = 0;

            while v1_index < v1.len() || v2_index < v2.len() {
                let v1_element = if v1_index < v1.len() {
                    v1[v1_index]
                } else {
                    usize::MAX
                };
                let v2_element = if v2_index < v2.len() {
                    v2[v2_index]
                } else {
                    usize::MAX
                };

                let max = if v1_element <= v2_element {
                    v1_index += 1;
                    v1[v1_index - 1]
                } else {
                    v2_index += 1;
                    v2[v2_index - 1]
                };

                new_array.push(max)
            }

            new_array
        };

        let sorted_array = SegTree::new(
            vec![
                vec![26],
                vec![8],
                vec![17],
                vec![19],
                vec![6],
                vec![38],
                vec![57],
                vec![43],
                vec![135],
                vec![0],
            ],
            combine_sorted_vec,
        );

        assert_eq!(sorted_array.query(0, 4), vec![8, 17, 19, 26]);
        assert_eq!(sorted_array.query(2, 6), vec![6, 17, 19, 38]);
        assert_eq!(sorted_array.query(4, 10), vec![0, 6, 38, 43, 57, 135]);
        assert_eq!(sorted_array.query(3, 8), vec![6, 19, 38, 43, 57]);
        assert_eq!(
            sorted_array.query(0, 9),
            vec![6, 8, 17, 19, 26, 38, 43, 57, 135]
        );
    }

    #[test]
    fn random_array_min() {
        let n = 1000;
        let array = (0..n)
            .map(|_| {
                return rand::thread_rng().gen::<i64>();
            })
            .collect::<Vec<_>>();
        let sin_array = array.clone();

        let mut segTree = SegTree::new(array, |a, b| cmp::min(*a, *b));
        for i in 0..n {
            let mut min = MAX;
            for j in 0..(i + 1) {
                min = cmp::min(min, sin_array[j]);
            }
            assert_eq!(segTree.query(0, i + 1), min);
        }
    }

    #[test]
    fn random_len_sum() {
        let n = rand::thread_rng().gen::<u8>() as usize;
        let mut array = vec![0; n as usize];
        for i in 0..n {
            array[i as usize] = i as usize;
        }

        let mut segTree = SegTree::new(array, |a, b| a + b);
        for i in 1..n {
            for j in i + 2..n {
                let i = i as usize;
                let j = j as usize;
                let sum = j * (j - 1) / 2 - i * (i - 1) / 2;
                assert_eq!(segTree.query(i as usize, j as usize), sum);
            }
        }
    }
}
