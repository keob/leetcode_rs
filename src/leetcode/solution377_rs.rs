pub struct Solution;

impl Solution {
    pub fn combination_sum4(nums: Vec<i32>, target: i32) -> i32 {
        let mut dp: Vec<i32> = vec![0; target as usize + 1];
        dp[0] = 1;

        for i in 1..=target as usize {
            for &j in nums.iter() {
                if i as i32 >= j {
                    dp[i] += dp[i - j as usize];
                }
            }
        }

        dp[target as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_1() {
        let nums: Vec<i32> = vec![1, 2, 3];
        assert_eq!(Solution::combination_sum4(nums, 4), 7);
    }

    #[test]
    fn test_2() {
        let nums: Vec<i32> = vec![9];
        assert_eq!(Solution::combination_sum4(nums, 3), 0);
    }
}
