pub struct Solution;

impl Solution {
    pub fn number_of_matches(n: i32) -> i32 {
        n - 1
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_1() {
        assert_eq!(Solution::number_of_matches(7), 6);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::number_of_matches(14), 13);
    }
}
