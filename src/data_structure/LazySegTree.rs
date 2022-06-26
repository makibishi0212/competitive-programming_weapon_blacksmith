use cargo_snippet::snippet;

#[snippet("@LazySegTree")]
pub struct LazySegTree<
    T: std::clone::Clone + std::fmt::Debug,
    U: std::clone::Clone + std::fmt::Debug,
> {
    // operation,effector,resolve_effectはそれぞれ、f(a,b) = f(b,a)、f(a,f(b,c)) = f(f(a,b),c) を満たす必要がある
    // operation、effectorは,
    // effector(operation(a,b),c) = operation(effector(a,c),effector(b,c))を満たす必要がある
    // 区間に対するクエリの処理
    operation: fn(T, T) -> T,

    // 区間更新時に各要素に対して行う処理
    effector: fn(T, U) -> T,

    // 区間に対する作用が重なった時の解決処理
    resolve_effect: fn(U, U) -> U,
    segment_array: Vec<Option<T>>,
    lazy_array: Vec<Option<U>>,
    origin_length: usize,
    origin_power: usize,
    log: usize,
}

#[snippet("@LazySegTree")]
impl<T: std::clone::Clone + std::fmt::Debug, U: std::clone::Clone + std::fmt::Debug>
    LazySegTree<T, U>
{
    pub fn new(
        array: Vec<T>,
        operation: fn(T, T) -> T,
        effector: fn(T, U) -> T,
        resolve_effect: fn(U, U) -> U,
    ) -> LazySegTree<T, U> {
        let origin_length = array.len();
        let mut power = 1;
        let mut log = 0;
        while origin_length > power {
            power <<= 1;
            log += 1;
        }

        let mut segment_array: Vec<Option<T>> = vec![None; 2 * power];
        let lazy_array: Vec<Option<U>> = vec![None; power];

        for i in power..(power + origin_length) {
            segment_array[i] = Some(array[i - power].clone());
        }

        let mut lazy_seg_tree = LazySegTree {
            operation,
            effector,
            resolve_effect,
            segment_array,
            lazy_array,
            origin_length,
            origin_power: power,
            log,
        };

        for i in (1..power).rev() {
            lazy_seg_tree.update(i);
        }

        lazy_seg_tree
    }

    // [left,right)でクエリに答える
    pub fn query(&mut self, mut left_index: usize, mut right_index: usize) -> T {
        assert!(left_index < right_index && right_index <= self.origin_power);
        left_index += self.origin_power;
        right_index += self.origin_power;

        for i in (1..=self.log).rev() {
            if ((left_index >> i) << i) != left_index {
                self.push(left_index >> i);
            }
            if ((right_index >> i) << i) != right_index {
                self.push(right_index >> i);
            }
        }

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
        for i in (1..=self.log).rev() {
            self.push(index >> i);
        }
        self.segment_array[index].clone().unwrap()
    }

    pub fn set(&mut self, mut index: usize, new_value: T) {
        assert!(index < self.origin_length);
        index += self.origin_power;
        for i in (1..=self.log).rev() {
            self.push(index >> i);
        }
        self.segment_array[index] = Some(new_value);
        for i in 1..=self.log {
            self.update(index >> i);
        }
    }

    fn push(&mut self, index: usize) {
        //println!("lazy [{:?}]={:?}", index, self.lazy_array[index]);
        if self.lazy_array[index].is_some() {
            let effect = self.lazy_array[index].clone().unwrap();
            self.all_apply(2 * index, effect.clone());
            self.all_apply(2 * index + 1, effect);
        }
        self.lazy_array[index] = None;
    }

    fn all_apply(&mut self, seg_index: usize, effect: U) {
        if self.segment_array[seg_index].is_some() {
            let value = self.segment_array[seg_index].clone().unwrap();
            let effector = self.effector;
            self.segment_array[seg_index] = Some(effector(value, effect.clone()));
        }
        if seg_index < self.origin_power {
            if self.lazy_array[seg_index].is_some() {
                let origin_effect = self.lazy_array[seg_index].clone().unwrap();
                let resolve_effect = self.resolve_effect;
                self.lazy_array[seg_index] = Some(resolve_effect(origin_effect, effect.clone()));
            } else {
                self.lazy_array[seg_index] = Some(effect.clone());
            }
        }
    }

    pub fn apply(&mut self, mut index: usize, effect: U) {
        assert!(index < self.origin_length);
        index += self.origin_power;

        for i in (1..=self.log).rev() {
            self.push(index >> i);
        }
        //self.segment_array[index] = F::mapping(&f, &self.d[p]);
        if self.segment_array[index].is_some() {
            let effector = self.effector;
            self.segment_array[index] =
                Some(effector(self.segment_array[index].clone().unwrap(), effect));
        }
        for i in 1..=self.log {
            self.update(index >> i);
        }
    }

    // 指定した範囲全てに対してeffectorを適用する
    pub fn apply_range(&mut self, mut left_index: usize, mut right_index: usize, effect: U) {
        assert!(left_index < right_index && right_index <= self.origin_power);

        left_index += self.origin_power;
        right_index += self.origin_power;

        for i in (1..=self.log).rev() {
            if ((left_index >> i) << i) != left_index {
                self.push(left_index >> i);
            }
            if ((right_index >> i) << i) != right_index {
                self.push((right_index - 1) >> i);
            }
        }

        {
            let left_index_tmp = left_index;
            let right_index_tmp = right_index;
            while left_index < right_index {
                if left_index & 1 != 0 {
                    self.all_apply(left_index, effect.clone());
                    left_index += 1;
                }
                if right_index & 1 != 0 {
                    right_index -= 1;
                    self.all_apply(right_index, effect.clone());
                }
                left_index >>= 1;
                right_index >>= 1;
            }

            left_index = left_index_tmp;
            right_index = right_index_tmp;
        }

        for i in 1..=self.log {
            if ((left_index >> i) << i) != left_index {
                self.update(left_index >> i);
            }
            if ((right_index >> i) << i) != right_index {
                self.update((right_index - 1) >> i);
            }
        }
    }
}

#[snippet("@LazySegTree")]
impl<T: std::clone::Clone + std::fmt::Debug, U: std::clone::Clone + std::fmt::Debug> std::fmt::Debug
    for LazySegTree<T, U>
{
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
mod tests {
    use super::*;

    #[test]
    fn lazy_seg_tree_test() {
        let mut cum_sum = LazySegTree::<usize, usize>::new(
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
            |a, b| std::cmp::max(a, b),
            |target, effect| target + effect,
            |effect1, effect2| effect1 + effect2,
        );

        assert_eq!(cum_sum.origin_power, 16);
        assert_eq!(cum_sum.origin_length, 10);
        assert_eq!(cum_sum.log, 4);
        assert_eq!(cum_sum.get(2), 3);
        assert_eq!(cum_sum.get(0), 1);
        assert_eq!(cum_sum.get(7), 8);
        assert_eq!(cum_sum.get(9), 10);
        assert_eq!(cum_sum.query(0, 5), 5);
        cum_sum.set(0, 20);
        assert_eq!(cum_sum.get(0), 20);
        assert_eq!(cum_sum.query(0, 4), 20);
        cum_sum.apply(0, 10);
        assert_eq!(cum_sum.get(0), 30);
        assert_eq!(cum_sum.query(0, 4), 30);

        cum_sum.apply_range(1, 3, 100);
        assert_eq!(cum_sum.query(0, 4), 103);
        assert_eq!(cum_sum.get(1), 102);

        assert_eq!(cum_sum.query(5, 10), 10);
        cum_sum.apply_range(5, 10, 5);
        assert_eq!(cum_sum.query(5, 10), 15);
    }
}
