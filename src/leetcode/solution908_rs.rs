pub struct Solution;

impl Solution {
    pub fn smallest_range_i(nums: Vec<i32>, k: i32) -> i32 {
        (nums.iter().max().unwrap() - nums.iter().min().unwrap() - 2 * k).max(0)
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_1() {
        let nums: Vec<i32> = vec![1];
        assert_eq!(Solution::smallest_range_i(nums, 0), 0);
    }

    #[test]
    fn test_2() {
        let nums: Vec<i32> = vec![0, 10];
        assert_eq!(Solution::smallest_range_i(nums, 2), 6);
    }

    #[test]
    fn test_3() {
        let nums: Vec<i32> = vec![1, 3, 6];
        assert_eq!(Solution::smallest_range_i(nums, 3), 0);
    }
}
