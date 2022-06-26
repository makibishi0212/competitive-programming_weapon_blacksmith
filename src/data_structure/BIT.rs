use cargo_snippet::snippet;

#[snippet("@BIT")]
pub struct BIT<
    T: std::ops::AddAssign + std::ops::Sub<Output = T> + std::marker::Copy + std::fmt::Debug,
> {
    internal_array: Vec<T>,
}

#[snippet("@BIT")]
impl<T: std::ops::AddAssign + std::ops::Sub<Output = T> + std::marker::Copy + std::fmt::Debug>
    BIT<T>
{
    pub fn new(array: Vec<T>) -> BIT<T> {
        let mut internal_array: Vec<T> = Vec::with_capacity(array.len());

        if array.len() == 0 {
            return BIT { internal_array };
        }

        let mut cum_sum: Vec<T> = Vec::with_capacity(array.len());

        internal_array.push(array[0]);
        cum_sum.push(array[0]);
        let mut sum = array[0];
        for i in 1..array.len() {
            let mut lsb = 0;
            let mut tmp_i = i + 1;
            let mut now_digit = 1;
            while lsb == 0 {
                if tmp_i & 1 == 1 {
                    lsb = now_digit;
                } else {
                    tmp_i >>= 1;
                    now_digit += 1;
                }
            }

            sum += array[i];
            if lsb == 1 {
                internal_array.push(array[i]);
            } else {
                if (1 << lsb) < i {
                    internal_array.push(sum - cum_sum[i - (1 << (lsb - 1))]);
                } else {
                    internal_array.push(sum);
                }
            }
            cum_sum.push(sum);
        }

        BIT { internal_array }
    }

    pub fn add(&mut self, mut index: usize, new_value: T) {
        index += 1;
        while index <= self.internal_array.len() {
            self.internal_array[index - 1] += new_value;
            index += index & index.wrapping_neg();
        }
    }

    pub fn query(&self, start_index: usize, end_index: usize) -> T {
        if start_index == 0 {
            self.calc_sum(end_index)
        } else {
            self.calc_sum(end_index) - self.calc_sum(start_index)
        }
    }

    fn calc_sum(&self, mut index: usize) -> T {
        let mut sum = self.internal_array[index - 1];
        index &= index - 1;

        while index > 0 {
            sum += self.internal_array[index - 1];
            index &= index - 1;
        }

        sum
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn bit_test() {
        let mut bit = BIT::new(vec![0, 1, 0, 1, 0, 1]);
        assert_eq!(bit.query(0, 6), 3);
        bit.add(0, 1);
        assert_eq!(bit.query(0, 6), 4);
        assert_eq!(bit.query(0, 1), 1);
        assert_eq!(bit.query(1, 6), 3);
        bit.add(2, 10);
        assert_eq!(bit.query(0, 6), 14);
        assert_eq!(bit.query(2, 3), 10);
        assert_eq!(bit.query(3, 6), 2);
        bit.add(4, 100);
        assert_eq!(bit.query(0, 6), 114);
        assert_eq!(bit.query(4, 5), 100);
        assert_eq!(bit.query(5, 6), 1);

        let mut nums = Vec::with_capacity(100);
        let mut cum_sum = vec![0; 101];
        for i in 1..100 {
            cum_sum[i] = cum_sum[i - 1] + i;
            nums.push(i);
        }

        let mut bit2 = BIT::new(nums);

        for i in 1..100 {
            for j in i + 1..100 {
                assert_eq!(bit2.query(i, j), cum_sum[j] - cum_sum[i]);
            }
        }
    }
}
