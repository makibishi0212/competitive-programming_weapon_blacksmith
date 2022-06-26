use cargo_snippet::snippet;

#[snippet("@SegTree")]
pub struct SegTree<T: std::clone::Clone + std::fmt::Debug> {
    // operationはf(a,b) = f(b,a)、f(a,f(b,c)) = f(f(a,b),c) を満たす必要がある
    // 区間に対するクエリの処理
    operation: fn(T, T) -> T,
    segment_array: Vec<Option<T>>,
    origin_length: usize,
    origin_power: usize,
    log: usize,
}

#[snippet("@SegTree")]
impl<T: std::clone::Clone + std::fmt::Debug> SegTree<T> {
    pub fn new(array: Vec<T>, operation: fn(T, T) -> T) -> SegTree<T> {
        let origin_length = array.len();
        let mut power = 1;
        let mut log = 0;
        while origin_length > power {
            power <<= 1;
            log += 1;
        }

        let mut segment_array: Vec<Option<T>> = vec![None; 2 * power];

        for i in power..(power + origin_length) {
            segment_array[i] = Some(array[i - power].clone());
        }

        let mut seg_tree = SegTree {
            operation,
            segment_array,
            origin_length,
            origin_power: power,
            log,
        };

        for i in (1..power).rev() {
            seg_tree.update(i);
        }

        seg_tree
    }

    // [left,right)でクエリに答える
    pub fn query(&mut self, mut left_index: usize, mut right_index: usize) -> T {
        assert!(left_index < right_index && right_index <= self.origin_power);
        left_index += self.origin_power;
        right_index += self.origin_power;

        let operation = self.operation;

        let mut left_value = None;
        let mut right_value = None;

        while left_index < right_index {
            if left_index & 1 != 0 {
                left_value = if left_value.is_some() && self.segment_array[left_index].is_some() {
                    let operation = self.operation;

                    Some(operation(
                        left_value.unwrap(),
                        self.segment_array[left_index].clone().unwrap(),
                    ))
                } else if self.segment_array[left_index].is_some() {
                    Some(self.segment_array[left_index].clone().unwrap())
                } else {
                    left_value
                };

                left_index += 1;
            }
            if right_index & 1 != 0 {
                right_index -= 1;
                right_value = if right_value.is_some() && self.segment_array[right_index].is_some()
                {
                    Some(operation(
                        self.segment_array[right_index].clone().unwrap(),
                        right_value.unwrap(),
                    ))
                } else if self.segment_array[right_index].is_some() {
                    Some(self.segment_array[right_index].clone().unwrap())
                } else {
                    right_value
                };
            }
            left_index >>= 1;
            right_index >>= 1;
        }

        if left_value.is_some() && right_value.is_some() {
            operation(left_value.clone().unwrap(), right_value.clone().unwrap())
        } else if left_value.is_some() {
            left_value.clone().unwrap()
        } else if right_value.is_some() {
            right_value.clone().unwrap()
        } else {
            panic!()
        }
    }

    fn update(&mut self, index: usize) {
        let operation = self.operation;
        let a = self.segment_array[2 * index].clone();
        let b = self.segment_array[2 * index + 1].clone();
        let value = if a.is_some() && b.is_some() {
            Some(operation(a.unwrap(), b.unwrap()))
        } else if a.is_some() {
            Some(a.unwrap())
        } else if b.is_some() {
            Some(b.unwrap())
        } else {
            None
        };

        self.segment_array[index] = value;
    }

    pub fn get(&mut self, mut index: usize) -> T {
        assert!(index < self.origin_length);
        index += self.origin_power;
        self.segment_array[index].clone().unwrap()
    }

    pub fn set(&mut self, mut index: usize, new_value: T) {
        assert!(index < self.origin_length);
        index += self.origin_power;
        self.segment_array[index] = Some(new_value);
        for i in 1..=self.log {
            self.update(index >> i);
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

#[cfg(test)]
mod test {
    use super::*;
    use rand::Rng;
    use std::cmp;
    use std::i64::MAX;
    #[test]
    fn seg_tree_test() {
        let mut cum_sum = SegTree::new(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], |a, b| a + b);
        assert_eq!(cum_sum.query(0, 4), 10);
        assert_eq!(cum_sum.query(2, 6), 18);
        assert_eq!(cum_sum.query(4, 10), 45);
        assert_eq!(cum_sum.query(3, 8), 30);
        assert_eq!(cum_sum.query(0, 9), 45);
        assert_eq!(cum_sum.get(0), 1);
        assert_eq!(cum_sum.get(4), 5);
        assert_eq!(cum_sum.get(7), 8);

        cum_sum.set(0, 10);
        assert_eq!(cum_sum.get(0), 10);
        assert_eq!(cum_sum.query(0, 4), 19);
        assert_eq!(cum_sum.query(2, 6), 18);
        assert_eq!(cum_sum.query(4, 10), 45);
        assert_eq!(cum_sum.query(3, 8), 30);
        assert_eq!(cum_sum.query(0, 9), 54);

        cum_sum.set(4, 55);
        assert_eq!(cum_sum.get(4), 55);
        assert_eq!(cum_sum.query(0, 4), 19);
        assert_eq!(cum_sum.query(2, 6), 68);
        assert_eq!(cum_sum.query(4, 10), 95);
        assert_eq!(cum_sum.query(3, 8), 80);
        assert_eq!(cum_sum.query(0, 9), 104);

        cum_sum.set(4, 5);
        assert_eq!(cum_sum.get(4), 5);
        assert_eq!(cum_sum.query(0, 4), 19);
        assert_eq!(cum_sum.query(2, 6), 18);
        assert_eq!(cum_sum.query(4, 10), 45);
        assert_eq!(cum_sum.query(3, 8), 30);
        assert_eq!(cum_sum.query(0, 9), 54);

        let mut max_value = SegTree::new(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], |a, b| {
            std::cmp::max(a, b)
        });
        assert_eq!(max_value.query(0, 4), 4);
        assert_eq!(max_value.query(2, 6), 6);
        assert_eq!(max_value.query(4, 10), 10);
        assert_eq!(max_value.query(3, 8), 8);
        assert_eq!(max_value.query(0, 9), 9);

        max_value.set(0, 10);
        assert_eq!(max_value.query(0, 4), 10);
        assert_eq!(max_value.query(2, 6), 6);
        assert_eq!(max_value.query(4, 10), 10);
        assert_eq!(max_value.query(3, 8), 8);
        assert_eq!(max_value.query(0, 9), 10);

        max_value.set(4, 55);
        assert_eq!(max_value.query(0, 4), 10);
        assert_eq!(max_value.query(2, 6), 55);
        assert_eq!(max_value.query(4, 10), 55);
        assert_eq!(max_value.query(3, 8), 55);
        assert_eq!(max_value.query(0, 9), 55);

        max_value.set(4, 5);
        assert_eq!(max_value.query(0, 4), 10);
        assert_eq!(max_value.query(2, 6), 6);
        assert_eq!(max_value.query(4, 10), 10);
        assert_eq!(max_value.query(3, 8), 8);
        assert_eq!(max_value.query(0, 9), 10);

        let mut min_value = SegTree::new(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], |a, b| {
            std::cmp::min(a, b)
        });
        assert_eq!(min_value.query(0, 4), 1);
        assert_eq!(min_value.query(2, 6), 3);
        assert_eq!(min_value.query(4, 10), 5);
        assert_eq!(min_value.query(3, 8), 4);
        assert_eq!(min_value.query(0, 9), 1);

        min_value.set(0, 10);
        assert_eq!(min_value.query(0, 4), 2);
        assert_eq!(min_value.query(2, 6), 3);
        assert_eq!(min_value.query(4, 10), 5);
        assert_eq!(min_value.query(3, 8), 4);
        assert_eq!(min_value.query(0, 9), 2);

        min_value.set(4, 55);
        assert_eq!(min_value.query(0, 4), 2);
        assert_eq!(min_value.query(2, 6), 3);
        assert_eq!(min_value.query(4, 10), 6);
        assert_eq!(min_value.query(3, 8), 4);
        assert_eq!(min_value.query(0, 9), 2);

        min_value.set(4, 5);
        assert_eq!(min_value.query(0, 4), 2);
        assert_eq!(min_value.query(2, 6), 3);
        assert_eq!(min_value.query(4, 10), 5);
        assert_eq!(min_value.query(3, 8), 4);
        assert_eq!(min_value.query(0, 9), 2);

        let desc_vec = |v1: Vec<usize>, v2: Vec<usize>| {
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

        desc_array.set(0, vec![10]);
        assert_eq!(desc_array.query(0, 4), vec![19, 17, 10, 8]);
        assert_eq!(desc_array.query(2, 6), vec![38, 19, 17, 6]);
        assert_eq!(desc_array.query(4, 10), vec![135, 57, 43, 38, 6, 0]);
        assert_eq!(desc_array.query(3, 8), vec![57, 43, 38, 19, 6]);
        assert_eq!(
            desc_array.query(0, 9),
            vec![135, 57, 43, 38, 19, 17, 10, 8, 6]
        );

        desc_array.set(4, vec![138]);
        assert_eq!(desc_array.query(0, 4), vec![19, 17, 10, 8]);
        assert_eq!(desc_array.query(2, 6), vec![138, 38, 19, 17]);
        assert_eq!(desc_array.query(4, 10), vec![138, 135, 57, 43, 38, 0]);
        assert_eq!(desc_array.query(3, 8), vec![138, 57, 43, 38, 19,]);
        assert_eq!(
            desc_array.query(0, 9),
            vec![138, 135, 57, 43, 38, 19, 17, 10, 8]
        );

        desc_array.set(4, vec![6]);
        assert_eq!(desc_array.query(0, 4), vec![19, 17, 10, 8]);
        assert_eq!(desc_array.query(2, 6), vec![38, 19, 17, 6]);
        assert_eq!(desc_array.query(4, 10), vec![135, 57, 43, 38, 6, 0]);
        assert_eq!(desc_array.query(3, 8), vec![57, 43, 38, 19, 6]);
        assert_eq!(
            desc_array.query(0, 9),
            vec![135, 57, 43, 38, 19, 17, 10, 8, 6]
        );

        let combine_sorted_vec = |v1: Vec<usize>, v2: Vec<usize>| {
            let mut new_array: Vec<usize> = Vec::new();
            let mut v1_index = 0;
            let mut v2_index = 0;

            while v1_index < v1.len() || v2_index < v2.len() {
                let v1_element = if v1_index < v1.len() {
                    v1[v1_index]
                } else {
                    std::usize::MAX
                };
                let v2_element = if v2_index < v2.len() {
                    v2[v2_index]
                } else {
                    std::usize::MAX
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

        let mut sorted_array = SegTree::new(
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

        let mut segTree = SegTree::new(array, |a, b| cmp::min(a, b));
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
