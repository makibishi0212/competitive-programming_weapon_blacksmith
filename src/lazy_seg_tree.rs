use cargo_snippet::snippet;

#[snippet("@LazySegTree")]
pub struct LazySegTree<T: std::clone::Clone + std::fmt::Debug, U: std::clone::Clone> {
  // operation(a,b) = operation(b,a)、operation(a,operation(b,c)) = operation(operation(a,b),c)の必要がある
  // 区間に対するクエリの処理
  operation: fn(&T, &T) -> T,

  // 区間更新時に各要素に対して行う処理
  effector: fn(&T, &U) -> T,

  // 区間に対する作用が重なった時の解決処理
  resolve_effect: fn(&U, &U) -> U,
  segment_array: Vec<Option<T>>,
  lazy_array: Vec<Option<U>>,
  origin_length: usize,
}

#[snippet("@LazySegTree")]
impl<T: std::clone::Clone + std::fmt::Debug, U: std::clone::Clone> LazySegTree<T, U> {
  pub fn new(
    array: Vec<T>,
    operation: fn(&T, &T) -> T,
    effector: fn(&T, &U) -> T,
    resolve_effect: fn(&U, &U) -> U,
  ) -> LazySegTree<T, U> {
    let origin_length = array.len();
    let mut all_array_length = 1;
    while origin_length * 2 > all_array_length {
      all_array_length <<= 1;
    }

    let mut segment_array: Vec<Option<T>> = vec![None; all_array_length];
    let lazy_array: Vec<Option<U>> = vec![None; all_array_length];
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

    LazySegTree {
      operation,
      effector,
      resolve_effect,
      segment_array,
      lazy_array,
      origin_length: array.len(),
    }
  }

  // [start_index,end_index)でクエリに答える
  pub fn query(&mut self, start_index: usize, end_index: usize) -> T {
    if start_index >= end_index {
      panic!();
    }
    let origin_start_index = self.segment_array.len() >> 1;
    let internal_start = origin_start_index + start_index;
    let internal_end = origin_start_index + end_index - 1;

    if internal_end >= origin_start_index + self.origin_length {
      panic!();
    }
    return match self.resolve_effectcalc_section(internal_start, internal_end) {
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

  // 指定した範囲の要素にeffectorを適用させ、更新する
  pub fn range_effect(&mut self, start_index: usize, end_index: usize, effect: U) {
    if start_index >= end_index {
      panic!();
    }
    let origin_start_index = self.segment_array.len() >> 1;
    let internal_start = origin_start_index + start_index;
    let internal_end = origin_start_index + end_index - 1;

    if internal_end >= origin_start_index + self.origin_length {
      panic!();
    }
    self.resolve_effectapply_effector(internal_start, internal_end, &effect);
  }

  // 区間に対して作用を適用させる
  fn resolve_effectapply_effector(
    &mut self,
    internal_start: usize,
    internal_end: usize,
    effect: &U,
  ) -> () {
    if internal_start == internal_end {
      self.resolve_effectapply_effector_lazy(internal_start, effect);
    } else {
      let section_and = internal_start & internal_end;
      let end_offset = section_and ^ internal_end;
      // CAUTION: 64bit only
      let xor_digits = 64 - end_offset.leading_zeros();

      // start,endがXXX000とXXX111のような形になっている
      if xor_digits == end_offset.count_ones() {
        // XXX000とXXX111ならXXX部分を取り出す
        let section_index = section_and >> xor_digits;
        self.resolve_effectapply_effector_lazy(section_index, effect);
      } else {
        // XXX000とXXX101なら、(XXX000 - XXX011)と(XXX100 - XXX101)をそれぞれ計算
        let left_end = section_and + (1 << (xor_digits - 1)) - 1;
        let right_start = left_end + 1;

        self.resolve_effectapply_effector(internal_start, left_end, effect);
        self.resolve_effectapply_effector(right_start, internal_end, effect);
      }
    }
  }

  // 作用に作用を適用させる
  fn resolve_effectapply_effector_lazy(&mut self, lazy_index: usize, effect: &U) {
    if let Some(x) = &self.lazy_array[lazy_index] {
      let resolve_effect = self.resolve_effect;
      self.lazy_array[lazy_index] = Some(resolve_effect(x, effect));
    } else {
      self.lazy_array[lazy_index] = Some(effect.clone());
    }
  }

  // 区間の計算
  fn resolve_effectcalc_section(
    &mut self,
    internal_start: usize,
    internal_end: usize,
  ) -> Option<T> {
    if internal_start == internal_end {
      return self.resolve_effectget_data(internal_start);
    } else {
      let section_and = internal_start & internal_end;
      let end_offset = section_and ^ internal_end;
      // CAUTION: 64bit only
      let xor_digits = 64 - end_offset.leading_zeros();

      // start,endがXXX000とXXX111のような形になっている
      if xor_digits == end_offset.count_ones() {
        // XXX000とXXX111ならXXX部分を取り出す
        let section_index = section_and >> xor_digits;
        return self.resolve_effectget_data(section_index);
      } else {
        // XXX000とXXX101なら、(XXX000 - XXX011)と(XXX100 - XXX101)をそれぞれ計算
        let left_end = section_and + (1 << (xor_digits - 1)) - 1;
        let right_start = left_end + 1;

        let left = self.resolve_effectcalc_section(internal_start, left_end);
        let right = self.resolve_effectcalc_section(right_start, internal_end);

        self.operation(left, right)
      }
    }
  }

  // 各区間のデータにlazyの値を適用させて返す
  fn resolve_effectget_data(&mut self, internal_index: usize) -> Option<T> {
    if let Some(value) = &self.segment_array[internal_index] {
      let origin_start_index = self.segment_array.len() >> 1;
      if let Some(effect) = &self.lazy_array[internal_index] {
        if internal_index >= origin_start_index {
          // 葉の場合はeffectを適用し値を返す
          let effector = self.effector;
          self.segment_array[internal_index] = Some(effector(value, effect));
        } else {
          // 葉でない場合は子にeffectを伝播させる
          let left_index = internal_index << 1;
          let right_index = (internal_index << 1) + 1;

          let effect = effect.clone();
          self.resolve_effectapply_effector_lazy(left_index, &effect);
          self.resolve_effectapply_effector_lazy(right_index, &effect);
          let left = self.resolve_effectget_data(left_index);
          let right = self.resolve_effectget_data(right_index);
          self.segment_array[internal_index] = self.operation(left, right);
        }

        // 更新完了
        self.lazy_array[internal_index] = None;
      } else {
        if internal_index >= origin_start_index {
          // 葉の場合は何もしない
        } else {
          // 葉でない場合は値を再計算
          let left_index = internal_index << 1;
          let right_index = (internal_index << 1) + 1;
          let left = self.resolve_effectget_data(left_index);
          let right = self.resolve_effectget_data(right_index);
          self.segment_array[internal_index] = self.operation(left, right);
        }
      }
    }

    self.segment_array[internal_index].clone()
  }

  fn operation(&self, left: Option<T>, right: Option<T>) -> Option<T> {
    if let Some(left) = left {
      if let Some(right) = right {
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

#[snippet("@LazySegTree")]
impl<T: std::clone::Clone + std::fmt::Debug, U: std::clone::Clone> std::fmt::Debug
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

#[test]
fn lazy_seg_tree_test() {
  let mut cum_sum = LazySegTree::new(
    vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
    |&a, &b| a + b,
    |target, effect| target + effect,
    |effect1, effect2| effect1 + effect2,
  );
  assert_eq!(cum_sum.query(0, 4), 10);
  assert_eq!(cum_sum.query(2, 6), 18);
  assert_eq!(cum_sum.query(4, 10), 45);
  assert_eq!(cum_sum.query(3, 8), 30);
  assert_eq!(cum_sum.query(0, 9), 45);

  cum_sum.update(0, 10);
  assert_eq!(cum_sum.query(0, 4), 19);
  assert_eq!(cum_sum.query(2, 6), 18);
  assert_eq!(cum_sum.query(4, 10), 45);
  assert_eq!(cum_sum.query(3, 8), 30);
  assert_eq!(cum_sum.query(0, 9), 54);

  cum_sum.range_effect(2, 4, 10);
  assert_eq!(cum_sum.query(0, 4), 39);
  assert_eq!(cum_sum.query(2, 6), 38);
  assert_eq!(cum_sum.query(4, 10), 45);
  assert_eq!(cum_sum.query(3, 8), 40);
  assert_eq!(cum_sum.query(0, 9), 74);

  cum_sum.range_effect(4, 6, 100);
  assert_eq!(cum_sum.query(0, 4), 39);
  assert_eq!(cum_sum.query(2, 6), 238);
  assert_eq!(cum_sum.query(4, 10), 245);
  assert_eq!(cum_sum.query(3, 8), 240);
  assert_eq!(cum_sum.query(0, 9), 274);

  let mut max_value = LazySegTree::new(
    vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
    |&a, &b| std::cmp::max(a, b),
    |target, effect| target + effect,
    |effect1, effect2| effect1 + effect2,
  );
  assert_eq!(max_value.query(0, 4), 4);
  assert_eq!(max_value.query(2, 6), 6);
  assert_eq!(max_value.query(4, 10), 10);
  assert_eq!(max_value.query(3, 8), 8);
  assert_eq!(max_value.query(0, 9), 9);
}
