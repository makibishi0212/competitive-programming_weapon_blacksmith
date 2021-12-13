use cargo_snippet::snippet;

// double_ended_priority_queue
#[snippet("@DEPQ")]
pub struct DEPQ<T: Ord> {
    min_queue: std::collections::BinaryHeap<std::cmp::Reverse<T>>,
    max_queue: std::collections::BinaryHeap<T>,
    min_queue_delete: std::collections::BinaryHeap<std::cmp::Reverse<T>>,
    max_queue_delete: std::collections::BinaryHeap<T>,
}

#[snippet("@DEPQ")]
impl<T: Ord + Copy> DEPQ<T> {
    pub fn new() -> DEPQ<T> {
        return DEPQ {
            min_queue: std::collections::BinaryHeap::new(),
            max_queue: std::collections::BinaryHeap::new(),
            min_queue_delete: std::collections::BinaryHeap::new(),
            max_queue_delete: std::collections::BinaryHeap::new(),
        };
    }

    pub fn push(&mut self, value: T) {
        self.min_queue.push(std::cmp::Reverse(value));
        self.max_queue.push(value);
    }

    fn internal_peek_min(&mut self) -> Option<&std::cmp::Reverse<T>> {
        while self.min_queue.peek() == self.min_queue_delete.peek()
            && self.min_queue.peek().is_some()
        {
            self.min_queue.pop();
            self.min_queue_delete.pop();
        }

        self.min_queue.peek()
    }

    pub fn peek_max(&mut self) -> Option<&T> {
        while self.max_queue.peek() == self.max_queue_delete.peek()
            && self.max_queue.peek().is_some()
        {
            self.max_queue.pop();
            self.max_queue_delete.pop();
        }

        self.max_queue.peek()
    }

    pub fn peek_min(&mut self) -> Option<&T> {
        let peek = self.internal_peek_min();

        let peek = match peek {
            None => None,
            Some(rev_value) => {
                let std::cmp::Reverse(value) = rev_value;
                Some(value)
            }
        };

        peek
    }

    pub fn pop_min(&mut self) -> Option<T> {
        let min_top = self.internal_peek_min();
        if min_top.is_none() {
            return None;
        }

        let std::cmp::Reverse(min_top) = *min_top.unwrap();

        self.max_queue_delete.push(min_top);
        self.min_queue.pop();

        Some(min_top)
    }

    pub fn pop_max(&mut self) -> Option<T> {
        let max_top = self.peek_max();
        if max_top.is_none() {
            return None;
        }

        let max_top = *max_top.unwrap();

        self.min_queue_delete.push(std::cmp::Reverse(max_top));
        self.max_queue.pop();

        Some(max_top)
    }
}

mod test {
    use super::*;
    use proptest::prelude::*;

    #[test]
    fn depq_test() {
        let mut depq = DEPQ::new();
        depq.push(1);
        depq.push(12);
        depq.push(5);
        depq.push(4);
        depq.push(8);

        assert_eq!(depq.peek_min(), Some(&1));
        assert_eq!(depq.peek_max(), Some(&12));
        assert_eq!(depq.pop_min(), Some(1));

        assert_eq!(depq.peek_min(), Some(&4));
        assert_eq!(depq.peek_max(), Some(&12));
        assert_eq!(depq.pop_min(), Some(4));

        assert_eq!(depq.peek_min(), Some(&5));
        assert_eq!(depq.peek_max(), Some(&12));
        assert_eq!(depq.pop_max(), Some(12));

        assert_eq!(depq.peek_min(), Some(&5));
        assert_eq!(depq.peek_max(), Some(&8));
        assert_eq!(depq.pop_max(), Some(8));

        depq.push(3);
        depq.push(4);

        assert_eq!(depq.peek_min(), Some(&3));
        assert_eq!(depq.peek_max(), Some(&5));
        assert_eq!(depq.pop_min(), Some(3));

        assert_eq!(depq.peek_min(), Some(&4));
        assert_eq!(depq.peek_max(), Some(&5));
        assert_eq!(depq.pop_max(), Some(5));

        depq.push(5);
        depq.push(6);
        depq.push(7);
        depq.push(8);

        assert_eq!(depq.peek_min(), Some(&4));
        assert_eq!(depq.peek_max(), Some(&8));
        assert_eq!(depq.pop_min(), Some(4));

        assert_eq!(depq.peek_min(), Some(&5));
        assert_eq!(depq.peek_max(), Some(&8));
        assert_eq!(depq.pop_min(), Some(5));

        assert_eq!(depq.peek_min(), Some(&6));
        assert_eq!(depq.peek_max(), Some(&8));
        assert_eq!(depq.pop_min(), Some(6));

        assert_eq!(depq.peek_min(), Some(&7));
        assert_eq!(depq.peek_max(), Some(&8));
        assert_eq!(depq.pop_max(), Some(8));

        depq.push(1000);
        depq.push(0);

        assert_eq!(depq.peek_min(), Some(&0));
        assert_eq!(depq.peek_max(), Some(&1000));
        assert_eq!(depq.pop_max(), Some(1000));

        assert_eq!(depq.peek_min(), Some(&0));
        assert_eq!(depq.peek_max(), Some(&7));
        assert_eq!(depq.pop_max(), Some(7));

        assert_eq!(depq.peek_min(), Some(&0));
        assert_eq!(depq.peek_max(), Some(&0));
        assert_eq!(depq.pop_max(), Some(0));

        assert_eq!(depq.peek_min(), None);
        assert_eq!(depq.peek_max(), None);
        assert_eq!(depq.pop_max(), None);
    }

    proptest! {
      #[test]
      fn depq_test_random(a :u8) {
        let a = a as usize;
        let mut depq = DEPQ::new();

        for i in 0..a {
          depq.push(i);
        }

        for i in 0..a/2 {
          let min = depq.pop_min().unwrap();
          prop_assert!(min == i);
        }

        for i in 0..a/2 {
          let max = depq.pop_max().unwrap();
          prop_assert!(max == a-1-i);
        }
      }
    }
}
