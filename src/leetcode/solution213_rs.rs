use std::cmp::max;

pub struct Solution;

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let n = nums.len();

        if n == 0 {
            return 0;
        } else if n == 1 {
            return nums[0];
        } else {
            max(Self::helper(&nums[0..n - 1]), Self::helper(&nums[1..n]))
        }
    }

    fn helper(nums: &[i32]) -> i32 {
        let mut first = 0;
        let mut second = 0;

        for i in 0..nums.len() {
            if i == 0 {
                first = nums[0]
            } else if i == 1 {
                second = max(nums[0], nums[1])
            } else {
                let tmp = second;
                second = max(first + nums[i], second);
                first = tmp;
            }
        }

        max(first, second)
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_1() {
        let nums: Vec<i32> = vec![1, 2];
        assert_eq!(Solution::rob(nums), 2);
    }

    #[test]
    fn test_2() {
        let nums: Vec<i32> = vec![2, 3, 2];
        assert_eq!(Solution::rob(nums), 3);
    }

    #[test]
    fn test_3() {
        let nums: Vec<i32> = vec![1, 2, 3, 1];
        assert_eq!(Solution::rob(nums), 4);
    }

    #[test]
    fn test_4() {
        let nums: Vec<i32> = vec![0];
        assert_eq!(Solution::rob(nums), 0);
    }
}
