pub struct Solution;

impl Solution {
    pub fn count_bits(num: i32) -> Vec<i32> {
        let mut res = vec![0i32; (num + 1) as usize];

        for i in 1..=num as usize {
            res[i] = res[i & (i - 1)] + 1;
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_1() {
        assert_eq!(Solution::count_bits(2), vec![0, 1, 1]);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::count_bits(5), vec![0, 1, 1, 2, 1, 2]);
    }
}
