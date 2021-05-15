pub struct Solution;

impl Solution {
    pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
        let n = text1.len();
        let m = text2.len();
        let mut dp = vec![vec![0; m + 1]; n + 1];

        for i in 0..n {
            for j in 0..m {
                if text1.as_bytes()[i] == text2.as_bytes()[j] {
                    dp[i + 1][j + 1] = dp[i][j] + 1
                } else {
                    dp[i + 1][j + 1] = dp[i][j + 1].max(dp[i + 1][j])
                }
            }
        }

        dp[n][m]
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_1() {
        let text1 = String::from("abcde");
        let text2 = String::from("ace");
        assert_eq!(Solution::longest_common_subsequence(text1, text2), 3);
    }

    #[test]
    fn test_2() {
        let text1 = String::from("abc");
        let text2 = String::from("abc");
        assert_eq!(Solution::longest_common_subsequence(text1, text2), 3);
    }

    #[test]
    fn test_3() {
        let text1 = String::from("abc");
        let text2 = String::from("def");
        assert_eq!(Solution::longest_common_subsequence(text1, text2), 0);
    }
}
