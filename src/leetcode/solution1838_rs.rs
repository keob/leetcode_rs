pub struct Solution;

impl Solution {
    pub fn max_frequency(nums: Vec<i32>, k: i32) -> i32 {
        let (mut total, mut l, mut res) = (0, 0, 1);
        let mut nums = nums;
        let n = nums.len();

        nums.sort_unstable();

        for r in 1..n {
            let t = (r - l) as i32;
            total += t * (nums[r] - nums[r - 1]);
            while total > k {
                total -= nums[r] - nums[l];
                l += 1;
            }
            let t = (r - l) as i32;
            res = std::cmp::max(res, t + 1);
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_1() {
        let nums: Vec<i32> = vec![1, 2, 4];
        assert_eq!(Solution::max_frequency(nums, 5), 3);
    }

    #[test]
    fn test_2() {
        let nums: Vec<i32> = vec![1, 4, 8, 13];
        assert_eq!(Solution::max_frequency(nums, 5), 2);
    }

    #[test]
    fn test_3() {
        let nums: Vec<i32> = vec![3, 9, 6];
        assert_eq!(Solution::max_frequency(nums, 2), 1);
    }
}
