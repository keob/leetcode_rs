pub struct Solution;

impl Solution {
    pub fn maximum_difference(nums: Vec<i32>) -> i32 {
        let mut res = -1;
        let mut min = nums[0];
        for &n in nums.iter().skip(1) {
            if n > min {
                res = res.max(n - min);
            } else {
                min = n;
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_1() {
        let nums: Vec<i32> = vec![7, 1, 5, 4];
        assert_eq!(Solution::maximum_difference(nums), 4);
    }

    #[test]
    fn test_2() {
        let nums: Vec<i32> = vec![9, 4, 3, 2];
        assert_eq!(Solution::maximum_difference(nums), -1);
    }

    #[test]
    fn test_3() {
        let nums: Vec<i32> = vec![1, 5, 2, 10];
        assert_eq!(Solution::maximum_difference(nums), 9);
    }
}
