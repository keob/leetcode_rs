pub struct Solution;

impl Solution {
    pub fn num_squares(n: i32) -> i32 {
        let mut ans = n;
        let n = n as usize;
        let mut dp = vec![ans; n + 1];
        dp[0] = 0;

        for i in 1..=n {
            let x = i * i;
            for j in x..=n {
                dp[j] = dp[j].min(dp[j - x] + 1);
            }
            ans = ans.min(dp[n]);
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_1() {
        assert_eq!(Solution::num_squares(12), 3);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::num_squares(13), 2);
    }
}
