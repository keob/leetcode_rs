pub struct Solution;

impl Solution {
    pub fn max_uncrossed_lines(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let m = nums1.len();
        let n = nums2.len();
        let mut dp = vec![vec![0; n + 1]; m + 1];

        for (i, _) in nums1.iter().enumerate().take(m) {
            for (j, _) in nums2.iter().enumerate().take(n) {
                if nums1[i] == nums2[j] {
                    dp[i + 1][j + 1] = dp[i][j] + 1;
                } else {
                    dp[i + 1][j + 1] = dp[i + 1][j].max(dp[i][j + 1]);
                }
            }
        }

        dp[m][n]
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_1() {
        let nums1: Vec<i32> = vec![1, 4, 2];
        let nums2: Vec<i32> = vec![1, 2, 4];

        assert_eq!(Solution::max_uncrossed_lines(nums1, nums2), 2);
    }

    #[test]
    fn test_2() {
        let nums1: Vec<i32> = vec![2, 5, 1, 2, 5];
        let nums2: Vec<i32> = vec![10, 5, 2, 1, 5, 2];

        assert_eq!(Solution::max_uncrossed_lines(nums1, nums2), 3);
    }

    #[test]
    fn test_3() {
        let nums1: Vec<i32> = vec![1, 3, 7, 1, 7, 5];
        let nums2: Vec<i32> = vec![1, 9, 2, 5, 1];

        assert_eq!(Solution::max_uncrossed_lines(nums1, nums2), 2);
    }
}
